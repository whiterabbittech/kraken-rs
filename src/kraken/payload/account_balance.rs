use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,)]
pub struct AccountBalanceInput {
    pub nonce: String,
}
