use pgn_reader::SanPlus;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

/// A wrapper around SanPlus that implements Serialize and Deserialize.
#[derive(Clone, Debug)]
pub struct SanPlusWrapper(pub SanPlus);

impl Serialize for SanPlusWrapper {
    /// Serializes the SanPlusWrapper into a string.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for SanPlusWrapper {
    /// Deserializes a string into a SanPlusWrapper.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer).expect("Failed to deserialize SanPlusWrapper");
        Ok(SanPlusWrapper(
            pgn_reader::SanPlus::from_str(&s).expect("Failed to parse SanPlus"),
        ))
    }
}
