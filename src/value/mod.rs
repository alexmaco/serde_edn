mod de;

use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    String(String),
    Char(char),
    Symbol(Symbol),
    Keyword(String),
    Integer(i64),
    Float(f64),
    List(Vec<Value>),
    Vector(Vec<Value>),
    //Map(HashMap<Value, Value>),
    //Set(HashSet<Value>),
    Tagged(Tagged),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    Simple(String),
    Namespaced { prefix: String, name: String },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tagged {
    Inst(std::time::Instant),
    UUID(u128),
    User {
        prefix: String,
        name: String,
        value: Box<Value>,
    },
}
