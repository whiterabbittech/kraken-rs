use reqwest::Url;

pub const ACCOUNT_BALANCE: &str = concat!("https://api.kraken.com/0", "/private/Balance");
pub const TRADE_BALANCE: &str = concat!("https://api.kraken.com/0", "/private/TradeBalance");
pub const OPEN_ORDERS: &str = concat!("https://api.kraken.com/0", "/private/OpenOrders");
pub const ASSET_INFO: &str = concat!("https://api.kraken.com/0", "/public/Assets");
pub const SYSTEM_STATUS: &str = concat!("https://api.kraken.com/0", "/public/SystemStatus");
pub const SYSTEM_TIME: &str = concat!("https://api.kraken.com/0", "/public/Time");
pub const TICKER: &str = concat!("https://api.kraken.com/0", "/public/Ticker");
pub const RECENT_SPREADS: &str = concat!("https://api.kraken.com/0", "/public/Spread");
pub const ASSET_PAIRS: &str = concat!("https://api.kraken.com/0", "/public/AssetPairs");

pub fn endpoint(name: &str) -> Url {
    Url::parse(name).unwrap()
}
