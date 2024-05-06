use actix_web::HttpRequest;

pub fn is_authorized(req: &HttpRequest, api_key: &Option<String>) -> bool {
    if let Some(api_key) = api_key {
        req.headers()
            .get("Authorization")
            .and_then(|val| val.to_str().ok())
            .map_or(false, |header| header == format!("Bearer {}", api_key))
    } else {
        false
    }
}
