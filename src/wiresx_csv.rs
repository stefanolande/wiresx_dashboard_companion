use chrono::NaiveDateTime;
use serde::{de, Deserialize, Deserializer, Serializer};

fn naive_date_time_from_str<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y/%m/%d %H:%M:%S").map_err(de::Error::custom)
}

fn naive_date_time_to_str<S>(ndt: &NaiveDateTime, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&ndt.format("%Y/%m/%d %H:%M:%S").to_string())
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Record {
    callsign: String,
    serial: String,
    name: String,
    #[serde(
        deserialize_with = "naive_date_time_from_str",
        serialize_with = "naive_date_time_to_str"
    )]
    pub(crate) datetime: NaiveDateTime,
    port: String,
    location: String,
}
