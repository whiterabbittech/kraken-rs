use serde_json::Value;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct RawTickerResponse {
    error: Vec<String>,
    result: Option<HashMap<String, Value>>
}
