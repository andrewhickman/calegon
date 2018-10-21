use ty::automaton::nfa;
use ty::automaton::nfa::arb_ty;
use ty::polar;

proptest! {
    #[test]
    fn reduce(nfa in arb_ty()) {
        nfa.reduce()
    }
}
