use bigdecimal::BigDecimal;

pub struct BidInfo {
    pub bid: BigDecimal,
    pub whole_lot_volume: BigDecimal,
    pub lot_volume: BigDecimal,
}
