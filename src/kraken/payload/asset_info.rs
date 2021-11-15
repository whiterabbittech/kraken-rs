use serde::{Deserialize, Serialize};
use std::fmt;
use std::collections::HashMap;

#[derive(Serialize, Deserialize,)]
pub struct AssetInfoInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<String>,
}

#[derive(Serialize, Deserialize,)]
pub struct AssetInfoResponse {
    error: Vec<String>,
    result: HashMap<String, AssetInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetInfo {
    #[serde(rename = "aclass")]
    asset_class: String,
    #[serde(rename = "altname")]
    alt_name: String,
    decimals: u64,
    display_decimals: u64,
}

impl fmt::Display for AssetInfoResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = if self.error.len() > 0 {
            format!("{:?}", self.error)
        } else {
            format!("{:?}", self.result)
        };
        write!(f, "{}", val)
    }
}

impl fmt::Display for AssetInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.alt_name)
    }
}
