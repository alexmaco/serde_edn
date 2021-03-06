use serde_derive::Deserialize;
use serde_edn::{from_str, Error};

use maplit::{hashmap, hashset};

macro_rules! integer_test {
    ($int:ty, $normal:expr, $overflow:expr) => {
        let normal: $int = $normal;
        assert_eq!(from_str::<$int>(&normal.to_string()), Ok(normal));
        let over = $overflow;
        assert_eq!(
            from_str::<$int>(&over.to_string()),
            Err(Error::NumericOutOfBounds)
        );

        assert_eq!(from_str::<$int>(":kw"), Err(Error::Bad));
    };
}

#[test]
fn bool() {
    assert_eq!(from_str::<bool>("true"), Ok(true));
    assert_eq!(from_str::<bool>("false"), Ok(false));
    assert_eq!(from_str::<bool>("33"), Err(Error::Bad));
}

#[test]
fn integers() {
    integer_test!(i8, 3, 128);
    integer_test!(i16, 300, 2_usize.pow(15) + 1);
    integer_test!(i32, 70_000, 2_usize.pow(31) + 1);

    // FIXME disabled led due to parser bug
    //integer_test!(i64, 2_i64.pow(33), 2_usize.pow(63) + 1);

    integer_test!(u8, 3, 256);
    integer_test!(u16, 300, 2_usize.pow(16) + 1);
    integer_test!(u32, 70_000, 2_usize.pow(32) + 1);

    // FIXME disabled led due to parser bug
    //integer_test!(u64, 2_u64.pow(33), 2_usize.pow(63) + 1);
}

#[test]
fn float() {
    assert_eq!(from_str::<f32>("0.3"), Ok(0.3_f32));
    assert_eq!(from_str::<f64>("0.5"), Ok(0.5_f64));
}

#[test]
fn char() {
    assert_eq!(from_str::<char>(r#"\c"#), Ok('c'));
    assert_eq!(from_str::<char>(r#"\tababab"#), Err(Error::Bad));
}

#[test]
fn strings() {
    let data = String::from(r#""astring""#);
    let parsed = from_str::<String>(data.as_str());

    assert_eq!(parsed, Ok("astring".into()));

    // str disabled due to parser limitations
    /*
    let data = String::from(r#""astring""#);
    let middle: &str = data.as_str().trim_matches('"');
    let parsed = from_str::<&str>(data.as_str());

    assert_eq!(parsed, Ok(middle));
    */
}

#[test]
fn option() {
    assert_eq!(from_str::<Option<u32>>("nil"), Ok(None));
    assert_eq!(from_str::<Option<u32>>("3"), Ok(Some(3)));
}

#[test]
fn unit() {
    assert_eq!(from_str::<()>("nil"), Ok(()));
    assert_eq!(from_str::<()>("3"), Err(Error::Bad));
}

#[test]
fn tuple_from_list_or_vec() {
    type Tup = (i32, String);
    let expected = Ok((10, String::from("abcd")));
    assert_eq!(from_str::<Tup>(r#"(10 "abcd")"#), expected);
    assert_eq!(from_str::<Tup>(r#"[10 "abcd"]"#), expected);

    assert_eq!(from_str::<Tup>(r#"[10 "abcd" 3]"#), Err(Error::Bad));
}

#[test]
fn tuple_struct_from_list_or_vec() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct Tup(i32, String);

    let expected = Ok(Tup(10, String::from("abcd")));
    assert_eq!(from_str::<Tup>(r#"(10 "abcd")"#), expected);
    assert_eq!(from_str::<Tup>(r#"[10 "abcd"]"#), expected);

    assert_eq!(from_str::<Tup>(r#"[10 "abcd" 3]"#), Err(Error::Bad));
}

#[test]
fn newtype_struct() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct Tup(i32);

    let expected = Ok(Tup(10));
    assert_eq!(from_str::<Tup>(r#"10"#), expected);
    assert_eq!(from_str::<Tup>(r#"(10)"#), expected);
    assert_eq!(from_str::<Tup>(r#"[10]"#), expected);

    assert_eq!(from_str::<Tup>(r#"[10 "abcd" 3]"#), Err(Error::Bad));
}

#[test]
fn unit_struct() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct YewNit;

    let expected = Ok(YewNit);
    assert_eq!(from_str::<YewNit>(r#"()"#), expected);
    assert_eq!(from_str::<YewNit>(r#"[]"#), expected);
    assert_eq!(from_str::<YewNit>(r#"YewNit"#), expected);

    let err = Err(Error::Bad);
    assert_eq!(from_str::<YewNit>(r#"[10]"#), err);
    assert_eq!(from_str::<YewNit>(r#"(10)"#), err);
}

#[test]
fn map() {
    use std::collections::HashMap;

    type M = HashMap<i32, String>;

    let m: M = hashmap! {
        5 => "abc".into(),
        8 => "def".into(),
    };

    let expected = Ok(m);
    assert_eq!(from_str::<M>(r#"{8 "def" 5 "abc"}"#), expected);
}

#[test]
fn set() {
    use std::collections::HashSet;

    type S = HashSet<i32>;

    let s: S = hashset! { 9, 4, 3 };

    let expected = Ok(s);
    assert_eq!(from_str::<S>(r#"#{3 9 4}"#), expected);
}

#[test]
fn vector() {
    type V = Vec<i32>;
    let v = vec![3, 4, 6, 4, 4];

    let expected = Ok(v);
    assert_eq!(from_str::<V>(r#"[3 4 6 4 4]"#), expected);
    assert_eq!(from_str::<V>(r#"(3 4 6 4 4)"#), expected);

    //let err = Err(Error::Bad);
    //assert_eq!(from_str::<V>(r#"#{3 4 6}"#), err);
}

#[test]
fn struct_from_map() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct S {
        a: u32,
        b: String,
    }

    let expected = Ok(S {
        a: 74,
        b: "abc".into(),
    });
    assert_eq!(from_str::<S>(r#"{:b "abc" :a 74}"#), expected);
}
