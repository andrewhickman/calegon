use std::ops::Deref;

use automaton::{nfa, Automaton};

proptest! {
    #[test]
    fn reduce(nfa in nfa::arb_ty_pos()) {
        let mut dfa = Automaton::new();
        let start = dfa.reduce(&nfa, nfa.start());

        let mut dfa2 = Automaton::new();
        dfa2.reduce(&dfa, start);

        assert_eq!(dfa.deref(), dfa2.deref());
    }
}
