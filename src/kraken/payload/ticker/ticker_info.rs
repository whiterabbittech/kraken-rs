use super::{AskInfo, BidInfo, LastTradeInfo, NumTradesInfo};
use bigdecimal::BigDecimal;

pub struct TickerInfo {
    pub ask: AskInfo,
    pub bid: BidInfo,
    pub last_trade_closed: LastTradeInfo,
    pub volume: NumTradesInfo,
    pub daily_low: BigDecimal,
    pub rolling_24h_low: BigDecimal,
    pub daily_high: BigDecimal,
    pub rolling_24h_high: BigDecimal,
    pub opening_price: BigDecimal,
    pub daily_volume_weighted_avg_price: BigDecimal,
    pub rolling_24h_volume_weighted_avg_price: BigDecimal,
}
