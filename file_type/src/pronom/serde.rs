use jiff::civil::Date;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

const DATE_FORMAT: &str = "%d %b %Y";

pub fn deserialize_naive_date<'de, D>(deserializer: D) -> Result<Date, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    let date = Date::strptime(DATE_FORMAT, value.as_str()).map_err(Error::custom)?;
    Ok(date)
}

pub fn serialize_naive_date<S>(date: &Date, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&date.strftime(DATE_FORMAT).to_string())
}

pub fn deserialize_option_naive_date<'de, D>(deserializer: D) -> Result<Option<Date>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<&str> = Option::deserialize(deserializer)?;
    if let Some(value) = value {
        if value.is_empty() {
            return Ok(None);
        }

        let date = Date::strptime(DATE_FORMAT, value).map_err(Error::custom)?;
        Ok(Some(date))
    } else {
        Ok(None)
    }
}

#[expect(clippy::ref_option)]
pub fn serialize_option_naive_date<S>(date: &Option<Date>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(date) = date {
        serializer.serialize_str(&date.strftime(DATE_FORMAT).to_string())
    } else {
        serializer.serialize_none()
    }
}
