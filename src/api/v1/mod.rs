use actix_web::web;
pub mod handlers;
pub mod models;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/watchdog", web::post().to(handlers::watchdog_handler))
            .route("/alert", web::post().to(handlers::alert_handler))
//            .route("/endpoints", web::post().to(handlers::endpoints_handler))
//            .route("/bootnodes", web::post().to(handlers::bootnodes_handler))
    );
}
