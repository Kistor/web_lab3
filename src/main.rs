mod config;
mod entries;
mod http;
mod postgress;
use log::*;

use std::{
    env,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

use axum::routing::Router;
use config::Config;
use postgress::Postgress;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();

    let Some(config_path) = args.get(1) else {
        help();
        std::process::exit(1);
    };

    let config = load_config(config_path);

    Postgress::new(config.postgres).await;

    let app = Router::new().nest("/", http::router());

    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, config.port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|err| panic!("{err}"));

    Ok(())
}

fn help() {
    info!("Usage: fine [config_path]");
}

fn load_config(config_path: &str) -> config::Config {
    match config::Config::from_file(config_path) {
        Ok(config) => {
            debug!("Loaded config {config:#?}");
            config
        }
        Err(e) => {
            error!("Failed to load config from `{config_path}`. Error: {e}");
            std::process::exit(1);
        }
    }
}
