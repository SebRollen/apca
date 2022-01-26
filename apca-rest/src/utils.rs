use chrono::NaiveTime;
use serde::de::{self, Deserialize, Deserializer};
use serde::ser::Serializer;
use serde_json::Value;
use std::fmt::Display;
use std::str::FromStr;

pub fn hm_from_str<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveTime::parse_from_str(&s, "%H:%M").map_err(de::Error::custom)
}

pub fn hm_to_string<S>(value: &NaiveTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.collect_str(&value.format("%H:%M").to_string())
}

pub fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

pub fn to_string<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Display,
    S: Serializer,
{
    serializer.collect_str(value)
}

pub fn to_string_optional<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Display,
    S: Serializer,
{
    match value {
        Some(v) => serializer.collect_str(v),
        None => serializer.serialize_none(),
    }
}

pub fn from_str_optional<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: serde::Deserializer<'de>,
{
    let deser_res: Result<Value, _> = serde::Deserialize::deserialize(deserializer);
    match deser_res {
        Ok(Value::String(s)) => T::from_str(&s)
            .map_err(serde::de::Error::custom)
            .map(Option::from),
        Ok(_) => Ok(None),
        Err(_) => Ok(None),
    }
}
