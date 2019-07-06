mod de;

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Nil,
    Bool(bool),
    String(String),
    Char(char),
    Symbol(Symbol),
    Keyword(String),
    Integer(i64),
    Float(ordered_float::OrderedFloat<f64>),
    List(Vec<Value>),
    Vector(Vec<Value>),
    Map(BTreeMap<Value, Value>),
    Set(BTreeSet<Value>),
    Tagged(Tagged),
}

macro_rules! is_accessor {
    ($is_method:ident, $as_method:ident) => {
        pub fn $is_method(&self) -> bool {
            self.$as_method().is_some()
        }
    }
}

macro_rules! is_as_accessor {
    ($is_method:ident, $as_method:ident, $as_mut:ident, $variant:ident, $return:ty) => {
        is_accessor!($is_method, $as_method);

        pub fn $as_method(&self) -> Option<&$return> {
            match self {
                Value::$variant(v) => Some(v),
                _ => None,
            }
        }

        pub fn $as_mut(&mut self) -> Option<&mut $return> {
            match self {
                Value::$variant(v) => Some(v),
                _ => None,
            }
        }
    };
}

macro_rules! is_as_accessor_val {
    ($is_method:ident, $as_method:ident, $as_mut:ident, $variant:ident, $return:ty) => {
        is_accessor!($is_method, $as_method);

        pub fn $as_method(&self) -> Option<$return> {
            match self {
                Value::$variant(v) => Some(v.clone().into()),
                _ => None,
            }
        }

        pub fn $as_mut(&mut self) -> Option<&mut $return> {
            match self {
                Value::$variant(v) => Some(v),
                _ => None,
            }
        }
    };
}

impl Value {
    pub fn symbol<S: ToString>(s: S) -> Value {
        Value::Symbol(Symbol {
            inner: s.to_string(),
        })
    }

    pub fn integer<I: num_traits::PrimInt>(i: I) -> Value {
        Value::Integer(<i64 as num_traits::NumCast>::from(i).unwrap())
    }

    pub fn float<F: num_traits::Float>(f: F) -> Value {
        Value::Float(ordered_float::OrderedFloat(
            <f64 as num_traits::NumCast>::from(f).unwrap(),
        ))
    }

    pub fn string<S: ToString>(s: S) -> Value {
        Value::String(s.to_string())
    }

    pub fn is_nil(&self) -> bool {
        self.as_nil().is_some()
    }

    pub fn as_nil(&self) -> Option<()> {
        match self {
            Value::Nil => Some(()),
            _ => None,
        }
    }

    is_as_accessor!(is_symbol, as_symbol, as_symbol_mut, Symbol, Symbol);
    is_as_accessor!(is_keyword, as_keyword, as_keyword_mut, Keyword, str);
    is_as_accessor!(is_list, as_list, as_list_mut, List, Vec<Value>);
    is_as_accessor!(is_vector, as_vector, as_vector_mut, Vector, Vec<Value>);
    is_as_accessor!(is_map, as_map, as_map_mut, Map, BTreeMap<Value, Value>);
    is_as_accessor!(is_set, as_set, as_set_mut, Set, BTreeSet<Value>);
    is_as_accessor!(is_tagged, as_tagged, as_tagged_mut, Tagged, Tagged);

    is_as_accessor_val!(is_integer, as_integer, as_integer_mut, Integer, i64);
    is_as_accessor_val!(is_float, as_float, as_float_mut, Float, f64);
    is_as_accessor_val!(is_char, as_char, as_char_mut, Char, char);
    is_as_accessor_val!(is_bool, as_bool, as_bool_mut, Bool, bool);
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol {
    pub(crate) inner: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tagged {
    Inst(std::time::Instant),
    UUID(u128),
    User(Symbol, Box<Value>),
}

// TODO: these are just test implementations
impl Symbol {
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    pub fn name(&self) -> &str {
        if self.inner.contains('/') {
            self.split('/').nth(1).unwrap()
        } else {
            &self.inner
        }
    }

    pub fn namespace(&self) -> Option<&str> {
        if self.inner.contains('/') {
            self.split('/').nth(0)
        } else {
            None
        }
    }
}

impl Into<String> for Symbol {
    fn into(self) -> String {
        self.inner
    }
}

impl std::ops::Deref for Symbol {
    type Target = str;

    fn deref(&self) -> &str {
        &self.inner
    }
}

use edn::Value as EValue;
impl From<EValue> for Value {
    fn from(e: EValue) -> Self {
        match e {
            EValue::Nil => Value::Nil,
            EValue::Boolean(b) => Value::Bool(b),
            EValue::String(s) => Value::String(s),
            EValue::Char(c) => Value::Char(c),
            EValue::Symbol(s) => Value::symbol(s),
            EValue::Keyword(s) => Value::Keyword(s),
            EValue::Integer(i) => Value::Integer(i),
            EValue::Float(f) => Value::Float(f.into_inner().into()),
            EValue::List(v) => Value::List(v.into_iter().map(Value::from).collect()),
            EValue::Vector(v) => Value::Vector(v.into_iter().map(Value::from).collect()),
            EValue::Map(m) => Value::Map(
                m.into_iter()
                    .map(|(k, v)| (Value::from(k), Value::from(v)))
                    .collect(),
            ),
            EValue::Set(s) => Value::Set(s.into_iter().map(Value::from).collect()),
            EValue::Tagged(s, val) => Value::Tagged(Tagged::User(
                Symbol { inner: s },
                Box::new(Value::from(*val)),
            )),
        }
    }
}
