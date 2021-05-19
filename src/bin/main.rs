use kraken::{AssetPair, Client};
use reqwest::{Method, Request, Url};
use serde::Deserialize;
use std::env;
use std::time::Duration;
use tower::service_fn;
use tower::Service;
use tower::ServiceExt;

#[derive(Deserialize, Debug)]
struct Ip {
    origin: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("KRAKEN_API_KEY").unwrap();
    let secret_key = env::var("KRAKEN_PRIVATE_KEY").unwrap();
    let client = Client::new(api_key, secret_key);
    let text = client.ticker(AssetPair::DotUsd).await?;
    // let text = client.assets().await?;
    println!("{}", text);
    let system_time = client.system_time().await?;
    println!("System Time: {}", system_time);
    let system_status = client.system_status().await?;
    println!("System Status: {}", system_status);
    Ok(())
}
