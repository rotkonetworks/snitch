// src/models/app_state.rs
use crate::settings::Config;
use crate::db::conn::DbPool;
use crate::db::redis::RedisPool;

pub struct AppState {
    pub config: Config,
    pub db_pool: DbPool,
    pub redis_pool: RedisPool,
}
