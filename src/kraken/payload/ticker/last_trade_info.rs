use super::util::{ArrayWrapper, LastTradeError};
use bigdecimal::BigDecimal;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct LastTradeInfo {
    pub price: BigDecimal,
    pub lot_volume: BigDecimal,
}

impl TryFrom<&Value> for LastTradeInfo {
    type Error = LastTradeError;

    fn try_from(val: &Value) -> Result<Self, Self::Error> {
        let array: Box<[BigDecimal; 2]> = ArrayWrapper::try_from(val)?.into();
        let price = array[0].clone();
        let lot_volume = array[1].clone();
        Ok(LastTradeInfo { price, lot_volume })
    }
}

#[cfg(test)]
mod tests {
    use super::LastTradeInfo;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use std::convert::TryFrom;

    #[test]
    fn parses_valid_json() {
        let input = json!(
            {
                "c": ["52641.10000", "0.00080000"]
            }
        );
        let last_trade = LastTradeInfo::try_from(&input);
        assert_eq!(last_trade.is_ok(), true);
        let last_trade_info = last_trade.unwrap();
        assert_eq!(last_trade_info.price.to_string(), "52641.10000");
        assert_eq!(last_trade_info.lot_volume.to_string(), "0.00080000");
    }

    #[test]
    fn parses_invalid_json() {
        let input = json!(
            {
                "c": ["52641.10000", 10.0]
            }
        );
        let last_trade = LastTradeInfo::try_from(&input);
        assert_eq!(last_trade.is_err(), true);
    }

    #[test]
    fn parses_invalid_json2() {
        let input = json!(
            {
                "z": ["52609.60000", "1", "1.000"]
            }
        );
        let last_trade = LastTradeInfo::try_from(&input);
        assert_eq!(last_trade.is_err(), true);
    }
}
