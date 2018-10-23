use std::ops::Deref;

use ty::automaton::nfa::arb_ty;
use ty::automaton::Automaton;

proptest! {
    #[test]
    fn reduce(nfa in arb_ty()) {
        let mut dfa = Automaton::new();
        let start = dfa.reduce(&nfa, nfa.start());

        let mut dfa2 = Automaton::new();
        dfa2.reduce(&dfa, start);

        assert_eq!(dfa.deref(), dfa2.deref());
    }
}
