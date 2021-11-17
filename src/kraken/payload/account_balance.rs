use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct AccountBalanceInput {
    pub nonce: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountBalanceResponse {
    error: Vec<String>,
    result: AccountBalanceResult,
}

type AccountBalanceResult = HashMap<String, String>;

impl fmt::Display for AccountBalanceResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = if self.error.is_empty() {
            format!("{:?}", self.error)
        } else {
            format!("{:?}", self.result)
        };
        write!(f, "{}", val)
    }
}
