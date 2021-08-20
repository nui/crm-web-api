use chrono::{DateTime, Utc};
use sqlx::postgres::{PgQueryResult, PgRow};
use sqlx::{PgPool, Row};
use tracing::instrument;

pub struct Account {
    pub account_id: i64,
    pub name: String,
    pub password_hash: String,
    pub roles: String,
    pub policies: String,
    pub created: DateTime<Utc>,
    pub allow_login: bool,
}

impl Account {
    pub fn new_account(name: String, password_hash: String) -> Self {
        Account {
            account_id: Default::default(),
            name,
            password_hash,
            policies: "[]".into(),
            roles: "[]".into(),
            created: Utc::now(),
            // We don't allow login by default
            allow_login: false,
        }
    }
}

const ACCOUNT_ID: &str = "account_id";
const NAME: &str = "name";
const PASSWORD_HASH: &str = "password_hash";
const ROLES: &str = "roles";
const POLICIES: &str = "policies";
const CREATED: &str = "created";
const ALLOW_LOGIN: &str = "allow_login";

pub fn row_mapper(row: PgRow) -> sqlx::Result<Account> {
    Ok(Account {
        account_id: row.try_get(ACCOUNT_ID)?,
        name: row.try_get(NAME)?,
        password_hash: row.try_get(PASSWORD_HASH)?,
        roles: row.try_get(ROLES)?,
        policies: row.try_get(POLICIES)?,
        created: row.try_get(CREATED)?,
        allow_login: row.try_get(ALLOW_LOGIN)?,
    })
}

#[instrument(err, skip(pool))]
pub async fn is_username_exist(pool: &PgPool, name: &str) -> sqlx::Result<bool> {
    sqlx::query(include_str!("sql/account__is-username-exist.sql"))
        .bind(name)
        .try_map(|row: PgRow| row.try_get(0).map(|c: i64| c > 0))
        .fetch_one(pool)
        .await
}

#[instrument(err, skip(pool))]
pub async fn get_by_id(pool: &PgPool, id: i64) -> sqlx::Result<Account> {
    sqlx::query(include_str!("sql/account__get-by-id.sql"))
        .bind(id)
        .try_map(row_mapper)
        .fetch_one(pool)
        .await
}

#[instrument(err, skip(pool))]
pub async fn find_allowed_login_by_name(
    pool: &PgPool,
    name: &str,
) -> sqlx::Result<Option<Account>> {
    sqlx::query(include_str!("sql/account__find-allowed-login-by-name.sql"))
        .bind(name)
        .try_map(row_mapper)
        .fetch_optional(pool)
        .await
}

#[instrument(err, skip(pool, new_hash))]
pub async fn change_password(
    pool: &PgPool,
    id: i64,
    new_hash: String,
) -> sqlx::Result<PgQueryResult> {
    sqlx::query(include_str!("sql/account__change-password.sql"))
        .bind(id)
        .bind(new_hash)
        .execute(pool)
        .await
}

#[instrument(err, skip(pool))]
pub async fn list(pool: &PgPool) -> sqlx::Result<Vec<Account>> {
    sqlx::query("select * from account")
        .try_map(row_mapper)
        .fetch_all(pool)
        .await
}

#[instrument(err, skip(pool, data))]
pub async fn create(pool: &PgPool, data: Account) -> sqlx::Result<PgQueryResult> {
    sqlx::query(include_str!("sql/account__create.sql"))
        .bind(data.name)
        .bind(data.password_hash)
        .bind(data.roles)
        .bind(data.policies)
        .bind(data.created)
        .bind(data.allow_login)
        .execute(pool)
        .await
}
