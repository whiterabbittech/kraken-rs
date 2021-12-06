use super::json_helpers::{LowError, unpack_decimal};
use bigdecimal::BigDecimal;
use serde_json::{Map, Value};

pub struct LowInfo {
    pub today: BigDecimal,
    pub rolling_24h: BigDecimal,
}

impl TryFrom<&Value> for LowInfo {
    type Error = LowError;

    fn try_from(val: &Value) -> Result<Self, Self::Error> {
        // first, remove the map element from its Value wrapper.
        match val.as_object() {
            Some(obj) => try_from_map(obj),
            None => Err(LowError::try_from_error()),
        }
    }
}

fn try_from_map(obj: &Map<String, Value>) -> Result<LowInfo, LowError> {
    // Expected only one key in the map: "h"
    match obj.get("l") {
        Some(array) => try_from_array(array),
        None => Err(LowError::no_key_error()),
    }
}

/// try_from_array is called with the value associated with
/// the object's key "a". The value is expected to be an array of len 3.
fn try_from_array(array: &Value) -> Result<LowInfo, LowError> {
    // The Value is expected to be an array.
    // This array is expected to have exactly three values.
    let today_val = array.get(0);
    let rolling_24h_val = array.get(1);
    Ok(LowInfo {
        today: unpack_decimal(today_val)?,
        rolling_24h: unpack_decimal(rolling_24h_val)?,
    })
}

#[cfg(test)]
mod tests {
    use super::LowInfo;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use std::convert::TryFrom;

    #[test]
    fn parses_valid_json() {
        let input = json!(
            {
                "l": ["51513.90000", "51513.90000"]
            }
        );
        let low = LowInfo::try_from(&input);
        assert_eq!(low.is_ok(), true);
        let low_info = low.unwrap();
        assert_eq!(low_info.today.to_string(), "51513.90000");
        assert_eq!(low_info.rolling_24h.to_string(), "51513.90000");
    }

    #[test]
    fn parses_invalid_json() {
        let input = json!(
            {
                "l": ["51513.90000", 51513.90000]
            }
        );
        let low = LowInfo::try_from(&input);
        assert_eq!(low.is_err(), true);
    }

    #[test]
    fn parses_invalid_json2() {
        let input = json!(
            {
                "y": ["51513.90000", "51513.90000"]
            }
        );
        let low = LowInfo::try_from(&input);
        assert_eq!(low.is_err(), true);
    }
}
