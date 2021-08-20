use chrono::{DateTime, Utc};
use futures::FutureExt;
use serde::Serialize;
use tracing::instrument;
use warp::reply::Response;
use warp::Rejection;

use crate::api::warp_helpers::ok_pretty_json_error_response;
use crate::app::context::{Context, RefContext};
use crate::db::stats::count_active_database_connections;

#[derive(Serialize)]
struct Stats {
    database: Database,
    #[cfg(feature = "jemalloc")]
    jemalloc: Option<jemalloc::Jemalloc>,
    system: System,
}

#[derive(Serialize)]
struct Database {
    pool_active_connections: u32,
    active_database_connections: i64,
}

#[derive(Serialize)]
struct System {
    start_time: DateTime<Utc>,
    uptime: String,
}

pub async fn system_stats(context: RefContext) -> Result<Response, Rejection> {
    system_stats_impl(context)
        .map(ok_pretty_json_error_response())
        .await
}

#[instrument(skip(context))]
async fn system_stats_impl(context: RefContext) -> crate::Result<Stats> {
    let Context { pool, system, .. } = &*context;
    let stats = Stats {
        database: Database {
            pool_active_connections: pool.size(),
            active_database_connections: count_active_database_connections(pool).await?,
        },
        #[cfg(feature = "jemalloc")]
        jemalloc: tokio::task::spawn_blocking(jemalloc::Jemalloc::read)
            .await
            .expect("jemalloc stats"),
        system: System {
            start_time: system.start_time,
            uptime: human_time(system.start_instant.elapsed().as_secs()),
        },
    };
    Ok(stats)
}

const DAY_SECS: u64 = 24 * 60 * 60;
const HOUR_SECS: u64 = 60 * 60;
const MINUTE_SECS: u64 = 60;

pub fn human_time(secs: u64) -> String {
    let days = secs / 60 / 60 / 24;
    let hours = secs / 60 / 60 % 24;
    let minutes = secs / 60 % 60;
    let seconds = secs % 60;
    let mut result: Vec<String> = Vec::with_capacity(4);
    if secs >= DAY_SECS {
        result.push(format!("{}d", days));
    }
    if secs >= HOUR_SECS {
        result.push(format!("{}h", hours));
    }
    if secs >= MINUTE_SECS {
        result.push(format!("{}m", minutes));
    }
    result.push(format!("{}s", seconds));
    result.join(" ")
}

#[cfg(feature = "jemalloc")]
mod jemalloc {
    use std::convert::TryFrom;

    use byte_unit::AdjustedByte;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Jemalloc {
        options: Options,
        stats: Stats,
    }

    #[derive(Serialize)]
    struct Stats {
        // these two are the most interested
        allocated: AdjustedByte,
        resident: AdjustedByte,
        // other values
        active: AdjustedByte,
        mapped: AdjustedByte,
        metadata: AdjustedByte,
        retained: AdjustedByte,
    }

    #[derive(Serialize)]
    struct Options {
        background_thread: Option<BackgroundThread>,
        number_of_arenas: u32,
    }

    #[derive(Serialize)]
    struct BackgroundThread {
        enabled: bool,
        max: usize,
    }

    struct RawData {
        // stats
        active_bytes: usize,
        allocated_bytes: usize,
        mapped_bytes: usize,
        metadata_bytes: usize,
        resident_bytes: usize,
        retained_bytes: usize,
        // options
        background_thread: Option<BackgroundThread>,
        number_of_arenas: u32,
    }

    impl RawData {
        fn read() -> Option<RawData> {
            // See https://crates.io/crates/tikv-jemalloc-ctl
            use tikv_jemalloc_ctl::{arenas, epoch, stats};
            // Many statistics are cached and only updated
            // when the epoch is advanced:
            epoch::advance().ok()?;
            let value = Self {
                // config
                background_thread: Self::read_background_thread(),
                number_of_arenas: arenas::narenas::read().ok()?,
                // stats
                active_bytes: stats::active::read().ok()?,
                allocated_bytes: stats::allocated::read().ok()?,
                mapped_bytes: stats::mapped::read().ok()?,
                metadata_bytes: stats::metadata::read().ok()?,
                resident_bytes: stats::resident::read().ok()?,
                retained_bytes: stats::retained::read().ok()?,
            };
            Some(value)
        }

        // this function return Option because it doesn't work on MacOS somehow
        fn read_background_thread() -> Option<BackgroundThread> {
            use tikv_jemalloc_ctl::{background_thread, max_background_threads};
            Some(BackgroundThread {
                max: max_background_threads::read().ok()?,
                enabled: background_thread::read().ok()?,
            })
        }
    }

    impl Jemalloc {
        pub fn read() -> Option<Self> {
            use byte_unit::Byte;
            fn byte_from_usize(n: usize) -> Option<AdjustedByte> {
                Some(Byte::from_bytes(u64::try_from(n).ok()?).get_appropriate_unit(true))
            }
            let raw_data = measure_time!("jemalloc: query data", { RawData::read() })?;
            let jemalloc = measure_time!("jemalloc: formatting", {
                let RawData {
                    active_bytes,
                    allocated_bytes,
                    background_thread,
                    mapped_bytes,
                    metadata_bytes,
                    number_of_arenas,
                    resident_bytes,
                    retained_bytes,
                } = raw_data;
                Jemalloc {
                    options: Options {
                        background_thread,
                        number_of_arenas,
                    },
                    stats: Stats {
                        active: byte_from_usize(active_bytes)?,
                        allocated: byte_from_usize(allocated_bytes)?,
                        mapped: byte_from_usize(mapped_bytes)?,
                        metadata: byte_from_usize(metadata_bytes)?,
                        resident: byte_from_usize(resident_bytes)?,
                        retained: byte_from_usize(retained_bytes)?,
                    },
                }
            });
            Some(jemalloc)
        }
    }
}
