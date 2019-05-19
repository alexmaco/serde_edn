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

impl Value {
    pub fn symbol<S: ToString>(s: S) -> Value {
        Value::Symbol(Symbol {
            inner: s.to_string(),
        })
    }
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
    pub fn full(&self) -> &str {
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
