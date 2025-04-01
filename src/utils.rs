use serde::{Deserialize, Deserializer, Serializer};

pub fn xlsx_formula<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    serializer.serialize_str(&value.to_string())
}

pub fn deserialize_xlsx_formula<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

#[cfg(feature = "image_support")]
pub fn xlsx_image<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsRef<[u8]>,
{
    serializer.serialize_bytes(value.as_ref())
}

#[cfg(feature = "image_support")]
pub fn deserialize_xlsx_image<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: From<Vec<u8>>,
{
    let bytes = Vec::<u8>::deserialize(deserializer)?;
    Ok(bytes.into())
}
