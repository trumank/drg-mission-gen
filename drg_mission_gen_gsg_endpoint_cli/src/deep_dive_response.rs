//! This module handles the JSON response format returned by the [GSG deep dive endpoint][endpoint].
//!
//! [endpoint]: https://drg.ghostship.dk/events/deepdive

use std::{fmt, ops};

use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

/// The JSON response that the GSG endpoint returns.
///
/// It is expected to have the shape:
///
/// ```json
/// {"Seed":165956950,"SeedV2":845016340,"ExpirationTime":"2024-07-18T11:00:00Z"}
/// ```
///
/// Note that I have seen negative integers and they seem to be accepted by the game fine (they are
/// likely to be reinterpreted as [`u32`] internally anyways.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct DeepDiveResponse {
    /// This may be a legacy seed used in older game versions. Does not seem to affect newer deep
    /// dives. Consider this unused.
    #[serde(rename = "Seed")]
    _seed: i64,
    /// This is the seed used to generate both deep dive missions, which is the seed we're
    /// interested in.
    pub(crate) seed_v2: i64,
    /// When does the dive expire?
    #[serde(rename = "ExpirationTime")]
    pub(crate) expiration_datetime: ExpirationDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct ExpirationDateTime {
    #[serde(with = "gsg_iso8601_offset_datetime")]
    pub(crate) inner: OffsetDateTime,
}

impl ops::Deref for ExpirationDateTime {
    type Target = OffsetDateTime;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ExpirationDateTime {
    /// When was the dive released?
    pub(crate) fn release_datetime(&self) -> OffsetDateTime {
        self.inner - Duration::days(7)
    }
}

impl fmt::Display for ExpirationDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

/// Custom GSG Deep Dive ISO8601-compliant datetime (de)serialization format with no decimal
/// precision.
mod gsg_iso8601_offset_datetime {
    use serde::{Deserialize, Deserializer, Serializer};
    use time::format_description::well_known::iso8601::{Config, EncodedConfig, TimePrecision};
    use time::format_description::well_known::Iso8601;
    use time::OffsetDateTime;

    /// The default [`time`] ISO8601 has decimal precision in second position when formatting,
    /// which does not match the GSG format.
    const ISO8601_CONFIG: EncodedConfig = Config::DEFAULT
        .set_time_precision(TimePrecision::Second {
            decimal_digits: None,
        })
        .encode();
    const ISO8601_CUSTOM: Iso8601<ISO8601_CONFIG> = Iso8601::<ISO8601_CONFIG>;

    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format(&ISO8601_CUSTOM).unwrap().to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = OffsetDateTime::parse(&s, &ISO8601_CUSTOM).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn test_time_format_roundtrip() {
        let time = r#""2024-07-18T11:00:00Z""#;
        let de: ExpirationDateTime = serde_json::from_str(time).unwrap();
        assert_eq!(de.inner, datetime!(2024-07-18 11:00 UTC));

        let se = serde_json::to_string(&de).unwrap();
        assert_eq!(time, se);
    }

    #[test]
    fn test_deep_dive_response_ser_de() {
        let example =
            r#"{"Seed":165956950,"SeedV2":845016340,"ExpirationTime":"2024-07-18T11:00:00Z"}"#;
        let de: DeepDiveResponse = serde_json::from_str(example).unwrap();
        assert_eq!(de._seed, 165956950);
        assert_eq!(de.seed_v2, 845016340);
        assert_eq!(
            de.expiration_datetime.inner,
            datetime!(2024-07-18 11:00 UTC)
        );
        let se = serde_json::to_string(&de).unwrap();
        assert_eq!(example, se);
    }

    #[test]
    fn test_release_datetime() {
        let exp_datetime = ExpirationDateTime {
            inner: datetime!(2024-07-18 11:00 UTC),
        };
        assert_eq!(
            exp_datetime.release_datetime(),
            datetime!(2024-07-11 11:00 UTC)
        )
    }
}
