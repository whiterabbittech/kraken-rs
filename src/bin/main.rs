use kraken_http::{AccountTier, AssetPair, Client, KrakenCredentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let creds = KrakenCredentials::new_from_env().unwrap();
    let client = Client::new(creds, AccountTier::Pro);
    let asset = "DOT".to_owned();
    // let text = client.assets().await?;
    // println!("{}", text);
    let system_time = client.server_time().await?;
    println!("System Time: {}", system_time);
    let system_status = client.system_status().await?;
    println!("System Status: {}", system_status);
    let _assets = client.asset_info(Some(asset.clone()), None).await?;
    // println!("Assets: {}", _assets);
    let _account_balance = client.account_balance().await?;
    // println!("Account Balance: {}", _account_balance);
    let _trade_balance = client.trade_balance(Some(asset)).await?;
    // println!("Trade Balance: {}", _trade_balance);
    let _recent_spreads = client.recent_spreads("XBTUSD".to_owned(), None).await?;
    // println!("Recent Spreads: {}", _recent_spreads);
    let pairs = vec!["XXBTZUSD".to_owned(), "XETHXXBT".to_owned()];
    let _asset_pairs = client.asset_pairs(pairs, None).await?;
    // println!("Asset Pairs: {:?}", asset_pairs);
    let _ticker = client.ticker(AssetPair::DotUsd).await?;
    println!("Ticker: {:?}", _ticker);
    Ok(())
}
