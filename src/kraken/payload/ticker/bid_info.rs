use super::json_helpers::{ArrayWrapper, BidError};
use bigdecimal::BigDecimal;
use serde_json::Value;

pub struct BidInfo {
    pub bid: BigDecimal,
    pub whole_lot_volume: BigDecimal,
    pub lot_volume: BigDecimal,
}

impl TryFrom<&Value> for BidInfo {
    type Error = BidError;

    fn try_from(val: &Value) -> Result<Self, Self::Error> {
        let array: [BigDecimal; 3] = ArrayWrapper::try_from(val)?.into();
        let bid = array[0].clone();
        let whole_lot_volume = array[1].clone();
        let lot_volume = array[2].clone();
        Ok(BidInfo {
            bid,
            whole_lot_volume,
            lot_volume,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::BidInfo;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use std::convert::TryFrom;

    #[test]
    fn parses_valid_json() {
        let input = json!(
            {
                "b": ["52609.60000", "1", "1.000"]
            }
        );
        let bid = BidInfo::try_from(&input);
        assert_eq!(bid.is_ok(), true);
        let bid_info = bid.unwrap();
        assert_eq!(bid_info.bid.to_string(), "52609.60000");
        assert_eq!(bid_info.whole_lot_volume.to_string(), "1");
        assert_eq!(bid_info.lot_volume.to_string(), "1.000");
    }

    #[test]
    fn parses_invalid_json() {
        let input = json!(
            {
                "b": ["52609.60000", true, "1.000"]
            }
        );
        let bid = BidInfo::try_from(&input);
        assert_eq!(bid.is_err(), true);
    }

    #[test]
    fn parses_invalid_json2() {
        let input = json!(
            {
                "x": ["52609.60000", "1", "1.000"]
            }
        );
        let bid = BidInfo::try_from(&input);
        assert_eq!(bid.is_err(), true);
    }
}
