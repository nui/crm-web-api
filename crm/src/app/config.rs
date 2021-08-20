use std::fmt::{Display, Write};
use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};

///! Configuration file parsing
///
/// This module contains Config struct which represent configuration from *.toml file.
use config::{ConfigError, File};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub auth: Auth,
    pub database: Database,
    pub http_server: HttpServer,
    pub jemalloc: Option<Jemalloc>,
    pub request_info: RequestInfo,
    pub runtime: Runtime,
    pub tracing: Tracing,
}

#[derive(Deserialize, Serialize)]
pub struct Database {
    pub connection_timeout_ms: u32,
    pub pool_size: u32,
    pub url: String,
}

#[derive(Deserialize, Serialize)]
pub struct HttpServer {
    pub host: IpAddr,
    pub port: u16,
    pub management_host: Option<IpAddr>,
    pub management_port: Option<u16>,
}

#[derive(Deserialize, Serialize)]
pub struct Auth {
    pub bcrypt_cost: u32,
    pub max_login_secs: u32,
    pub private_key: String,
    pub public_key: String,
    pub token_ttl: u32,
}

#[derive(Deserialize, Serialize)]
pub struct Tracing {
    pub file: Option<PathBuf>,
    pub filters: Option<Vec<String>>,
    pub json: Option<PathBuf>,
    pub rotation: bool,
    pub stdout: bool,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Jemalloc {
    pub background_thread: Option<bool>,
    pub max_background_threads: Option<u32>,
    pub number_of_arenas: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestInfo {
    pub emit: bool,
    pub skip: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Runtime {
    pub call_old_panic_hook: bool,
    pub workers: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            auth: Auth {
                bcrypt_cost: 10,
                max_login_secs: 86400,
                private_key: "DM2RpqPWMqoUm7MNEezPkgX33vvGhn6oZsthbScOmi8".to_owned(),
                public_key: "bONQdW4AoWvhw6mXuK2KxfBs0vWgiVgSmebCETGYMAc".to_owned(),
                token_ttl: 3600,
            },
            database: Database {
                connection_timeout_ms: 3000,
                pool_size: 4,
                url: "postgresql://crm_web_api:crm_web_api@localhost/crm_web_api".to_owned(),
            },
            http_server: HttpServer {
                host: IpAddr::from([0, 0, 0, 0]),
                port: 8000,
                management_host: None,
                management_port: Some(8001),
            },
            jemalloc: None,
            request_info: RequestInfo {
                emit: true,
                skip: ["/", "/favicon.ico"]
                    .iter()
                    .map(|p| p.to_string())
                    .collect(),
            },
            runtime: Runtime {
                call_old_panic_hook: false,
                workers: num_cpus::get(),
            },
            tracing: Tracing {
                file: None,
                filters: None,
                json: None,
                rotation: false,
                stdout: true,
            },
        }
    }
}

const DATABASE_POOL_SIZE: &str = "database.pool_size";

impl Config {
    pub fn try_load(file: Option<&Path>) -> Result<Self, ConfigError> {
        // start with black configuration
        let mut cfg = ::config::Config::new();

        // merge with default config
        let default_config = ::config::Config::try_from(&Config::default())?;
        cfg.merge(default_config)?;

        // merge configuration from config file
        if let Some(c) = file {
            cfg.merge(File::from(c))?;
        }

        // override with env
        if let Ok(size) = std::env::var("DATABASE_POOL_SIZE") {
            cfg.set(DATABASE_POOL_SIZE, size)?;
        }

        let config: Self = cfg.try_into()?;
        let cost = config.auth.bcrypt_cost;
        // 4 is min cost we hard coded because bcrypt doesn't export this value :|
        // 14 is our choice of max cost, it take 700 ms. to hash on Ryzen 3700x cpu
        if !matches!(cost, 4..=14) {
            return Err(ConfigError::Message(format!(
                "Invalid bcrypt cost {}",
                cost
            )));
        }

        Ok(config)
    }

    pub fn to_pretty_toml(&self) -> String {
        toml::to_string_pretty(self).expect("Configuration in valid toml")
    }
}

impl HttpServer {
    pub fn to_server_address(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }

    pub fn to_management_server_address(&self) -> Option<SocketAddr> {
        let port = self.management_port?;
        // default to local host unless specified
        let host = self
            .management_host
            .unwrap_or_else(|| IpAddr::from([127, 0, 0, 1]));
        Some(SocketAddr::new(host, port))
    }
}

impl Jemalloc {
    #[allow(dead_code)]
    pub fn to_config(&self) -> String {
        let mut config = String::with_capacity(64);
        // Abort program if invalid jemalloc configurations are found.
        config.push_str("abort_conf:true");

        let mut write_config = |key: &str, value: &dyn Display| {
            write!(&mut config, ",{}:{}", key, value)
                .expect("a Display implementation returned an error unexpectedly");
        };
        if self.background_thread.is_some() {
            // Do nothing, this is intended.
            // background thread should be enabled at runtime to avoid deadlock.
        }
        if let Some(v) = self.max_background_threads {
            write_config("max_background_threads", &v);
        }
        if let Some(v) = self.number_of_arenas {
            write_config("narenas", &v);
        }
        config
    }
}

// Prevent accidentally exposed secret, we don't need to debug configuration since we already
// provide flag to print it on startup.
macro_rules! impl_masked_debug {
    ($ty:ty) => {
        impl std::fmt::Debug for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($ty))
                    .field(&"* * * value masked by empty_debug macro * * *")
                    .finish()
            }
        }
    };
}

impl_masked_debug!(Auth);
impl_masked_debug!(Config);
impl_masked_debug!(Database);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jemalloc_to_config() {
        let val = Jemalloc {
            background_thread: None,
            max_background_threads: None,
            number_of_arenas: None,
        };
        assert_eq!(val.to_config(), "abort_conf:true");

        let val = Jemalloc {
            background_thread: None,
            max_background_threads: None,
            number_of_arenas: Some(16),
        };
        assert_eq!(val.to_config(), "abort_conf:true,narenas:16");

        let val = Jemalloc {
            background_thread: Some(true),
            max_background_threads: None,
            number_of_arenas: None,
        };
        assert_eq!(val.to_config(), "abort_conf:true");

        let val = Jemalloc {
            background_thread: None,
            max_background_threads: Some(4),
            number_of_arenas: None,
        };
        assert_eq!(val.to_config(), "abort_conf:true,max_background_threads:4");

        let val = Jemalloc {
            background_thread: Some(true),
            max_background_threads: Some(8),
            number_of_arenas: Some(64),
        };
        assert_eq!(
            val.to_config(),
            "abort_conf:true,max_background_threads:8,narenas:64"
        );
    }
}
