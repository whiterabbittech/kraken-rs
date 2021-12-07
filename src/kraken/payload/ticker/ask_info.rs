use super::json_helpers::{AskError, unpack_decimal_array};
use bigdecimal::BigDecimal;
use serde_json::{Map, Value};

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
    let parsed_array: [BigDecimal; 3] = unpack_decimal_array(array)?;
    let ask = parsed_array[0].clone();
    let whole_lot_volume = parsed_array[1].clone();
    let lot_volume = parsed_array[2].clone();
    Ok(AskInfo {
        ask,
        whole_lot_volume,
        lot_volume,
    })
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
