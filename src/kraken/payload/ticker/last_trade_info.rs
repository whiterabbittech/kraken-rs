use bigdecimal::BigDecimal;
use super::json_helpers::{LastTradeError, unpack_decimal_array};
use serde_json::{Map, Value};

pub struct LastTradeInfo {
    pub price: BigDecimal,
    pub lot_volume: BigDecimal,
}

impl TryFrom<&Value> for LastTradeInfo {
    type Error = LastTradeError;

    fn try_from(val: &Value) -> Result<Self, Self::Error> {
        // First, remove the map element from its Value wrapper.
        match val.as_object() {
            Some(obj) => try_from_map(obj),
            None => Err(LastTradeError::try_from_error()),
        }
    }
}

fn try_from_map(obj: &Map<String, Value>) -> Result<LastTradeInfo, LastTradeError> {
    // Expected only one key in the map: "a"
    match obj.get("c") {
        Some(array) => try_from_array(array),
        None => Err(LastTradeError::no_key_error()),
    }
}

/// try_from_array is called with the value associated with
/// the object's key "a". The value is expected to be an array of len 3.
fn try_from_array(array: &Value) -> Result<LastTradeInfo, LastTradeError> {
    // The Value is expected to be an array.
    // This array is expected to have exactly three values.
    let parsed_array: [BigDecimal; 2] = unpack_decimal_array(array)?;
    let price = parsed_array[0].clone();
    let lot_volume = parsed_array[1].clone();
    Ok(LastTradeInfo {
        price,
        lot_volume,
    })
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
