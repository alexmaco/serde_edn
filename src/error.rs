#![allow(warnings)]
/// An error that can occur during `edn` deserialization
#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    Bad,
    IntegerOutOfBounds,
    Eof,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BULLSHIIIIIT!!!!")
    }
}

use serde::de;
use std::fmt::Display;

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        //Error::Message(msg.to_string())
        Error::Bad
    }
}
