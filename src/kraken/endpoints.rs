use reqwest::Url;

pub const URL_ROOT: &'static str = "https://api.kraken.com/0";
pub const ACCOUNT_BALANCE: &'static str =  concat!("https://api.kraken.com/0", "/private/Balance");
pub const TRADE_BALANCE: &'static str = concat!("https://api.kraken.com/0", "/private/TradeBalance");
pub const ASSETS: &'static str = concat!("https://api.kraken.com/0", "/public/Assets");
pub const SYSTEM_STATUS: &'static str = concat!("https://api.kraken.com/0", "/public/SystemStatus");
pub const SYSTEM_TIME: &'static str = concat!("https://api.kraken.com/0", "/public/Time");
pub const TICKER: &'static str = concat!("https://api.kraken.com/0", "/public/Ticker");

pub fn endpoint(name: &str) -> Url {
    Url::parse(name).unwrap()
}
