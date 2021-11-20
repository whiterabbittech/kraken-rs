use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct RecentSpreadsInput {
    pub pair: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<u64>,
}

///////////////////////////////////////////
//// User-facing Types for Client /////////
///////////////////////////////////////////

pub struct RecentSpreadsResponse {
    pub error: Vec<String>,
    pub result: Option<Vec<Spread>>,
}

#[derive(Debug)]
pub struct Spread {
    pub pair: String,
    pub time: u64,
    pub bid: String,
    pub ask: String,
}

fn strongly_type_hashmap(hash: HashMap<String, Value>) -> Vec<Spread> {
    hash.iter().map(parse_spread_json).flatten().collect()
}

fn parse_spread_json(args: (&String, &Value)) -> Vec<Spread> {
    let (key, val) = args;
    let pair = key.to_string();
    let arr = val
        .as_array()
        .expect("Value received does not meet API specification.");

    // arr is a list of spreads. Each spread is an array itself.
    arr.iter()
        .map(|elem| {
            let pair = pair.clone();
            let time = elem.get(0).unwrap().as_u64().unwrap();
            let bid = elem.get(1).unwrap().as_str().unwrap().to_string();
            let ask = elem.get(2).unwrap().as_str().unwrap().to_string();
            Spread {
                pair,
                time,
                ask,
                bid,
            }
        })
        .collect()
}

impl From<RawRecentSpreadsResponse> for RecentSpreadsResponse {
    fn from(raw: RawRecentSpreadsResponse) -> Self {
        let error = raw.error;
        // Iterate over the HashMap, converting each
        // key/value into a Spread.
        let result = raw.result.map(strongly_type_hashmap);
        Self { error, result }
    }
}

impl fmt::Display for RecentSpreadsResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = if self.error.is_empty() {
            format!("{:?}", self.error)
        } else {
            format!("{:?}", self.result)
        };
        write!(f, "{}", val)
    }
}

///////////////////////////////////////////
//// Raw Types for accepting JSON /////////
///////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct RawRecentSpreadsResponse {
    pub error: Vec<String>,
    pub result: Option<HashMap<String, Value>>,
}

impl fmt::Display for RawRecentSpreadsResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = if self.error.is_empty() {
            format!("{:?}", self.error)
        } else {
            format!("{:?}", self.result)
        };
        write!(f, "{}", val)
    }
}
