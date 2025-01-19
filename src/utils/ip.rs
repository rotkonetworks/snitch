use actix_web::{HttpRequest};
use std::net::IpAddr;

pub fn extract_ip(req: &HttpRequest) -> Option<IpAddr> {
    // Attempt to extract and parse the IP from the "X-Forwarded-For" header
    let header_ip = req.headers()
        .get("X-Forwarded-For")
        .and_then(|header_value| header_value.to_str().ok())
        .and_then(|ips| ips.split(',').next())
        .and_then(|ip| ip.trim().parse::<IpAddr>().ok());

    // Fallback to the peer address if no valid IP was found in the header
    header_ip.or_else(|| req.peer_addr().map(|socket_addr| socket_addr.ip()))
}
