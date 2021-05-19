use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Hash, PartialEq, Eq)]
pub enum Endpoints {
    Ticker,
    Assets,
}

type EndpointMap = HashMap<Endpoints, &'static str>;
static ENDPOINTS: Lazy<RwLock<EndpointMap>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(Endpoints::Ticker, "/public/Ticker");
    m.insert(Endpoints::Assets, "/public/Assets");
    RwLock::new(m)
});

const URL_ROOT: &'static str = "https://api.kraken.com/0";

pub fn endpoint(name: Endpoints) -> String {
    let map = ENDPOINTS.read().unwrap();
    let path = map.get(&name).unwrap();
    format!("{}{}", URL_ROOT, path)
}
