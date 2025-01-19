use std::net::IpAddr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AlertLog {
    pub message: String,
    pub ip_address: IpAddr,
    pub timestamp: i64,
}

impl AlertLog {
    // Method to log the alert details
    pub fn log_details(&self) {
        eprintln!("Received alert with IP: {} and message: {} at {}", self.ip_address, self.message, self.timestamp);
    }
}
