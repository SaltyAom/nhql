use std::num::NonZeroI32;
use std::fmt;

use serde;
use serde::Serialize;
use serde::de::{Deserialize, Deserializer, Visitor};

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(transparent)]
pub struct MaybeI32OrString(NonZeroI32);

impl<'de> Deserialize<'de> for MaybeI32OrString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = MaybeI32OrString;

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.write_str("integer or string")
            }

            fn visit_u64<E>(self, val: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match NonZeroI32::new(val as i32) {
                    Some(val) => Ok(MaybeI32OrString(val)),
                    None => Err(E::custom("invalid integer value")),
                }
            }

            fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match val.parse::<u64>() {
                    Ok(val) => self.visit_u64(val),
                    Err(_) => Err(E::custom("failed to parse integer")),
                }
            }
        }

        deserializer.deserialize_any(MyVisitor)
    }
}