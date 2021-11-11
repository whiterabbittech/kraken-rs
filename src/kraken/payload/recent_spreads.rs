use serde::{Deserialize, Serialize};
use serde_json::{Value};

#[derive(Serialize, Deserialize,)]
pub struct RecentSpreadsInput {
    pub pair: String,
    pub since: Option<u64>,
}

#[derive(Serialize, Deserialize,)]
pub struct RecentSpreadsResponse {
    pub error: Vec<String>,
    pub result: RecentSpreadsResult,
}

// TODO: Remove the Value here by calling "Index" and extracting
// the values into a strong type.
#[derive(Serialize, Deserialize,)]
pub struct RecentSpreadsResult {
    pub last: u64,
    // pub pair: Vec<Value>,
}
