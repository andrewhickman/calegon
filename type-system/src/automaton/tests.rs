use automaton::{dfa, nfa, Automaton};

proptest! {
    #[test]
    fn proptest_append_ty(a in nfa::arb_ty_pos(), b in nfa::arb_ty_pos()) {
        let mut lhs = Automaton::new();
        lhs.reduce(&a, a.start());
        lhs.reduce(&b, b.start());

        let mut rhs = Automaton::new();
        rhs.append(dfa::Ty::new(&a).as_mut());
        rhs.append(dfa::Ty::new(&b).as_mut());

        prop_assert_eq!(lhs.as_ref(), rhs.as_ref());
    }
}
