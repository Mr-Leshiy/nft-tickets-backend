use rest::run_rest_server;
use std::{net::SocketAddr, str::FromStr};

mod context;
mod rest;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let context = Default::default();

    run_rest_server(context, SocketAddr::from_str("127.0.0.1:8000").unwrap()).await
}
