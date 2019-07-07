#![allow(warnings)]

use crate::error::Error;
use crate::value::Value;

use serde::de;
use serde::de::{SeqAccess, Visitor};

use serde::{Deserialize, Deserializer};

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

//type Result<T> = std::result::Result<T, Error>;

macro_rules! deserialize_integer {
    ($method:ident, $visit_method:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value, Error>
        where
            V: Visitor<'de>,
        {
            match self {
                Value::Integer(i) => {
                    let conv = num_traits::NumCast::from(i).ok_or(Error::NumericOutOfBounds)?;
                    visitor.$visit_method(conv)
                }
                _ => Err(Error::Bad),
            }
        }
    }
}

macro_rules! deserialize_float {
    ($method:ident, $visit_method:ident) => {
        fn $method<V>(self, _visitor: V) -> Result<V::Value, Error>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }
    }
}

impl<'de> Deserializer<'de> for Value {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Nil => visitor.visit_unit(),
            Value::Bool(v) => visitor.visit_bool(v),
            Value::String(v) => visitor.visit_string(v),
            //Value::Char(v) => visitor.visit_char(v),
            //Value::Symbol(v) => visitor.visit(v),
            //Value::Keyword(v) => visitor.visit(v),
            Value::Integer(v) => visitor.visit_i64(v),
            Value::Float(v) => visitor.visit_f64(v.into()),
            //Value::List(v) => visitor.visit(v),
            //Value::Vector(v) => visitor.visit(v),
            //Value::Map(v) => visitor.visit(v),
            //Value::Set(v) => visitor.visit(v),
            //Value::Tagged(v) => visitor.visit(v),
            other => panic!("unhandled case {:?}", other),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    deserialize_integer!(deserialize_i8, visit_i8);
    deserialize_integer!(deserialize_i16, visit_i16);
    deserialize_integer!(deserialize_i32, visit_i32);
    deserialize_integer!(deserialize_i64, visit_i64);

    deserialize_integer!(deserialize_u8, visit_u8);
    deserialize_integer!(deserialize_u16, visit_u16);
    deserialize_integer!(deserialize_u32, visit_u32);
    deserialize_integer!(deserialize_u64, visit_u64);

    deserialize_float!(deserialize_f32, visit_f32);
    deserialize_float!(deserialize_f64, visit_f64);

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::String(s) => visitor.visit_string(s),
            _ => Err(Error::Bad),
        }
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Keyword(s) => visitor.visit_str(&s),
            _ => Err(Error::Bad),
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("the world to end")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Nil)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
        Ok(Value::Bool(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
        Ok(Value::String(v.to_owned()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
        Ok(Value::String(v))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E> {
        Ok(Value::Char(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
        Ok(Value::Integer(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
        Ok(Value::Float(v.into()))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut v = vec![];

        while let Some(e) = seq.next_element()? {
            v.push(e);
        }

        Ok(Value::List(v))
    }
}
