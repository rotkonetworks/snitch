use serde::Deserialize;
use serde::Serialize;

// For JSON body
#[derive(Deserialize)]
pub struct JsonAlertParams {
    pub message: Option<String>,
}

// For query parameters
#[derive(Deserialize)]
pub struct QueryAlertParams {
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

