use super::util::{ArrayWrapper, VolumeError};
use bigdecimal::BigDecimal;
use serde_json::Value;

pub struct VolumeInfo {
    pub daily_volume: BigDecimal,
    pub rolling_24h_volume: BigDecimal,
}

impl TryFrom<&Value> for VolumeInfo {
    type Error = VolumeError;

    fn try_from(val: &Value) -> Result<Self, Self::Error> {
        let array: Box<[BigDecimal; 2]> = ArrayWrapper::try_from(val)?.into();
        let daily_volume = array[0].clone();
        let rolling_24h_volume = array[1].clone();
        Ok(VolumeInfo {
            daily_volume,
            rolling_24h_volume,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::VolumeInfo;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use std::convert::TryFrom;

    #[test]
    fn parses_valid_json() {
        let input = json!(
            {
                "v": ["1920.83610601", "7954.00219674"],
            }
        );
        let volume = VolumeInfo::try_from(&input);
        assert_eq!(volume.is_ok(), true);
        let volume_info = volume.unwrap();
        assert_eq!(volume_info.daily_volume.to_string(), "1920.83610601");
        assert_eq!(volume_info.rolling_24h_volume.to_string(), "7954.00219674");
    }

    #[test]
    fn parses_invalid_json() {
        let input = json!(
            {
                "v": ["1920.83610601", 7954.00219674],
            }
        );
        let volume = VolumeInfo::try_from(&input);
        assert_eq!(volume.is_err(), true);
    }

    #[test]
    fn parses_invalid_json2() {
        let input = json!(
            {
                "z": ["1920.83610601", "7954.00219674"],
            }
        );
        let volume = VolumeInfo::try_from(&input);
        assert_eq!(volume.is_err(), true);
    }
}
