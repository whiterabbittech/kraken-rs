use bigdecimal::BigDecimal;
use super::json_helpers::{BidError};
use serde_json::{Map, Value};
use std::fmt;
use std::str::FromStr;

pub struct BidInfo {
    pub bid: BigDecimal,
    pub whole_lot_volume: BigDecimal,
    pub lot_volume: BigDecimal,
}

impl TryFrom<&Value> for BidInfo {
    type Error = BidError;

    fn try_from(val: &Value) -> Result<Self, Self::Error> {
        // First, remove the map element from its Value wrapper.
        match val.as_object() {
            Some(obj) => try_from_map(obj),
            None => Err(BidError::try_from_error()),
        }
    }
}

fn try_from_map(obj: &Map<String, Value>) -> Result<BidInfo, BidError> {
    // Expected only one key in the map: "b"
    match obj.get("b") {
        Some(array) => try_from_array(array),
        None => Err(BidError::no_key_error()),
    }
}

fn try_from_array(array: &Value) -> Result<BidInfo, BidError> {
    // The Value is expected to be an array.
    // This array is expected to have exactly three values.
    let bid_val = array.get(0);
    let whole_volume_val = array.get(1);
    let lot_volume_val = array.get(2);
    Ok(BidInfo {
        bid: unpack_decimal(bid_val)?,
        whole_lot_volume: unpack_decimal(whole_volume_val)?,
        lot_volume: unpack_decimal(lot_volume_val)?,
    })
}

fn unpack_decimal(val: Option<&Value>) -> Result<BigDecimal, BidError> {
    match val {
        Some(v) => unpack_unwrapped_decimal(v),
        None => Err(BidError::none_value_error()),
    }
}

fn unpack_unwrapped_decimal(val: &Value) -> Result<BigDecimal, BidError> {
    match val {
        Value::String(decimal_str) => unpack_decimal_str(decimal_str),
        _ => Err(BidError::not_a_string_error()),
    }
}

fn unpack_decimal_str(val: &str) -> Result<BigDecimal, BidError> {
    let parsed_decimal = BigDecimal::from_str(val);
    parsed_decimal.map_err(|_| BidError::not_a_float_error())
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
