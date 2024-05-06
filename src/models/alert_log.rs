use std::net::IpAddr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AlertLog {
    pub message: String,
    pub ip_address: IpAddr,
    pub timestamp: i64,
}
