use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TickerInput {
    pub pair: String,
}
