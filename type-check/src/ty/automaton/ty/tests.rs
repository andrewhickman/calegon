use ty::automaton::state::constructor::Constructor;
use ty::automaton::state::transition;
use ty::automaton::Ty;
use ty::polar;
use ty::Var;
use variance::Polarity;

#[test]
fn test() {
    let ty = Ty::new(polar::recursive_pos(polar::fn_pos(
        polar::fn_neg(polar::var_pos(0), polar::var_neg(1)),
        polar::var_pos(1),
    )));

    assert_eq!(ty.start().polarity(), Polarity::Pos);
    assert!(ty.start().has_constructor(&Constructor::Fn));

    let range = {
        let range = ty.start().get_transitions(transition::Symbol::Range);
        assert_eq!(range.len(), 1);
        range[0].to
    };
    assert_eq!(ty.states[range].polarity(), Polarity::Pos);
    assert!(ty.states[range].has_constructor(&Constructor::Var(Var(1))));

    let domain = {
        let domain = ty.start().get_transitions(transition::Symbol::Domain);
        assert_eq!(domain.len(), 1);
        domain[0].to
    };
    assert_eq!(ty.states[domain].polarity(), Polarity::Neg);
    assert!(ty.states[domain].has_constructor(&Constructor::Fn));

    let range = {
        let range = ty.states[domain].get_transitions(transition::Symbol::Range);
        assert_eq!(range.len(), 1);
        range[0].to
    };
    assert_eq!(ty.states[range].polarity(), Polarity::Neg);
    assert!(ty.states[range].has_constructor(&Constructor::Var(Var(1))));

    let domain = {
        let domain = ty.states[domain].get_transitions(transition::Symbol::Domain);
        assert_eq!(domain.len(), 1);
        domain[0].to
    };
    assert_eq!(domain, ty.start);
}

proptest!{
    #[test]
    fn proptest_construct_nfa_pos(ty in polar::arb_ty_pos()) {
        Ty::new(ty);
    }

    #[test]
    fn proptest_construct_nfa_neg(ty in polar::arb_ty_neg()) {
        Ty::new(ty);
    }
}
