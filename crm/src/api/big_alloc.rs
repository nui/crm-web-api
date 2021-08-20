use std::time::Duration;

use byte_unit::Byte;
use rand::Rng;
use warp::reply::Response;
use warp::Rejection;

/// Use for test jemalloc
pub async fn big_alloc() -> Result<Response, Rejection> {
    let mut rng = rand::thread_rng();
    let size: usize = rng.gen_range(0..2 * 1024 * 1024);
    let mut blob = Vec::<u8>::with_capacity(size);
    let duration = Duration::from_secs(rng.gen_range(30..60));
    let half_size = size / 2;
    // this my reallocate
    let num_extend = rng.gen_range(half_size..(size + half_size));
    tracing::info!(
        "Allocated {} and wait {} seconds",
        Byte::from(size).get_appropriate_unit(true),
        duration.as_secs()
    );
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        blob.extend((u8::MIN..u8::MAX).cycle().take(num_extend));
        tokio::time::sleep(duration).await;
        drop(blob);
    });
    Ok(Response::default())
}
