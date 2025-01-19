use actix_web::{web, HttpResponse, HttpRequest};
use crate::services::{alert_service, watchdog_service, db_services::{save_alert_log_to_postgres, cache_recent_alert_in_redis}};
use crate::models::app_state::AppState;
use crate::models::alert_log::AlertLog;
use crate::utils::auth::is_authorized;
use crate::utils::ip::extract_ip;
use crate::api::v1::models::{JsonAlertParams, QueryAlertParams};
use serde_json::json;
use chrono::offset::Utc;

// Handler to receive the watchdog heartbeat
pub async fn watchdog_handler(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    if !is_authorized(&req, &data.config.api_key_watchdog) {
        return HttpResponse::Unauthorized().body("Unauthorized access");
    }
    // Call the service to handle the heartbeat reception logic
    watchdog_service::receive_heartbeat(&data).await;
    HttpResponse::Ok().json(json!({"message": "Watchdog heartbeat received"}))
}

// Handler to receive and send the alert
pub async fn alert_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    json_params: Option<web::Json<JsonAlertParams>>,
    query_params: web::Query<QueryAlertParams>
) -> HttpResponse {
    let ip_address = match extract_ip(&req) {
        Some(ip) => ip,
        None => return HttpResponse::BadRequest().json("Could not extract IP address"),
    };

    let message = json_params
                    .as_ref()
                    .and_then(|json| json.message.as_deref())
                    .or_else(|| query_params.message.as_deref())
                    .unwrap_or("Alert!");

    let alert_log = AlertLog {
        message: message.to_string(),
        ip_address,
        timestamp: Utc::now().timestamp(),
    };

    alert_log.log_details();

    // Save the log to PostgreSQL
    // if let Err(e) = save_alert_log_to_postgres(&data, &alert_log).await {
    //     eprintln!("Error saving to PostgreSQL: {}", e);
    //     return HttpResponse::InternalServerError().json("Failed to log alert");
    // }
    
    // Cache the log in Redis
    if let Err(e) = cache_recent_alert_in_redis(&data, &alert_log).await {
        eprintln!("Error caching in Redis: {}", e);
    }

    // Send the alert via the external service (e.g., Pushover)
    match alert_service::send_pushover_alert(&data.config, &message).await {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Alert sent"})),
        Err(e) => {
            eprintln!("Error sending alert via Pushover: {}", e);
            HttpResponse::InternalServerError().json(json!({"error": "Failed to send alert", "details": e.to_string()}))
        },
    }
}
