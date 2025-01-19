// src/db/conn.rs
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{NoTls, Config, Error as PgError};
use std::str::FromStr;

pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;

pub async fn create_postgres_pool(db_uri: &str) -> Result<DbPool, PgError> {
    let config = Config::from_str(db_uri)?;
    let manager = PostgresConnectionManager::new(config, NoTls);
    let pool = Pool::builder().build(manager).await?;
    Ok(pool)
}
