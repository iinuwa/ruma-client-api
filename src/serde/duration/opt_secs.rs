//! De-/serialization functions for `std::time::Duration` objects represented as seconds.
//! Delegates to `js_int::UInt` to ensure integer size is within bounds.

use std::time::Duration;

use js_int::UInt;
use serde::{
    de::{Deserialize, Deserializer},
    ser::Serializer,
};

/// Serialize an Option<Duration>.
/// Will fail if integer is greater than the maximum integer that can be
/// unambiguously represented by an f64.
#[allow(dead_code)]
pub fn serialize<S>(opt_duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(duration) = opt_duration {
        return super::secs::serialize(duration, serializer);
    }
    serializer.serialize_none()
}

/// Deserializes an Option<Duration>.
/// Will fail if integer is greater than the maximum integer that can be
/// unambiguously represented by an f64.
#[allow(dead_code)]
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let millis = UInt::deserialize(deserializer)?;
    Ok(Some(Duration::from_secs(millis.into())))
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::time::Duration;

    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    struct DurationTest {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default, with = "crate::serde::duration::opt_secs")]
        timeout: Option<Duration>,
    }

    #[test]
    fn test_deserialize_some_duration_as_secs() {
        let json = json!({ "timeout": 60 });

        assert_eq!(
            serde_json::from_value::<DurationTest>(json).unwrap(),
            DurationTest {
                timeout: Some(Duration::from_secs(60))
            },
        );
    }

    #[test]
    fn test_deserialize_empty_duration_as_secs() {
        let json = json!({});

        assert_eq!(
            serde_json::from_value::<DurationTest>(json).unwrap(),
            DurationTest { timeout: None },
        );
    }

    #[test]
    fn test_serialize_some_duration_as_secs() {
        let request = DurationTest {
            timeout: Some(Duration::from_secs(2)),
        };
        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            json!({ "timeout": 2 })
        );
    }

    #[test]
    fn test_serialize_empty_duration_as_secs() {
        let request = DurationTest { timeout: None };
        assert_eq!(serde_json::to_value(&request).unwrap(), json!({}));
    }
}
