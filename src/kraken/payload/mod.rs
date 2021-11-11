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

#[derive(Serialize, Deserialize,)]
pub struct SystemStatusResponse {
    pub result: SystemStatusResult,
    pub error: Vec<String>,
}

impl fmt::Display for SystemStatusResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.error.len() == 0 {
            write!(f, "{}", &self.result)
        } else {
            write!(f, "{:#?}", &self.error)
        }
    }
}

#[derive(Serialize, Deserialize,)]
pub struct SystemStatusResult {
    pub timestamp: String,
    pub status: SystemStatusEnum,
}

impl fmt::Display for SystemStatusResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Time: {}, Status: {}", &self.timestamp, &self.status)
    }
}

#[derive(Serialize, Deserialize,)]
#[serde(rename_all = "snake_case")] 
pub enum SystemStatusEnum {
    Online,
    Maintenance,
    CancelOnly,
    PostOnly,
}

impl fmt::Display for SystemStatusEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            Self::Online => "online",
            Self::Maintenance => "maintenance",
            Self::CancelOnly => "cancel_only",
            Self::PostOnly => "post_only",
        };
        write!(f, "{}", val)
    }
}
