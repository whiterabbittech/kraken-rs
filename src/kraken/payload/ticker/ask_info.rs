use bigdecimal::BigDecimal;
use serde_json::Value;
use super::util::{AskError, ArrayWrapper};

pub struct AskInfo {
    pub ask: BigDecimal,
    pub whole_lot_volume: BigDecimal,
    pub lot_volume: BigDecimal,
}

impl TryFrom<&Value> for AskInfo {
    type Error = AskError;

    fn try_from(val: &Value) -> Result<Self, Self::Error> {
        let array: Box<[BigDecimal; 3]> = ArrayWrapper::try_from(val)?.into();
        let ask = array[0].clone();
        let whole_lot_volume = array[1].clone();
        let lot_volume = array[2].clone();
        Ok(AskInfo {
            ask,
            whole_lot_volume,
            lot_volume,
        })
    }
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
