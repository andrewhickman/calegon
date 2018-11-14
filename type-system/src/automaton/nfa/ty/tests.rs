use proptest::prelude::*;

use automaton::nfa::Ty;
use automaton::state::constructor::Constructor;
use automaton::state::transition;
use polar;
use variance::{Neg, Polarity, Pos};
use Var;

pub fn arb_ty_pos() -> impl Strategy<Value = Ty<Pos>> {
    polar::arb_ty_pos().prop_map(Ty::new)
}

pub fn arb_ty_neg() -> impl Strategy<Value = Ty<Neg>> {
    polar::arb_ty_neg().prop_map(Ty::new)
}

#[test]
fn test() {
    let ty = Ty::new(polar::recursive_pos(polar::fn_pos(
        polar::fn_neg(polar::var_pos(0), polar::var_neg(1)),
        polar::var_pos(1),
    )));
    let auto = ty.as_ref();

    assert_eq!(auto[ty.start()].polarity(), Polarity::Pos);
    assert!(auto[ty.start()].constructors().has(&Constructor::Fn));

    let range = {
        let range = auto[ty.start()]
            .transitions()
            .getn(transition::Symbol::Range);
        assert_eq!(range.len(), 1);
        range[0].to
    };
    assert_eq!(auto[range].polarity(), Polarity::Pos);
    assert!(auto[range].constructors().has(&Constructor::Var(Var(1))));

    let domain = {
        let domain = auto[ty.start()]
            .transitions()
            .getn(transition::Symbol::Domain);
        assert_eq!(domain.len(), 1);
        domain[0].to
    };
    assert_eq!(auto[domain].polarity(), Polarity::Neg);
    assert!(auto[domain].constructors().has(&Constructor::Fn));

    let range = {
        let range = auto[domain].transitions().getn(transition::Symbol::Range);
        assert_eq!(range.len(), 1);
        range[0].to
    };
    assert_eq!(auto[range].polarity(), Polarity::Neg);
    assert!(auto[range].constructors().has(&Constructor::Var(Var(1))));

    let domain = {
        let domain = auto[domain].transitions().getn(transition::Symbol::Domain);
        assert_eq!(domain.len(), 1);
        domain[0].to
    };
    assert_eq!(domain, ty.start);
}

proptest!{
    #[test]
    fn proptest_construct(_ in arb_ty_pos()) {}
}
