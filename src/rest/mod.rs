use self::tickets::tickets;
use crate::context::ContextLock;
use actix_web::{web, App, HttpServer};
use std::net::SocketAddr;

mod tickets;

fn config_app(context: ContextLock, cfg: &mut web::ServiceConfig) {
    cfg.service(tickets(context));
}

pub async fn run_rest_server(context: ContextLock, listen: SocketAddr) -> std::io::Result<()> {
    println!("Starting server");

    HttpServer::new(move || App::new().configure(|cfg| config_app(context.clone(), cfg)))
        .bind(listen)?
        .run()
        .await
}
