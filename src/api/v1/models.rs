use serde::Deserialize;
use serde::Serialize;

/// Parameters for alert messages.
#[derive(Deserialize)]
pub struct AlertParams {
    pub message: Option<String>,
}

/// Standard API response for successful operations.
#[derive(Serialize)]
pub struct ApiResponse {
    pub status: String,
    pub message: String,
}

/// Response model for error messages.
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub detail: Option<String>,
}

