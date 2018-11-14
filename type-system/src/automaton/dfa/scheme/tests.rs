use std::collections::HashSet;

use proptest::prelude::*;

use automaton::dfa::Scheme;
use automaton::nfa;
use automaton::state::StateId;

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

proptest! {
    #[test]
    fn proptest_construct(_ in arb_scheme()) {}

    #[test]
    fn proptest_subsume((mut scheme, q1, q2) in arb_scheme_and_indices()) {
        let mut seen = HashSet::new();
        let q1 = scheme.env[q1];
        let q2 = scheme.env[q2];
        let _ = scheme.as_mut().subsume(&mut seen, q1, q2);
    }

    #[test]
    fn proptest_subsume_eq(mut scheme in arb_scheme()) {
        let qn = scheme.env[0];
        assert!(scheme.as_mut().subsume(&mut Default::default(), qn, qn).is_ok());
    }

    #[test]
    fn proptest_admissible((mut scheme, q1, _) in arb_scheme_and_indices()) {
        let qp = scheme.expr;
        let qn = scheme.env[q1];
        scheme.as_mut().admissible(qn, qp);
    }

    #[test]
    fn proptest_biunify(mut nfa in nfa::arb_scheme()) {
        let mut dfa = Scheme::new(&nfa);
        let nqp = nfa.expr();
        let nqn = nfa.env()[0];
        let dqp = dfa.expr();
        let dqn = dfa.env()[0];

        assert_eq!(
            dfa.as_mut().biunify(&mut Default::default(), dqp, dqn),
            nfa.as_mut().biunify(&mut Default::default(), nqp, nqn)
        );
    }
}
