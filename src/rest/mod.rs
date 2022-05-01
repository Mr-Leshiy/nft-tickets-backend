use self::tickets::tickets;
use actix_web::{web, App, HttpServer};
use std::net::SocketAddr;

mod tickets;

fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(tickets());
}

pub async fn run_rest_server(listen: SocketAddr) -> std::io::Result<()> {
    println!("Starting server");

    HttpServer::new(|| App::new().configure(config_app))
        .bind(listen)?
        .run()
        .await
}
