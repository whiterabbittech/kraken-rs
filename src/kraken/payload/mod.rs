use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize,)]
pub struct AccountBalanceInput {
    pub nonce: String,
}

#[derive(Serialize, Deserialize,)]
pub struct OpenOrdersInput {
    pub nonce: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trades: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ref: Option<u32>,
}

#[derive(Serialize, Deserialize,)]
pub struct TradeBalanceInput {
    pub nonce: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
}

#[derive(Serialize, Deserialize,)]
pub struct ServerTimeResponse {
    pub error: Vec<String>,
    pub result: ServerTimeResult,
}

impl fmt::Display for ServerTimeResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.result.rfc1123)
    }
}

#[derive(Serialize, Deserialize,)]
pub struct ServerTimeResult {
    #[serde(rename = "unixtime")] 
    pub unix_time: u64,
    pub rfc1123: String,
}
