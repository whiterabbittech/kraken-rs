use bigdecimal::BigDecimal;
use serde_json::{Map, Value};
use std::fmt;
use std::str::FromStr;

pub struct HighInfo {
    pub today: BigDecimal,
    pub rolling_24h: BigDecimal,
}

impl TryFrom<&Value> for HighInfo {
    type Error = HighError;

    fn try_from(val: &Value) -> Result<Self, Self::Error> {
        // first, remove the map element from its Value wrapper.
        match val.as_object() {
            Some(obj) => try_from_map(obj),
            None => Err(HighError::new("Value is not an Object")),
        }
    }
}

fn try_from_map(obj: &Map<String, Value>) -> Result<HighInfo, HighError> {
    // Expected only one key in the map: "h"
    match obj.get("h") {
        Some(array) => try_from_array(array),
        None => Err(HighError::new("Object has no key \"h\"")),
    }
}

/// try_from_array is called with the value associated with
/// the object's key "a". The value is expected to be an array of len 3.
fn try_from_array(array: &Value) -> Result<HighInfo, HighError> {
    // The Value is expected to be an array.
    // This array is expected to have exactly three values.
    let today_val = array.get(0);
    let rolling_24h_val = array.get(1);
    Ok(HighInfo {
        today: unpack_decimal(today_val)?,
        rolling_24h: unpack_decimal(rolling_24h_val)?,
    })
}

fn unpack_decimal(val: Option<&Value>) -> Result<BigDecimal, HighError> {
    if val.is_none() {
        let err = HighError::new("Value is none.");
        return Err(err);
    }
    unpack_unwrapped_decimal(val.unwrap())
}

fn unpack_unwrapped_decimal(val: &Value) -> Result<BigDecimal, HighError> {
    match val {
        Value::String(decimal_str) => unpack_decimal_str(decimal_str),
        _ => Err(HighError::new("Value is not a String.")),
    }
}

fn unpack_decimal_str(val: &str) -> Result<BigDecimal, HighError> {
    let parsed_decimal = BigDecimal::from_str(val);
    let err_transformer =
        |err| HighError::new(format!("Value provided is not a big decimal: {}", err));
    parsed_decimal.map_err(err_transformer)
}

pub struct HighError(String);

impl HighError {
    pub fn new<T: Into<String>>(message: T) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for HighError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing HighInfo: {}", self.0)
    }
}

impl fmt::Debug for HighError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::HighInfo;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use std::convert::TryFrom;

    #[test]
    fn parses_valid_json() {
        let input = json!(
            {
                "h": ["53219.90000", "57200.00000"]
            }
        );
        let high = HighInfo::try_from(&input);
        assert_eq!(high.is_ok(), true);
        let high_info = high.unwrap();
        assert_eq!(high_info.today.to_string(), "53219.90000");
        assert_eq!(high_info.rolling_24h.to_string(), "57200.00000");
    }

    #[test]
    fn parses_invalid_json() {
        let input = json!(
            {
                "h": ["53219.90000", 1000.0]
            }
        );
        let high = HighInfo::try_from(&input);
        assert_eq!(high.is_err(), true);
    }

    #[test]
    fn parses_invalid_json2() {
        let input = json!(
            {
                "x": ["53219.90000", "57200.00000"]
            }
        );
        let high = HighInfo::try_from(&input);
        assert_eq!(high.is_err(), true);
    }
}
