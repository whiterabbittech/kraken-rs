use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct RawTickerResponse {
    pub error: Vec<String>,
    pub result: Option<HashMap<String, Value>>,
}
