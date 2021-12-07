use super::json_helpers::{LowError, ArrayWrapper};
use bigdecimal::BigDecimal;
use serde_json::{Value};

pub struct LowInfo {
    pub today: BigDecimal,
    pub rolling_24h: BigDecimal,
}

impl TryFrom<&Value> for LowInfo {
    type Error = LowError;

    fn try_from(val: &Value) -> Result<Self, Self::Error> {
        let array: [BigDecimal; 2] = ArrayWrapper::try_from(val)?.into();
        let today = array[0].clone();
        let rolling_24h = array[1].clone();
        Ok(LowInfo {
            today,
            rolling_24h,
        })
    }
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
