use std::collections::HashSet;

use proptest::prelude::*;

use ty::automaton::dfa::Scheme;
use ty::automaton::nfa;
use ty::automaton::state::StateId;

prop_compose! {
    [pub] fn arb_scheme()(scheme in nfa::arb_scheme())
        -> Scheme
    {
        Scheme::new(&scheme)
    }
}

fn arb_scheme_and_indices() -> impl Strategy<Value = (Scheme, StateId, StateId)> {
    arb_scheme().prop_flat_map(|scheme| {
        let len = scheme.env.len();
        (Just(scheme), 0..len, 0..len)
    })
}

proptest!{
    #[test]
    fn proptest_construct(_ in arb_scheme()) {}

    #[test]
    fn proptest_subsume((mut scheme, q1, q2) in arb_scheme_and_indices()) {
        let mut seen = HashSet::new();
        let q1 = scheme.env[q1];
        let q2 = scheme.env[q2];
        let _ = scheme.subsume(&mut seen, q1, q2);
    }

    #[test]
    fn proptest_subsume_eq(mut scheme in arb_scheme()) {
        let mut seen = HashSet::new();
        let qn = scheme.env[0];
        assert!(scheme.subsume(&mut seen, qn, qn).is_ok());
    }

    #[test]
    fn proptest_admissible((mut scheme, q1, _) in arb_scheme_and_indices()) {
        let qp = scheme.expr;
        let qn = scheme.env[q1];
        scheme.admissible(qn, qp);
    }
}
