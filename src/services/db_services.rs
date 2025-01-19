// src/services/db_services.rs
use actix_web::web::Data;
use crate::models::app_state::AppState;
use crate::models::alert_log::AlertLog;
use std::error::Error;
use redis::AsyncCommands;

pub async fn save_alert_log_to_postgres(data: &Data<AppState>, alert_log: &AlertLog) -> Result<(), Box<dyn Error>> {
    let conn = data.db_pool.get().await?;
    let stmt = "INSERT INTO alert_logs (message, ip_address, timestamp) VALUES ($1, $2, $3)";
    conn.execute(stmt, &[&alert_log.message, &alert_log.ip_address.to_string(), &alert_log.timestamp]).await?;
    Ok(())
}

pub async fn cache_recent_alert_in_redis(data: &Data<AppState>, alert_log: &AlertLog) -> Result<(), Box<dyn Error>> {
    let mut conn = data.redis_pool.get().await?;
    let _: () = conn.set_ex(
        "recent_alert",
        serde_json::to_string(alert_log)?,
        300  // TTL
    ).await?;
    Ok(())
}
