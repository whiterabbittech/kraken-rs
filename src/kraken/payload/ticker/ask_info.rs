use bigdecimal::BigDecimal;
use serde_json::map::Map;
use serde_json::Value;
use std::fmt;
use std::str::FromStr;

/// TODO: Add a couple unit tests for TryFrom.
/// • Test the happy case.
/// • Test the each of the bad cases too.
/// • Install Pretty Assertions.

pub struct AskInfo {
    pub ask: BigDecimal,
    pub whole_lot_volume: BigDecimal,
    pub lot_volume: BigDecimal,
}

impl TryFrom<Value> for AskInfo {
    type Error = AskError;

    fn try_from(val: Value) -> Result<Self, Self::Error> {
        // First, remove the map element from its Value wrapper.
        match val.as_object() {
            None => return Err(AskError::new("Value is not an Object")),
            Some(obj) => try_from_map(obj),
        }
    }
}

fn try_from_map(obj: &Map<String, Value>) -> Result<AskInfo, AskError> {
    // Expected only one key in the map: "a"
    match obj.get("a") {
        Some(array) => try_from_array(array),
        None => return Err(AskError::new("Object has no key \"a\"")),
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

fn unpack_decimal(val: Option<&Value>) -> Result<BigDecimal, AskError> {
    if val.is_none() {
        let err = AskError::new("Value is none.");
        return Err(err);
    }
    unpack_unwrapped_decimal(val.unwrap())
}

fn unpack_unwrapped_decimal(val: &Value) -> Result<BigDecimal, AskError> {
    match val {
        Value::String(decimal_str) => unpack_decimal_str(decimal_str),
        _ => Err(AskError::new("Value is not a String.")),
    }
}

fn unpack_decimal_str(val: &String) -> Result<BigDecimal, AskError> {
    let parsed_decimal = BigDecimal::from_str(val);
    let err_transformer =
        |err| AskError::new(format!("Value provided is not a big decimal: {}", err));
    parsed_decimal.map_err(err_transformer)
}

pub struct AskError(String);

impl AskError {
    pub fn new<T: Into<String>>(message: T) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for AskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing AskInfo: {}", self.0)
    }
}
