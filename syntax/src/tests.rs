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
    fn proptest_file(file in arb_file()) {
        test_roundtrip(file)
    }

    #[test]
    fn proptest_item(item in arb_item()) {
        test_roundtrip(item)
    }

    #[test]
    fn proptest_sys(sys in arb_sys()) {
        test_roundtrip(sys)
    }

    #[test]
    fn proptest_stmt(stmt in arb_stmt()) {
        test_roundtrip(stmt)
    }

    #[test]
    fn proptest_read(read in arb_read()) {
        test_roundtrip(read)
    }

    #[test]
    fn proptest_write(write in arb_write()) {
        test_roundtrip(write)
    }

    #[test]
    fn proptest_struct(s in arb_struct()) {
        test_roundtrip(s)
    }

    #[test]
    fn proptest_enum(e in arb_enum()) {
        test_roundtrip(e)
    }

    #[test]
    fn proptest_symbol(symbol in arb_symbol()) {
        test_roundtrip(symbol)
    }

}
