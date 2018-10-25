use proptest::prelude::*;

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
fn subtypes() {
    assert!(subtype(polar::i32_neg(), polar::i32_pos()));
}
