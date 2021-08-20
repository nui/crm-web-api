use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::app::config::Database;

// This function is async because SQLx require async runtime to start a pool.
pub async fn create_connection_pool(database: &Database) -> PgPool {
    PgPoolOptions::new()
        .max_connections(database.pool_size)
        .connect_timeout(Duration::from_millis(database.connection_timeout_ms.into()))
        .connect_lazy(&database.url)
        .expect("Unable to create database connection pool")
}
