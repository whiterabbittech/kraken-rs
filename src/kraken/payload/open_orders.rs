use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,)]
pub struct OpenOrdersInput {
    pub nonce: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trades: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ref: Option<u32>,
}
