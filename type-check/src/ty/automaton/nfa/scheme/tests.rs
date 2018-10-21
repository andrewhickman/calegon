use std::collections::HashSet;

use proptest::collection::vec;

use ty::automaton::nfa::Scheme;
use ty::polar;

prop_compose! {
    [pub] fn arb_scheme()(env in vec(polar::arb_ty_neg(), 1..20), expr in polar::arb_ty_pos())
        -> Scheme
    {
        Scheme::new(env, expr)
    }
}

proptest!{
    #[test]
    fn proptest_construct(_ in arb_scheme()) {}

    #[test]
    fn proptest_biunify(mut scheme in arb_scheme()) {
        let mut seen = HashSet::new();
        let qp = scheme.expr;
        let qn = scheme.env[0];
        let _ = scheme.biunify(&mut seen, qp, qn);
    }
}
