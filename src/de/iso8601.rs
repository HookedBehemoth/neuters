use chrono::format::{parse, Item, Numeric, Pad, Parsed};

pub struct DateType(pub chrono::NaiveDate);

const FMT: &[Item] = &[
    Item::Numeric(Numeric::Year, Pad::Zero),
    Item::Literal("-"),
    Item::Numeric(Numeric::Month, Pad::Zero),
    Item::Literal("-"),
    Item::Numeric(Numeric::Day, Pad::Zero),
    Item::Literal("T00:00:00"),
];

struct DateTimeListVisitor;

impl<'de> serde::de::Visitor<'de> for DateTimeListVisitor {
    type Value = DateType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expected list of rfc3339 date times")
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
        let mut parsed = Parsed::new();
        parse(&mut parsed, v, FMT.iter()).unwrap();
        Ok(DateType(parsed.to_naive_date().unwrap()))
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
