use super::{
    AskInfo, BidInfo, HighInfo, LastTradeInfo, LowInfo, NumTradesInfo, VWAInfo, VolumeInfo,
};

#[derive(Debug)]
pub struct TickerInfo {
    pub ticker: String,
    pub ask: AskInfo,
    pub bid: BidInfo,
    pub high: HighInfo,
    pub low: LowInfo,
    pub last_trade_closed: LastTradeInfo,
    pub num_trades: NumTradesInfo,
    pub volume: VolumeInfo,
    pub vol_weighted_avg: VWAInfo,
    // TODO: Revisit capturing this value.
    // pub opening_price: BigDecimal,
}
