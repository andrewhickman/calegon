use std::error::Error;
use std::str::FromStr;
use std::string::ToString;

use proptest::prelude::*;

use arbitrary::*;

fn test_roundtrip<T>(value: T) -> Result<(), TestCaseError>
where
    T: FromStr + ToString,
    T::Err: Error,
{
    let lhs = value.to_string();
    let value = T::from_str(&lhs).unwrap();
    let rhs = value.to_string();
    prop_assert_eq!(lhs, rhs);
    Ok(())
}

proptest! {
    #[test]
    fn proptest_file(file in &*ARB_FILE) {
        test_roundtrip(file)
    }

    #[test]
    fn proptest_item(item in &*ARB_ITEM) {
        test_roundtrip(item)
    }

    #[test]
    fn proptest_sys(sys in &*ARB_SYS) {
        test_roundtrip(sys)
    }

    #[test]
    fn proptest_stmt(stmt in &*ARB_STMT) {
        test_roundtrip(stmt)
    }

    #[test]
    fn proptest_read(read in &*ARB_READ) {
        test_roundtrip(read)
    }

    #[test]
    fn proptest_write(write in &*ARB_WRITE) {
        test_roundtrip(write)
    }

    #[test]
    fn proptest_struct(s in &*ARB_STRUCT) {
        test_roundtrip(s)
    }

    #[test]
    fn proptest_enum(e in &*ARB_ENUM) {
        test_roundtrip(e)
    }

    #[test]
    fn proptest_ty(e in &*ARB_TY) {
        test_roundtrip(e)
    }

    #[test]
    fn proptest_binding(e in &*ARB_BINDING) {
        test_roundtrip(e)
    }

    #[test]
    fn proptest_expr(e in &*ARB_EXPR) {
        test_roundtrip(e)
    }

    #[test]
    fn proptest_term(e in &*ARB_TERM) {
        test_roundtrip(e)
    }

    #[test]
    fn proptest_symbol(symbol in arb_symbol()) {
        test_roundtrip(symbol)
    }
}
