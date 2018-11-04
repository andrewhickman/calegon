use std::error::Error;
use std::str::FromStr;
use std::string::ToString;

use proptest::prelude::*;

use {ast, Symbol};

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
    fn proptest_file(file in any::<ast::File>()) {
        test_roundtrip(file)
    }

    #[test]
    fn proptest_symbol(symbol in any::<Symbol>()) {
        test_roundtrip(symbol)
    }
}
