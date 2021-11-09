use once_cell::sync::Lazy;
use reqwest::Url;
use std::collections::HashMap;
use std::sync::RwLock;
use url::ParseError;

#[derive(Hash, PartialEq, Eq)]
pub enum Endpoints {
    AccountBalance,
    Assets,
    SystemStatus,
    SystemTime,
    Ticker,
}

type EndpointMap = HashMap<Endpoints, &'static str>;
static ENDPOINTS: Lazy<RwLock<EndpointMap>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(Endpoints::AccountBalance, "/private/Balance");
    m.insert(Endpoints::Assets, "/public/Assets");
    m.insert(Endpoints::SystemStatus, "/public/SystemStatus");
    m.insert(Endpoints::SystemTime, "/public/Time");
    m.insert(Endpoints::Ticker, "/public/Ticker");
    RwLock::new(m)
});

const URL_ROOT: &'static str = "https://api.kraken.com/0";

pub fn endpoint(name: Endpoints) -> Result<Url, ParseError> {
    let map = ENDPOINTS.read().unwrap();
    let path = map.get(&name).unwrap();
    let raw_url = format!("{}{}", URL_ROOT, path);
    Url::parse(&raw_url)
}
