use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TradeBalanceInput {
    pub nonce: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
}
