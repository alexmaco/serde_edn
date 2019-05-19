use crate::value::Value;

use serde::de::{Error, Visitor};

use serde::{Deserialize, Deserializer};

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("the world to end")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Nil)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Bool(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::String(v.to_owned()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::String(v))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Char(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Integer(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Float(v.into()))
    }
}
