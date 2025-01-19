// src/db/redis.rs
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::RedisError;

pub type RedisPool = Pool<RedisConnectionManager>;

pub async fn create_redis_pool(redis_uri: &str) -> Result<RedisPool, RedisError> {
    let manager = RedisConnectionManager::new(redis_uri)?;
    let pool = Pool::builder().build(manager).await?;
    Ok(pool)
}
