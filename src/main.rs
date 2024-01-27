use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Responder, rt};
use serde::Deserialize;
use std::{sync::Mutex, time::Instant, fs, env};
use tokio::time::{self, Duration};
use chrono::Local;
use env_logger;

#[derive(Deserialize, Clone)]
struct Config {
    server_port: String,
    pushover_token: String,
    pushover_user: String,
    #[serde(default)]
    pushover_message: Option<String>,
    #[serde(default = "default_priority")]
    pushover_priority: String,
    #[serde(default)]
    pushover_sound: Option<String>,
    #[serde(default = "default_alert_check_interval_secs")]
    alert_check_interval_secs: u64,
    #[serde(default)]
    api_key: Option<String>,
}

// Default functions...
fn default_priority() -> String {
    "0".to_string()
}

fn default_alert_check_interval_secs() -> u64 {
    600
}

// Shared state
struct AppState {
    last_alert_time: Mutex<Instant>,
}

// Handler for Prometheus alerts (both POST and GET)
async fn alert_handler(data: web::Data<AppState>, req: HttpRequest, config: web::Data<Config>) -> impl Responder {
    if config.api_key.as_ref().map_or(true, |key| key.is_empty()) {
        let mut last_alert = data.last_alert_time.lock().unwrap();
        let current_time = Local::now();
        println!("Heartbeat received at {}", current_time.format("%Y-%m-%d %H:%M:%S"));
        *last_alert = Instant::now();
        HttpResponse::Ok().body("Alert received.")
    } else {
        match req.headers().get("x-api-key") {
            Some(header_value) if Some(header_value.to_str().unwrap_or("")) == config.api_key.as_deref() => {
                // Correct API key; proceed with the handler logic
                let mut last_alert = data.last_alert_time.lock().unwrap();
                let current_time = Local::now();
                println!("Heartbeat received at {}", current_time.format("%Y-%m-%d %H:%M:%S"));
                *last_alert = Instant::now();
                HttpResponse::Ok().body("Alert received.")
            },
            _ => HttpResponse::Unauthorized().body("Invalid or missing API key."),
        }
    }
}

// Function to send a notification
async fn send_notification(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut form = vec![
        ("token", &config.pushover_token),
        ("user", &config.pushover_user),
        ("priority", &config.pushover_priority),
    ];

    if let Some(message) = &config.pushover_message {
        form.push(("message", message));
    }

    if let Some(sound) = &config.pushover_sound {
        form.push(("sound", sound));
    }

    let response = client.post("https://api.pushover.net/1/messages.json")
        .form(&form)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Notification sent successfully");
    } else {
        println!("Failed to send notification. Status: {}", response.status());
        if let Ok(text) = response.text().await {
            println!("Response: {}", text);
        }
    }

    Ok(())
}

async fn check_alerts(config: Config, data: web::Data<AppState>) {
    let alert_interval = Duration::from_secs(config.alert_check_interval_secs);
    let warning_threshold = alert_interval / 2; // 50% of the total time
    let mut interval = time::interval(Duration::from_secs(60)); // Check every minute

    println!("Timer set for {} seconds before sending notification.", alert_interval.as_secs());

    loop {
        interval.tick().await;
        let elapsed = data.last_alert_time.lock().unwrap().elapsed();

        // Debugging logs
        println!("Elapsed time: {} seconds", elapsed.as_secs());
        println!("Warning threshold: {} seconds", warning_threshold.as_secs());

        // Log a warning when appropriate
        if elapsed > warning_threshold && elapsed < alert_interval {
            println!("Warning: More than 50% of the alert interval has elapsed without receiving an alert.");
        }

        // Send notification if the interval has passed
        if elapsed >= alert_interval {
            if let Err(e) = send_notification(&config).await {
                println!("Error sending notification: {}", e);
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Load config
    let config_path = env::var("CONFIG_PATH").unwrap_or("config.toml".to_string());
    let config_contents = fs::read_to_string(&config_path)
        .expect("Failed to read the configuration file.");
    let config: Config = toml::from_str(&config_contents)
        .expect("Failed to parse the configuration file.");

    let server_port = config.server_port.clone();
    let app_data = web::Data::new(AppState {
        last_alert_time: Mutex::new(Instant::now()),
    });

    // Clone the config and app_data for the check_alerts task
    let config_for_task = config.clone();
    let app_data_for_task = app_data.clone();

    // Spawn the check_alerts task
    rt::spawn(async move {
        check_alerts(config_for_task, app_data_for_task).await;
    });

    // Start the web server
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone()) // Shared state
            .app_data(web::Data::new(config.clone())) // Configuration
            .route("/alerts", web::post().to(alert_handler))
            .route("/alerts", web::get().to(alert_handler))
    })
    .bind(format!("0.0.0.0:{}", server_port))?
    .workers(1)
    .run()
    .await
}
