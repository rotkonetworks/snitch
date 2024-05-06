use crate::models::app_state::AppState;
use redis::AsyncCommands;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use actix_web::web;

pub async fn monitor_watchdog(data: web::Data<AppState>) {
    let mut interval = tokio::time::interval(Duration::from_secs(60));
    let redis_key = "last_watchdog_beat";

    loop {
        interval.tick().await;
        let mut conn = data.redis_pool.get().await.expect("Failed to get Redis connection");
        let last_beat: Option<u64> = conn.get(redis_key).await.expect("Failed to get last beat from Redis");

        match last_beat {
            Some(timestamp) => {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                if now - timestamp > data.config.alert_check_interval_secs {
                    // Alert logic here
                    println!("Alert! Watchdog not triggered on time!");
                    // Reset in Redis
                    let _: () = conn.set(redis_key, now).await.expect("Failed to reset watchdog beat");
                }
            },
            None => {
                // Initialize if not set
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                let _: () = conn.set(redis_key, now).await.expect("Failed to set initial watchdog beat");
            }
        }
    }
}

pub async fn receive_heartbeat(data: &web::Data<AppState>) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut conn = data.redis_pool.get().await.expect("Failed to get Redis connection");
    let _: () = conn.set("last_watchdog_beat", now).await.expect("Failed to update last watchdog beat");
}
