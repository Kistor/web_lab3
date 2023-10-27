mod config;
mod entries;
mod http;
mod postgress;

use std::{
    env,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

use axum::routing::Router;
use config::Config;
use postgress::Postgress;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_file(&args[0])?;

    Postgress::new(config.postgress).await;

    let app = Router::new().nest("/employee", http::router());

    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, config.port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|err| panic!("{err}"));

    Ok(())
}
