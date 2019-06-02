#![allow(warnings)]

use serde::de::DeserializeOwned;
use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};

use edn::parser::Parser;
use edn::Value as EValue;

use crate::error::Error;
use crate::Value;

use std::ops::{AddAssign, MulAssign, Neg};

type Result<T> = std::result::Result<T, Error>;

pub struct Deserializer<'a> {
    parser: Parser<'a>,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer {
            parser: Parser::new(input),
        }
    }
}

/// Deserialize an instance of type `T` from a string of edn text
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    T::deserialize(&mut deserializer)
}

impl<'de> Deserializer<'de> {
    fn parse_unsigned<T>(&mut self) -> Result<T>
    where
        T: AddAssign<T> + MulAssign<T> + From<u8>,
    {
        unimplemented!()
    }

    fn parse_signed<T>(&mut self) -> Result<T>
    where
        T: Neg<Output = T> + AddAssign<T> + MulAssign<T> + From<i8>,
    {
        unimplemented!()
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        unimplemented!()
    }

    fn read_parsed(&mut self) -> Result<EValue> {
        match self.parser.read() {
            Some(Ok(v)) => Ok(v),
            None => Err(Error::Eof),
            Some(Err(e)) => Err(if e.message.contains("EOF") {
                Error::Eof
            } else {
                Error::Bad
            }),
        }
    }
}

struct ListAccess(Vec<Value>);

use std::result;
impl<'de> SeqAccess<'de> for ListAccess {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> result::Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.0.is_empty() {
            Ok(None)
        } else {
            Ok(Some(seed.deserialize(self.0.remove(0))?))
        }
    }
}

macro_rules! deserialize_integer {
    ($method:ident, $int:ty, $visit_method:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            let parsed = self.read_parsed()?;

            match parsed {
                EValue::Integer(i) => {
                    let conv = <$int as num_traits::NumCast>::from(i).ok_or(Error::IntegerOutOfBounds)?;
                    visitor.$visit_method(conv)
                }
                _ => Err(Error::Bad),
            }
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let parsed = self.read_parsed()?;

        match parsed {
            EValue::Nil => visitor.visit_unit(),
            EValue::Boolean(b) => visitor.visit_bool(b),
            EValue::String(s) => visitor.visit_str(&s),
            EValue::Char(c) => visitor.visit_char(c),
            EValue::Integer(i) => visitor.visit_i64(i),
            EValue::Float(f) => visitor.visit_f64(f.into_inner()),
            EValue::List(l) => {
                visitor.visit_seq(ListAccess(l.into_iter().map(Value::from).collect()))
            }
            other => panic!("unhandled case {:?}", other),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    deserialize_integer!(deserialize_i8, i8, visit_i8);
    deserialize_integer!(deserialize_i16, i16, visit_i16);
    deserialize_integer!(deserialize_i32, i32, visit_i32);
    deserialize_integer!(deserialize_i64, i64, visit_i64);

    deserialize_integer!(deserialize_u8, u8, visit_u8);
    deserialize_integer!(deserialize_u16, u16, visit_u16);
    deserialize_integer!(deserialize_u32, u32, visit_u32);
    deserialize_integer!(deserialize_u64, u64, visit_u64);

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
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
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
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
    ) -> Result<V::Value>
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
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}
