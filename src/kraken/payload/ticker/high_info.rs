use bigdecimal::BigDecimal;
use serde_json::Value;
use super::util::{HighError, ArrayWrapper};

pub struct HighInfo {
    pub today: BigDecimal,
    pub rolling_24h: BigDecimal,
}

impl TryFrom<&Value> for HighInfo {
    type Error = HighError;

    fn try_from(val: &Value) -> Result<Self, Self::Error> {
        let array: Box<[BigDecimal; 2]> = ArrayWrapper::try_from(val)?.into();
        let today = array[0].clone();
        let rolling_24h = array[1].clone();
        Ok(HighInfo { today, rolling_24h })
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
