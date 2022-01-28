use chrono::{DateTime, NaiveTime, TimeZone, Utc};
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::Serializer;
use serde_json::Value;
use std::fmt;
use std::str::FromStr;

pub(crate) fn datetime_from_vec_timestamp<'de, D>(
    deserializer: D,
) -> Result<Vec<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_seq(DateTimeFromEpochSecondsVisitor)
    //let v: Vec<i64> = Vec::deserialize(deserializer)?;
    //Ok(v.into_iter().map(|x| Utc.timestamp(x, 0)).collect())
}

struct DateTimeFromEpochSecondsVisitor;

impl<'de> Visitor<'de> for DateTimeFromEpochSecondsVisitor {
    type Value = Vec<DateTime<Utc>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer providing the number of seconds in epoch time")
    }

    fn visit_seq<E>(self, mut seq: E) -> Result<Self::Value, E::Error>
    where
        E: de::SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(elem) = seq.next_element()? {
            vec.push(Utc.timestamp(elem, 0));
        }
        Ok(vec)
    }
}

pub(crate) fn hm_from_str<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveTime::parse_from_str(&s, "%H:%M").map_err(de::Error::custom)
}

pub(crate) fn hm_to_string<S>(value: &NaiveTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.collect_str(&value.format("%H:%M").to_string())
}

pub(crate) fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: fmt::Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

pub(crate) fn to_string<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: fmt::Display,
    S: Serializer,
{
    serializer.collect_str(value)
}

pub(crate) fn to_string_optional<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: fmt::Display,
    S: Serializer,
{
    match value {
        Some(v) => serializer.collect_str(v),
        None => serializer.serialize_none(),
    }
}

pub(crate) fn from_str_optional<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: FromStr,
    T::Err: fmt::Display,
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
