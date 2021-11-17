use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct AssetPairsInput {
    pub pairs: Vec<String>,
    pub info: Option<AssetPairsInfo>,
}

impl From<AssetPairsInput> for SerializableAssetPairsInput {
    fn from(input: AssetPairsInput) -> Self {
        let info = input.info;
        let pairs = match input.pairs.len() {
            0 => None,
            _ => Some(input.pairs.join(",")),
        };

        Self { info, pairs }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableAssetPairsInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pairs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<AssetPairsInfo>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetPairsInfo {
    Info,
    Leverage,
    Fees,
    Margin,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetPairsResponse {
    pub error: Vec<String>,
    pub result: Option<HashMap<String, AssetPairInfo>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetPairInfo {
    #[serde(rename = "altname")]
    pub alt_name: String,
    #[serde(rename = "wsname")]
    pub websocket_name: String,
    #[serde(rename = "aclass_base")]
    pub base_component_asset_class: String,
    pub base: String,
    #[serde(rename = "aclass_quote")]
    pub quote_component_asset_class: String,
    pub quote: String,
    pub pair_decimals: u64,
    pub lot_decimals: u64,
    pub lot_multiplier: u64,
    pub leverage_buy: Vec<u64>,
    pub leverage_sell: Vec<u64>,
    pub fees: Vec<(u64, f64)>,
    #[serde(rename = "fees_maker")]
    pub maker_fees: Vec<(u64, f64)>,
    pub fee_volume_currency: String,
    pub margin_call: u64,
    pub margin_stop: u64,
    #[serde(rename = "ordermin")]
    pub order_minimum: String,
}
