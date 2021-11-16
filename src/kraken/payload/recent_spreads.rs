use serde::{Deserialize, Serialize};
use serde_json::{Value};
use std::fmt;
use std::collections::HashMap;

#[derive(Serialize, Deserialize,)]
pub struct RecentSpreadsInput {
    pub pair: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<u64>,
}

#[derive(Serialize, Deserialize,)]
pub struct RecentSpreadsResponse {
    pub error: Vec<String>,
    pub result: Option<RecentSpreadsResult>,
}

type RecentSpreadsResult = HashMap<String, Value>;
// type RecentSpreadsResult = HashMap<String, Vec<Vec<Value>>>;

// TODO: Remove the Value here by calling "Index" and extracting
// the values into a strong type.
// #[derive(Serialize, Deserialize,)]
// pub struct RecentSpreadsResult {
//    pub last: u64,
//    pub pair: Vec<Value>,
// }

impl fmt::Display for RecentSpreadsResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = if self.error.len() > 0 {
            format!("{:?}", self.error)
        } else {
            format!("{:?}", self.result)
        };
        write!(f, "{}", val)
    }
}

/*
impl fmt::Display for RecentSpreadsResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.pair)
    }
}
*/
