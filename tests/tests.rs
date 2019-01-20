use serde_edn::from_str;
use serde_edn::{Error, Value};

#[test]
fn nil() {
    let edn = "nil";
    assert_eq!(Ok(Value::Nil), from_str::<Value>(edn));
}

#[test]
fn bool() {
    let edn = "true";
    assert_eq!(Ok(Value::Bool(true)), from_str::<Value>(edn));

    let edn = "false";
    assert_eq!(Ok(Value::Bool(false)), from_str::<Value>(edn));
}

#[test]
fn string_basic() {
    let edn = r#""randomdata""#;
    assert_eq!(
        Ok(Value::String("randomdata".into())),
        from_str::<Value>(edn)
    );
}

#[test]
fn string_eof() {
    let edn = r#""randomda"#;
    assert_eq!(Err(Error::Eof), from_str::<Value>(edn));
}

#[test]
fn string_escapes() {
    let edn = r#""a\tb\rc\nd\\e\"f""#;
    assert_eq!(
        Ok(Value::String("a\tb\rc\nd\\e\"f".into())),
        from_str::<Value>(edn)
    );
}

#[test]
fn char() {
    let edn = r#"\Z"#;
    assert_eq!(Ok(Value::Char('Z')), from_str::<Value>(edn));
}

#[test]
fn char_sequences() {
    let edn = r#"\newline"#;
    assert_eq!(Ok(Value::Char('\n')), from_str::<Value>(edn));

    let edn = r#"\return"#;
    assert_eq!(Ok(Value::Char('\r')), from_str::<Value>(edn));

    let edn = r#"\space"#;
    assert_eq!(Ok(Value::Char(' ')), from_str::<Value>(edn));

    let edn = r#"\tab"#;
    assert_eq!(Ok(Value::Char('\t')), from_str::<Value>(edn));

    // FIXME: part of edn but not implemented by the parser we use
    //let edn = r#"\u1234"#;
    //assert_eq!(Ok(Value::Char('\u{1234}')), from_str::<Value>(edn));
}

#[test]
fn integer_fixed() {
    let expected = Ok(Value::Integer(13));
    assert_eq!(from_str("13"), expected);
    assert_eq!(from_str("+13"), expected);

    let expected = Ok(Value::Integer(-53));
    assert_eq!(from_str("-53"), expected);

    let expected = Ok(Value::Integer(0));
    assert_eq!(from_str("0"), expected);
    assert_eq!(from_str("-0"), expected);
    assert_eq!(from_str("+0"), expected);

    // FIXME: edn specifies integers may not start with 0, but the parser accepts them
    //let expected = Err(Error::Bad);
    //assert_eq!(from_str::<Value>("04"), expected);
}

#[test]
fn float_fixed() {
    let expected = Ok(Value::Float(13.0));
    assert_eq!(from_str("13.0"), expected);
    assert_eq!(from_str("+13.0"), expected);

    let expected = Ok(Value::Float(-53.0));
    assert_eq!(from_str("-53.0"), expected);

    let expected = Ok(Value::Float(0.0));
    assert_eq!(from_str("0.0"), expected);
    assert_eq!(from_str("-0.0"), expected);
    assert_eq!(from_str("+0.0"), expected);
}
