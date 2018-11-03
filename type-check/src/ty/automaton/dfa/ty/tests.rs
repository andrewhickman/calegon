use proptest::prelude::*;
use syntax::arb_symbol;

use ty::automaton::dfa::Ty;
use ty::automaton::{nfa, Automaton};
use ty::polar;
use variance::{Neg, Pos};

pub fn arb_ty_pos() -> impl Strategy<Value = Ty<Pos>> {
    nfa::arb_ty_pos().prop_map(|nfa| Ty::new(&nfa))
}

pub fn arb_ty_neg() -> impl Strategy<Value = Ty<Neg>> {
    nfa::arb_ty_neg().prop_map(|nfa| Ty::new(&nfa))
}

pub fn subtype(lhs: polar::Ty<Neg>, rhs: polar::Ty<Pos>) -> bool {
    let lhs = nfa::Ty::new(lhs);
    let rhs = nfa::Ty::new(rhs);

    let mut auto = Automaton::new();
    let lhs = auto.reduce(&lhs, lhs.start());
    let rhs = auto.reduce(&rhs, rhs.start());
    auto.populate_flow();
    auto.admissible(lhs, rhs)
}

#[test]
fn subtyping() {
    assert!(subtype(polar::i32_neg(), polar::i32_pos()));
}

proptest! {
    #[test]
    fn proptest_struct_subtyping(
        n in polar::arb_ty_neg(),
        p in polar::arb_ty_pos(),
        a in arb_symbol(),
        b in arb_symbol()
    ) {
        subtype(
            polar::struct_neg(vec![
                (b, polar::i32_neg()),
                (a, n)
            ]),
            polar::struct_pos(vec![(a, p)]),
        ) == subtype(n, p)
    }

    #[test]
    fn proptest_fn_domain_subtyping(
        n in polar::arb_ty_neg(),
        p in polar::arb_ty_pos(),
    ) {
        subtype(
            polar::fn_neg(p, polar::i32_neg()),
            polar::fn_pos(n, polar::i32_pos()),
        ) == subtype(n, p)
    }

    #[test]
    fn proptest_fn_range_subtyping(
        n in polar::arb_ty_neg(),
        p in polar::arb_ty_pos(),
    ) {
        subtype(
            polar::fn_neg(polar::i32_pos(), n),
            polar::fn_pos(polar::i32_neg(), p),
        ) == subtype(n, p)
    }
}
