use serde::{Deserialize, Serialize};
use std::fmt;

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
