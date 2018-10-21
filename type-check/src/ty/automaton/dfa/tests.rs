use ty::automaton::dfa;
use ty::automaton::nfa::arb_ty;

proptest! {
    #[test]
    fn reduce(nfa in arb_ty()) {
        let mut dfa = Vec::new();
        let start = dfa::reduce(&mut dfa, nfa.states(), nfa.start_id());

        let mut dfa2 = Vec::new();
        dfa::reduce(&mut dfa2, &dfa, start);

        assert_eq!(dfa, dfa2);
    }
}
