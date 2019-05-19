#![allow(warnings)]
use crate::value::{Symbol, Value};

use maplit::{btreemap, btreeset};

// modeled after json! from serde_json
#[macro_export]
macro_rules! edn {
    // Hide implementation details from the generated rustdoc.
    ($($edn:tt)+) => {
        edn_internal!($($edn)+)
    };
}

#[doc(hidden)]
macro_rules! edn_internal {
    () => {};

    (nil) => {
        $crate::Value::Nil
    };

    (true) => {
        $crate::Value::Bool(true)
    };

    (false) => {
        $crate::Value::Bool(false)
    };

    ( ( $($value:tt)* ) ) => {
        $crate::Value::List(edn_internal!(@seq @vec [] $($value)*))
    };

    ( [ $($value:tt)* ] ) => {
        $crate::Value::Vector(edn_internal!(@seq @vec [] $($value)*))
    };

    ( #{ $($value:tt)* } ) => {
        $crate::Value::Set(edn_internal!(@seq @set [] $($value)*))
    };

    ( { $($value:tt)* } ) => {
        $crate::Value::Map(edn_internal!(@seq @map [] $($value)*))
        //$crate::Value::Map(btreemap!())
    };

    (@seq @vec [$($elems:expr,)*]) => {
        vec![$($elems,)*]
    };

    (@seq @set [$($elems:expr,)*]) => {
        btreeset!{$($elems,)*}
    };

    // this matches an even number of things between square brackets
    (@seq @map [$($key:expr, $val:expr,)*]) => {
        btreemap!{$($key => $val,)*}
    };

    // eat commas with no effect
    (@seq @$kind:ident [$($elems:expr,)*] , $($rest:tt)*) => {
        edn_internal!(@seq @$kind [ $($elems,)* ] $($rest)*)
    };

    // keyword follows
    (@seq @$kind:ident [$($elems:expr,)*] :$head:tt $($rest:tt)*) => {
        edn_internal!(@seq @$kind [ $($elems,)* edn!(:$head) , ] $($rest)*)
    };

    // set
    (@seq @$kind:ident [$($elems:expr,)*] #{$($set_val:tt)*} $($rest:tt)*) => {
        edn_internal!(@seq @$kind [ $($elems,)* edn!(#{$($set_val)*}) , ] $($rest)*)
    };

    // symbol or anything else
    (@seq @$kind:ident [$($elems:expr,)*] $head:tt $($rest:tt)*) => {
        edn_internal!(@seq @$kind [ $($elems,)* edn!($head) , ] $($rest)*)
    };

    (:$head:tt) => {
        $crate::Value::Keyword(stringify!($head).into())
    };

    ($symbol:tt) => {
        edn!(@str_symbol stringify!($symbol))
    };

    ($ns:tt/$symbol:tt) => {
        edn!(@str_symbol concat!(stringify!($ns), "/", stringify!($symbol)));
    };

    (@str_symbol $symbol:expr) => {
        $crate::Value::Symbol(Symbol {
            inner: $symbol.into(),
        })
    };
}

#[test]
fn complex() {
    let s0 = Value::symbol("apply");
    let s1 = Value::symbol("f");
    let k1 = Value::Keyword("k1".into());
    let k2 = Value::Keyword("k2".into());
    let v1 = Value::Keyword("v1".into());
    let v2 = Value::symbol("v2");
    //trace_macros!(true);
    assert_eq!(
        edn!((apply f {:k1 :v1 :k2 v2} #{:k2})),
        Value::List(vec![
            s0,
            s1,
            Value::Map(btreemap! {k1=>v1, k2.clone()=>v2}),
            Value::Set(btreeset! {k2})
        ])
    );
}

#[test]
fn nil() {
    assert_eq!(edn!(nil), Value::Nil);
}

#[test]
fn bool() {
    assert_eq!(edn!(true), Value::Bool(true));
    assert_eq!(edn!(false), Value::Bool(false));
}

#[test]
fn list() {
    assert_eq!(edn!(()), Value::List(vec![]));

    let s = Value::symbol("sym");
    assert_eq!(edn!((sym)), Value::List(vec![s.clone()]));

    let k = Value::Keyword("key".into());
    assert_eq!(edn!((:key)), Value::List(vec![k.clone()]));

    assert_eq!(edn!((:key sym)), Value::List(vec![k.clone(), s.clone()]));
    assert_eq!(edn!((:key, sym)), Value::List(vec![k.clone(), s.clone()]));

    assert_eq!(
        edn!((sym false)),
        Value::List(vec![s.clone(), Value::Bool(false)])
    );
}

#[test]
fn vec() {
    assert_eq!(edn!([]), Value::Vector(vec![]));
}

#[test]
fn map() {
    assert_eq!(edn!({}), Value::Map(btreemap!()));
    assert_eq!(
        edn!({false nil}),
        Value::Map(btreemap!(Value::Bool(false) => Value::Nil))
    );
}

#[test]
fn set() {
    assert_eq!(edn!(#{}), Value::Set(btreeset!()));
}

#[test]
fn keyword() {
    assert_eq!(edn!(:thing), Value::Keyword("thing".into()));
}

#[test]
fn symbol() {
    assert_eq!(edn!(asym), Value::symbol("asym"),);
}

#[test]
fn namespaced_symbol() {
    // FIXME: find way to forbid spaces on either side of /
    assert_eq!(edn!(ns / asym), Value::symbol("ns/asym"),);
}
