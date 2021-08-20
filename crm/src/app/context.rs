use std::convert::Infallible;
use std::ops::Deref;
use std::time::Instant;

use chrono::{DateTime, Utc};
use once_cell::sync::OnceCell;
use sqlx::PgPool;
use warp::Filter;

use tokidator::crypto::{PrivateKey, PublicKey};

use crate::app::config::Config;
use crate::app::panic::PanicSender;
use crate::auth::rbac::ValidationAuthority;
use crate::db::pool::create_connection_pool;

/// Application level context
///
/// This struct contains things that should be available for the whole process lifetime.
pub struct Context {
    pub auth: ValidationAuthority,
    pub config: Config,
    pub panic_sender: PanicSender,
    pub pool: PgPool,
    pub private_key: PrivateKey,
    pub public_key: PublicKey,
    pub system: System,
}

fn _assert_send_sync_static() {
    fn _assert<T: Send + Sync + 'static>() {}
    // guarantee that context is sharable across threads
    _assert::<Context>();
}

/// A new-type of `Context`
///
/// See `Context::into_ref_context` for it usage.
#[derive(Copy, Clone)]
pub struct RefContext(&'static Context);

pub struct System {
    pub start_time: DateTime<Utc>,
    pub start_instant: Instant,
}

impl System {
    pub fn new() -> Self {
        System {
            start_time: Utc::now(),
            start_instant: Instant::now(),
        }
    }
}

impl Context {
    pub async fn create(config: Config, panic_sender: PanicSender) -> Self {
        let pool = create_connection_pool(&config.database).await;
        let system = System::new();
        let private_key = PrivateKey::from_base64(&config.auth.private_key).unwrap();
        let public_key = PublicKey::from_base64(&config.auth.public_key).unwrap();
        let auth = ValidationAuthority::new(public_key.clone());
        Self {
            auth,
            config,
            panic_sender,
            pool,
            private_key,
            public_key,
            system,
        }
    }

    /// Turn `Context` into `RefContext` which is easy to work with because it implement `Copy`
    ///
    /// This method should be called once.
    pub fn into_ref_context(self) -> RefContext {
        static CONTEXT: OnceCell<Context> = OnceCell::new();
        RefContext(CONTEXT.get_or_init(move || self))
    }
}

impl RefContext {
    /// Return a `wrap::Filter` that extract `Self`
    pub fn extract(self) -> impl Filter<Extract = (Self,), Error = Infallible> + Clone {
        warp::any().map(move || self)
    }
}

impl Deref for RefContext {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl AsRef<Context> for RefContext {
    fn as_ref(&self) -> &Context {
        self.0
    }
}
