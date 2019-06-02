use serde_edn::{edn, from_str, Error, Value};

macro_rules! integer_test {
    ($int:ty, $normal:expr, $overflow:expr) => {
        let normal: $int = $normal;
        assert_eq!(from_str::<$int>(&normal.to_string()), Ok(normal));
        let over = $overflow;
        assert_eq!(
            from_str::<$int>(&over.to_string()),
            Err(Error::IntegerOutOfBounds)
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
