use serde::{Deserialize, Deserializer};

pub fn parse_title<'de, D>(d: D) -> Result<String, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or("".to_string())
        })
}