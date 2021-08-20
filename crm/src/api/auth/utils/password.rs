use std::time::Duration;

use bcrypt::BcryptResult;
use once_cell::sync::OnceCell;
use thiserror::Error;
use tokio::task::spawn_blocking;
use tracing::instrument;

static JUNK_HASH_DURATION: OnceCell<std::time::Duration> = OnceCell::new();

pub async fn get_junk_hash_duration(cost: u32) -> std::time::Duration {
    match JUNK_HASH_DURATION.get() {
        Some(&duration) => duration,
        None => {
            let calculate_duration = move || {
                let start = std::time::Instant::now();
                bcrypt::hash("", cost).expect("Hash empty string should not fail");
                start.elapsed()
            };
            let duration = spawn_blocking(calculate_duration)
                .await
                .expect("calculate junk hash duration not panic");
            *JUNK_HASH_DURATION.get_or_init(|| duration)
        }
    }
}

const DEFAULT_BCRYPT_TIMEOUT: Duration = Duration::from_secs(10);

#[instrument(err, skip(password, cost))]
pub async fn hash_password<P>(password: P, cost: u32) -> crate::Result<String>
where
    P: AsRef<[u8]> + Send + 'static,
{
    // TODO: This can block forever if cost is too big
    let blocking = spawn_blocking(move || bcrypt::hash(password, cost));
    let hash = tokio::time::timeout(DEFAULT_BCRYPT_TIMEOUT, blocking)
        .await
        .map_err(|_| PasswordTimeoutError)?
        .unwrap()?;
    Ok(hash)
}

#[instrument(err, skip(password, hash))]
pub async fn verify_password<P, H>(password: P, hash: H) -> BcryptResult<bool>
where
    P: AsRef<[u8]> + Send + 'static,
    H: AsRef<str> + Send + 'static,
{
    spawn_blocking(move || bcrypt::verify(password, hash.as_ref()))
        .await
        .unwrap()
}

#[derive(Debug, Error)]
#[error("password timeout error")]
struct PasswordTimeoutError;
