use env_logger::Env;
use std::fs::File;
use std::io::{self, Read};
use warp::Filter;

use lndtip::lnd_filters;
use lndtip::lnd_service;

use r2d2_lndclient::lnd_pool::{LightningConnectionInfo, LightningConnectionManager};
use r2d2_lndclient::r2d2;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LndtipConfig {
    cert: String,
    host: String,
    macaroon: String,
}

#[tokio::main]
async fn main() {
    let log_env = Env::default().filter_or("RUST_LOG", "debug");
    env_logger::init_from_env(log_env);

    let config = File::open("lndtip.toml")
        .and_then(|mut file| {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;
            Ok(buffer)
        })
        .and_then(|buffer| {
            toml::from_str::<LndtipConfig>(&buffer)
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
        })
        .expect("Error reading config: lndtip.toml");

    let manager = LightningConnectionManager::new(LightningConnectionInfo {
        host: config.host,
        cert: config.cert,
        macaroon: config.macaroon,
    })
    .unwrap();

    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let lnds = lnd_service::LightningService::new(pool);
    let api = lnd_filters::invoices(lnds);
    let routes = api
        .with(warp::log("invoices"))
        .or(warp::path("frontend").and(warp::fs::dir("frontend")));
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
