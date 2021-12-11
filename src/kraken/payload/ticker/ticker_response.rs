use super::util::{ErrorWrapper, ParseError};
use super::{
    AskInfo, BidInfo, HighInfo, LastTradeInfo, LowInfo, NumTradesInfo, RawTickerResponse,
    TickerInfo, VWAInfo, VolumeInfo,
};
use std::convert::TryFrom;
use std::error::Error;

pub struct TickerResponse {
    pub error: Vec<String>,
    pub result: Option<Vec<TickerInfo>>,
}

impl TryFrom<RawTickerResponse> for TickerResponse {
    type Error = Box<dyn Error>;
    fn try_from(raw: RawTickerResponse) -> Result<Self, Self::Error> {
        // First, let's copy over any errors.
        let error = raw.error;
        // Next, let's parse the body.
        if raw.result.is_none() {
            return Ok(Self {
                error,
                result: None,
            });
        }
        let mut result = Vec::new();
        let hash = raw.result.unwrap();
        // Extract each of the Info types.
        for (k, v) in hash {
            let ticker = k;
            let ask = AskInfo::try_from(&v)?;
            let bid = BidInfo::try_from(&v)?;
            let low = LowInfo::try_from(&v)?;
            let high = HighInfo::try_from(&v)?;
            let volume = VolumeInfo::try_from(&v)?;
            let vol_weighted_avg = VWAInfo::try_from(&v)?;
            let num_trades = NumTradesInfo::try_from(&v)?;
            let last_trade_closed = LastTradeInfo::try_from(&v)?;

            let info = TickerInfo {
                ticker,
                ask,
                bid,
                high,
                low,
                last_trade_closed,
                num_trades,
                volume,
                vol_weighted_avg,
            };
            result.push(info)
        }
        Ok(TickerResponse {
            error,
            result: Some(result),
        })
    }
}
