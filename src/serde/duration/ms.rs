//! De-/serialization functions for `std::time::Duration` objects represented as milliseconds.
//! Delegates to `js_int::UInt` to ensure integer size is within bounds.

use std::{convert::TryFrom, time::Duration};

use js_int::UInt;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Error, Serialize, Serializer},
};

/// Serializes a Duration struct.
/// Will fail if integer is greater than the maximum integer that can be
/// unambiguously represented by an f64.
pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match UInt::try_from(duration.as_millis()) {
        Ok(uint) => uint.serialize(serializer),
        Err(err) => Err(S::Error::custom(err)),
    }
}

/// Deserializes a Duration struct.
/// Will fail if integer is greater than the maximum integer that can be
/// unambiguously represented by an f64.
#[allow(dead_code)]
pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let millis = UInt::deserialize(deserializer)?;
    Ok(Duration::from_millis(millis.into()))
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::time::Duration;

    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    struct DurationTest {
        #[serde(with = "crate::serde::duration::ms")]
        timeout: Duration,
    }

    #[test]
    fn test_deserialize_duration_as_milliseconds() {
        let json = json!({ "timeout": 3000 });

        assert_eq!(
            serde_json::from_value::<DurationTest>(json).unwrap(),
            DurationTest {
                timeout: Duration::from_millis(3000)
            },
        );
    }

    #[test]
    fn test_serialize_duration_as_milliseconds() {
        let test = DurationTest {
            timeout: Duration::from_secs(8),
        };
        assert_eq!(
            serde_json::to_value(test).unwrap(),
            json!({ "timeout": 8000 }),
        );
    }
}
