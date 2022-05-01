use rest::run_rest_server;
use std::{net::SocketAddr, str::FromStr};

mod rest;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run_rest_server(SocketAddr::from_str("127.0.0.1:8000").unwrap()).await
}
