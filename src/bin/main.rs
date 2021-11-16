use kraken::{AssetPair, Client};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct Ip {
    origin: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("KRAKEN_API_KEY").unwrap();
    let secret_key = env::var("KRAKEN_PRIVATE_KEY").unwrap();
    let client = Client::new(api_key, secret_key);
    let _text = client.ticker(AssetPair::DotUsd).await?;
    let asset = "DOT".to_owned();
    // let text = client.assets().await?;
    // println!("{}", text);
    let system_time = client.server_time().await?;
    println!("System Time: {}", system_time);
    let system_status = client.system_status().await?;
    println!("System Status: {}", system_status);
    // let assets = client.asset_info(Some(asset.clone()), None).await?;
    // println!("Assets: {}", assets);
    let account_balance = client.account_balance().await?;
    println!("Account Balance: {}", account_balance);
    let trade_balance = client.trade_balance(Some(asset)).await?;
    println!("Trade Balance: {}", trade_balance);
    // let recent_spreads = client.recent_spreads("XBTUSD".to_owned(), None).await?;
    // println!("Recent Spreads: {}", recent_spreads);
    Ok(())
}
