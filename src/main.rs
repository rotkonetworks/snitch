use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest}; 
use serde::Deserialize;
use std::{sync::Mutex, time::Instant, env, fs};
use tokio::time::{self, Duration};
use chrono::Local;

#[derive(Deserialize, Clone)]
struct Config {
    server_port: String,
    pushover_token: String,
    pushover_user: String,
    #[serde(default)]
    pushover_message: Option<String>,
    #[serde(default)]
    pushover_priority: Option<String>,
    #[serde(default)]
    pushover_sound: Option<String>,
    #[serde(default = "default_alert_check_interval_secs")]
    alert_check_interval_secs: u64,
    #[serde(default)]
    api_key_watchdog: Option<String>,
    #[serde(default)]
    api_key_alert: Option<String>,
}

fn default_alert_check_interval_secs() -> u64 { 600 }

struct AppState {
    last_alert_time: Mutex<Instant>,
}

#[derive(Deserialize)]
struct AlertParams {
    message: Option<String>,
}

fn is_authorized(req: &HttpRequest, api_key: &Option<String>) -> bool {
    api_key.as_ref()
          .map_or(true, |key| req.headers().get("Authorization")
                  .and_then(|val| val.to_str().ok())
                  .map_or(false, |token| token == format!("Bearer {}", key)))
}

async fn alert_handler(
    req: HttpRequest,
    config: web::Data<Config>,
    params: web::Query<AlertParams>,
) -> HttpResponse {
    if !is_authorized(&req, &config.api_key_alert) {
        return HttpResponse::Unauthorized().body("0xfailure");
    }
    let message = params.message.as_deref().or_else(|| config.pushover_message.as_deref()).unwrap_or("Default message");
    match send_notification(&config, message).await {
        Ok(_) => HttpResponse::Ok().body("msg delivered succesfully"),
        Err(_) => HttpResponse::InternalServerError().body("err")
    }
}

async fn watchdog_handler(data: web::Data<AppState>, req: HttpRequest, config: web::Data<Config>) -> HttpResponse {
    if is_authorized(&req, &config.api_key_watchdog) {
        let mut last_alert = data.last_alert_time.lock().unwrap();
        let current_time = Local::now();
        *last_alert = Instant::now();
        println!("Watchdog heartbeat received at {}", current_time.format("%Y-%m-%d %H:%M:%S"));
        HttpResponse::Ok().body("Heartbeat received!")
    } else {
        println!("Authorization failed");
        HttpResponse::Unauthorized().body("Heartbeat failed: Unauthorized")
    }
}

async fn send_notification(config: &Config, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let message_string = message.to_string(); // Convert message to String

    let mut form_data = vec![
        ("token", config.pushover_token.clone()),
        ("user", config.pushover_user.clone()),
        ("message", message_string),
    ];

    if let Some(ref sound) = config.pushover_sound {
        form_data.push(("sound", sound.clone()));
    }

    if let Some(ref priority) = config.pushover_priority {
        form_data.push(("priority", priority.clone()));
    }

    let response = client.post("https://api.pushover.net/1/messages.json")
                         .form(&form_data)
                         .send()
                         .await?;

    response.error_for_status_ref()?;
    Ok(())
}

async fn check_alerts(config: Config, data: web::Data<AppState>) {
    let alert_interval = Duration::from_secs(config.alert_check_interval_secs);
    let mut interval = time::interval(Duration::from_secs(60));
    let message = config.pushover_message.as_deref().unwrap_or("Watchdog not firing!");

    loop {
        interval.tick().await;
        let elapsed = data.last_alert_time.lock().unwrap().elapsed();

        // Send notification if the full interval has passed
        if elapsed >= alert_interval {
            if let Err(e) = send_notification(&config, &message).await {
                eprintln!("Error sending notification: {}", e);
            }

            // Reset the last alert time
            *data.last_alert_time.lock().unwrap() = Instant::now();
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
    let config_contents = fs::read_to_string(&config_path)
        .expect("Failed to read the configuration file.");
    let config: Config = toml::from_str(&config_contents)
        .expect("Failed to parse the configuration file.");

    let server_port = config.server_port.clone(); // Clone the server port here

    let app_data = web::Data::new(AppState {
        last_alert_time: Mutex::new(Instant::now()),
    });

    // Spawn the check_alerts task
    let config_for_alerts = config.clone();
    let app_data_for_alerts = app_data.clone();
    tokio::spawn(async move {
        check_alerts(config_for_alerts, app_data_for_alerts).await;
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone()) // Shared state
            .app_data(web::Data::new(config.clone())) // Clone the config inside the closure
            .route("/watchdog", web::post().to(watchdog_handler))
            .route("/alert", web::post().to(alert_handler))
    })
    .bind(format!("0.0.0.0:{}", server_port))?
    .workers(1)
    .run()
    .await
}

