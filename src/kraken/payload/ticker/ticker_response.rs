use super::TickerInfo;

pub struct TickerResponse {
    pub error: Vec<String>,
    pub result: Option<Vec<TickerInfo>>,
}
