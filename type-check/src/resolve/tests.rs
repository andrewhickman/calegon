use proptest::prelude::*;
use syntax::ast::File;

use resolve::resolve;

proptest! {
    #[test]
    fn proptest_resolve(file in any::<File>()) {
        let _ = resolve(&file);
    }
}
