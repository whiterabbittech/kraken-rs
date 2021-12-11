use super::util::{ArrayWrapper, NumTradesError};
use serde_json::Value;

pub struct NumTradesInfo {
    //pub daily_volume: u64,
    //pub rolling_24h_volume: u64,
    pub daily_num_trades: u64,
    pub rolling_24h_num_trades: u64,
}

impl TryFrom<&Value> for NumTradesInfo {
    type Error = NumTradesError;

    fn try_from(val: &Value) -> Result<Self, Self::Error> {
        let array: Box<[u64; 2]> = ArrayWrapper::try_from(val)?.into();
        //let daily_volume = array[0];
        //let rolling_24h_volume = array[1];
        let daily_num_trades = array[0];
        let rolling_24h_num_trades = array[1];
        Ok(NumTradesInfo {
            //daily_volume,
            //rolling_24h_volume,
            daily_num_trades,
            rolling_24h_num_trades,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::NumTradesInfo;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use std::convert::TryFrom;

    #[test]
    fn parses_valid_json() {
        let input = json!(
            {
                "t": [23329, 80463]
            }
        );
        let num_trades = NumTradesInfo::try_from(&input);
        assert_eq!(num_trades.is_ok(), true);
        let num_trades_info = num_trades.unwrap();
        assert_eq!(num_trades_info.daily_num_trades, 23329);
        assert_eq!(num_trades_info.rolling_24h_num_trades, 80463);
    }

    #[test]
    fn parses_invalid_json() {
        let input = json!(
            {
                "t": [23329, "80463"]
            }
        );
        let num_trades = NumTradesInfo::try_from(&input);
        assert_eq!(num_trades.is_err(), true);
    }

    #[test]
    fn parses_invalid_json2() {
        let input = json!(
            {
                "x": ["53219.90000", "57200.00000"]
            }
        );
        let num_trades = NumTradesInfo::try_from(&input);
        assert_eq!(num_trades.is_err(), true);
    }
}
