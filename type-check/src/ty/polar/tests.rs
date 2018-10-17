use std::collections::HashSet;
use std::sync::Mutex;

use proptest::collection::vec;
use proptest::prelude::*;
use proptest::strategy::Just;
use syntax::arb_symbol;

use ty::automaton;
use ty::polar::{Ty, TyKind};
use ty::{Fields, Var};
use variance::{Neg, Pos};

fn intern_pos(ty: TyKind<'static, Pos>) -> Ty<'static, Pos> {
    lazy_static! {
        static ref CONTEXT: Mutex<HashSet<&'static TyKind<'static, Pos>>> = Default::default();
    }

    let mut ctx = CONTEXT.lock().unwrap();
    let kind = if let Some(&interned) = ctx.get(&ty) {
        interned
    } else {
        Box::leak(ty.into())
    };
    ctx.insert(kind);
    Ty { kind, pol: Pos }
}

fn intern_neg(ty: TyKind<'static, Neg>) -> Ty<'static, Neg> {
    lazy_static! {
        static ref CONTEXT: Mutex<HashSet<&'static TyKind<'static, Neg>>> = Default::default();
    }

    let mut ctx = CONTEXT.lock().unwrap();
    let kind = if let Some(&interned) = ctx.get(&ty) {
        interned
    } else {
        Box::leak(ty.into())
    };
    ctx.insert(kind);
    Ty { kind, pol: Neg }
}

pub fn arb_ty_pos() -> BoxedStrategy<Ty<'static, Pos>> {
    arb_ty_pos_impl(|_| arb_ty_neg())
}

fn arb_ty_pos_impl<F>(neg: F) -> BoxedStrategy<Ty<'static, Pos>>
where
    F: Fn(BoxedStrategy<Ty<'static, Pos>>) -> BoxedStrategy<Ty<'static, Neg>> + 'static,
{
    prop_oneof![
        Just(intern_pos(TyKind::Zero)),
        Just(intern_pos(TyKind::I32)),
        (0usize..8).prop_map(|idx| intern_pos(TyKind::Var(Var::new(idx)))),
    ].prop_recursive(4, 16, 4, move |inner| {
        prop_oneof! {
            (inner.clone(), inner.clone()).prop_map(|(l, r)| intern_pos(TyKind::Add(l, r))),
            (neg(inner.clone()), inner.clone()).prop_map(|(d, r)| intern_pos(TyKind::Fn(d, r))),
            vec((arb_symbol(), inner.clone()), 0..4).prop_map(|fields| intern_pos(TyKind::Struct(Fields::new(fields)))),
            inner.prop_map(|ty| intern_pos(TyKind::Recursive(ty))),
        }
    }).boxed()
}

pub fn arb_ty_neg() -> BoxedStrategy<Ty<'static, Neg>> {
    arb_ty_neg_impl(|_| arb_ty_pos())
}

fn arb_ty_neg_impl<F>(pos: F) -> BoxedStrategy<Ty<'static, Neg>>
where
    F: Fn(BoxedStrategy<Ty<'static, Neg>>) -> BoxedStrategy<Ty<'static, Pos>> + 'static,
{
    prop_oneof![
        Just(intern_neg(TyKind::Zero)),
        Just(intern_neg(TyKind::I32)),
        (0usize..8).prop_map(|idx| intern_neg(TyKind::Var(Var::new(idx)))),
    ].prop_recursive(4, 16, 4, move |inner| {
        prop_oneof! {
            (inner.clone(), inner.clone()).prop_map(|(l, r)| intern_neg(TyKind::Add(l, r))),
            (pos(inner.clone()), inner.clone()).prop_map(|(d, r)| intern_neg(TyKind::Fn(d, r))),
            vec((arb_symbol(), inner.clone()), 0..4).prop_map(|fields| intern_neg(TyKind::Struct(Fields::new(fields)))),
            inner.prop_map(|ty| intern_neg(TyKind::Recursive(ty))),
        }
    }).boxed()
}

proptest!{
    #[test]
    fn proptest_construct_nfa_pos(ty in arb_ty_pos()) {
        let nfa = automaton::Ty::new(ty);

        ::std::fs::write("pos.ty", format!("{:#?}", ty));
        ::std::fs::write("pos.nfa", format!("{:#?}", nfa));
    }

    #[test]
    fn proptest_construct_nfa_neg(ty in arb_ty_neg()) {
        let nfa = automaton::Ty::new(ty);

        ::std::fs::write("neg.ty", format!("{:#?}", ty));
        ::std::fs::write("neg.nfa", format!("{:#?}", nfa));
    }
}
