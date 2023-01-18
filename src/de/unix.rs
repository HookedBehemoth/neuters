pub struct DateType(pub chrono::NaiveDate);

struct DateTimeListVisitor;

impl<'de> serde::de::Visitor<'de> for DateTimeListVisitor {
    type Value = DateType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expected unix timestamps")
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&v)
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_i64(
            v.parse::<i64>()
                .map_err(|_| E::invalid_value(serde::de::Unexpected::Str(v), &"Timestamp"))?,
        )
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_i64(v as i64)
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_i64(v as i64)
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match chrono::NaiveDateTime::from_timestamp_opt(v, 0) {
            Some(datetime) => Ok(DateType(datetime.date())),
            None => Err(serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &"Timestamp")),
        }
    }
}

impl<'de> serde::Deserialize<'de> for DateType {
    fn deserialize<D>(deserializer: D) -> Result<DateType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(DateTimeListVisitor)
    }
}
