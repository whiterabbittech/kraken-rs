use super::json_helpers::{AskError};
use bigdecimal::BigDecimal;
use serde_json::{Map, Value};
use std::str::FromStr;

pub struct AskInfo {
    pub ask: BigDecimal,
    pub whole_lot_volume: BigDecimal,
    pub lot_volume: BigDecimal,
}

impl TryFrom<&Value> for AskInfo {
    type Error = AskError;

    fn try_from(val: &Value) -> Result<Self, Self::Error> {
        // First, remove the map element from its Value wrapper.
        match val.as_object() {
            Some(obj) => try_from_map(obj),
            None => Err(AskError::try_from_error()),
        }
    }
}

fn try_from_map(obj: &Map<String, Value>) -> Result<AskInfo, AskError> {
    // Expected only one key in the map: "a"
    match obj.get("a") {
        Some(array) => try_from_array(array),
        None => Err(AskError::no_key_error()),
    }
}

/// try_from_array is called with the value associated with
/// the object's key "a". The value is expected to be an array of len 3.
fn try_from_array(array: &Value) -> Result<AskInfo, AskError> {
    // The Value is expected to be an array.
    // This array is expected to have exactly three values.
    let ask_val = array.get(0);
    let whole_volume_val = array.get(1);
    let lot_volume_val = array.get(2);
    Ok(AskInfo {
        ask: unpack_decimal(ask_val)?,
        whole_lot_volume: unpack_decimal(whole_volume_val)?,
        lot_volume: unpack_decimal(lot_volume_val)?,
    })
}

/// TODO: This was can probably extract into a helper.
fn unpack_decimal(val: Option<&Value>) -> Result<BigDecimal, AskError> {
    match val {
        Some(v) => unpack_unwrapped_decimal(v),
        None => Err(AskError::none_value_error()),
    }
}

/// TODO: This was can probably extract into a helper.
fn unpack_unwrapped_decimal(val: &Value) -> Result<BigDecimal, AskError> {
    match val {
        Value::String(decimal_str) => unpack_decimal_str(decimal_str),
        _ => Err(AskError::not_a_string_error()),
    }
}

/// TODO: This was can probably extract into a helper.
fn unpack_decimal_str(val: &str) -> Result<BigDecimal, AskError> {
    let parsed_decimal = BigDecimal::from_str(val);
    parsed_decimal.map_err(|_| AskError::not_a_float_error())
}

#[cfg(test)]
mod tests {
    use super::AskInfo;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use std::convert::TryFrom;

    #[test]
    fn parses_valid_json() {
        let input = json!(
            {
                "a": ["52609.60000", "1", "1.000"]
            }
        );
        let ask = AskInfo::try_from(&input);
        assert_eq!(ask.is_ok(), true);
        let ask_info = ask.unwrap();
        assert_eq!(ask_info.ask.to_string(), "52609.60000");
        assert_eq!(ask_info.whole_lot_volume.to_string(), "1");
        assert_eq!(ask_info.lot_volume.to_string(), "1.000");
    }

    #[test]
    fn parses_invalid_json() {
        let input = json!(
            {
                "a": ["52609.60000", true, "1.000"]
            }
        );
        let ask = AskInfo::try_from(&input);
        assert_eq!(ask.is_err(), true);
    }

    #[test]
    fn parses_invalid_json2() {
        let input = json!(
            {
                "b": ["52609.60000", "1", "1.000"]
            }
        );
        let ask = AskInfo::try_from(&input);
        assert_eq!(ask.is_err(), true);
    }
}
