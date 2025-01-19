// src/services/alert_service.rs
use crate::settings::Config;
use reqwest::Client;
use std::error::Error;

pub async fn send_pushover_alert(config: &Config, message: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let form_data = [
        ("token", &config.pushover_token),
        ("user", &config.pushover_user),
        ("message", &message.to_string()),
        // add sounds / priority / etc here
    ];

    let response = client.post("https://api.pushover.net/1/messages.json")
                         .form(&form_data)
                         .send()
                         .await?;

    response.error_for_status()?;
    Ok(())
}
