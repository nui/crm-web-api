use sqlx::postgres::PgRow;
use sqlx::prelude::*;
use sqlx::PgPool;

pub async fn count_active_database_connections(pool: &PgPool) -> sqlx::Result<i64> {
    sqlx::query(include_str!("sql/active_database_connection.sql"))
        .try_map(|row: PgRow| row.try_get(0))
        .fetch_one(pool)
        .await
}
