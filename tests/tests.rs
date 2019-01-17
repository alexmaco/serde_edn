use serde_edn::from_str;
use serde_edn::{Error, Value};

#[test]
fn nil() {
    let edn = "nil";
    assert_eq!(Ok(Value::Nil), from_str::<Value>(edn));
}

#[test]
fn nil_eof() {
    let edn = "ni";
    assert_eq!(Err(Error::Eof), from_str::<Value>(edn));
}

#[test]
fn bool() {
    let edn = "true";
    assert_eq!(Ok(Value::Bool(true)), from_str::<Value>(edn));

    let edn = "false";
    assert_eq!(Ok(Value::Bool(false)), from_str::<Value>(edn));
}

#[test]
fn bool_eof() {
    let edn = "tru";
    assert_eq!(Err(Error::Eof), from_str::<Value>(edn));

    let edn = "fals";
    assert_eq!(Err(Error::Eof), from_str::<Value>(edn));
}

#[test]
fn string_basic() {
    let edn = r#""randomdata""#;
    assert_eq!(Ok(Value::String("randomdata".into())), from_str::<Value>(edn));
}

#[test]
fn string_eof() {
    let edn = r#""randomda"#;
    assert_eq!(Err(Error::Eof), from_str::<Value>(edn));
}

#[test]
fn string_escapes() {
    let edn = r#""a\tb\rc\nd\\e\"f""#;
    assert_eq!(Ok(Value::String("a\tb\rc\nd\\e\"f".into())), from_str::<Value>(edn));
}
