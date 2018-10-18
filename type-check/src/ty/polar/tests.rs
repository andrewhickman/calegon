use std::collections::HashSet;
use std::hash::Hash;
use std::sync::Mutex;

use proptest::collection::vec;
use proptest::prelude::*;
use proptest::strategy::Just;
use syntax::{arb_symbol, Symbol};

use ty::automaton;
use ty::polar::{Ty, TyKind};
use ty::{Fields, Var};
use variance::{AsPolarity, Neg, Pos};

pub fn arb_ty_pos() -> impl Strategy<Value = Ty<'static, Pos>> {
    arb_ty::<Pos>()
}

pub fn arb_ty_neg() -> impl Strategy<Value = Ty<'static, Neg>> {
    arb_ty::<Neg>()
}

pub fn zero_pos() -> Ty<'static, Pos> {
    intern(TyKind::Zero)
}

pub fn zero_neg() -> Ty<'static, Neg> {
    intern(TyKind::Zero)
}

pub fn i32_pos() -> Ty<'static, Pos> {
    intern(TyKind::I32)
}

pub fn i32_neg() -> Ty<'static, Neg> {
    intern(TyKind::I32)
}

pub fn add_pos(lhs: Ty<'static, Pos>, rhs: Ty<'static, Pos>) -> Ty<'static, Pos> {
    intern(TyKind::Add(lhs, rhs))
}

pub fn add_neg(lhs: Ty<'static, Neg>, rhs: Ty<'static, Neg>) -> Ty<'static, Neg> {
    intern(TyKind::Add(lhs, rhs))
}

pub fn fn_pos(domain: Ty<'static, Neg>, range: Ty<'static, Pos>) -> Ty<'static, Pos> {
    intern(TyKind::Fn(domain, range))
}

pub fn fn_neg(domain: Ty<'static, Pos>, range: Ty<'static, Neg>) -> Ty<'static, Neg> {
    intern(TyKind::Fn(domain, range))
}

pub fn struct_pos(fields: Vec<(Symbol, Ty<'static, Pos>)>) -> Ty<'static, Pos> {
    intern(TyKind::Struct(Fields::new(fields)))
}

pub fn struct_neg(fields: Vec<(Symbol, Ty<'static, Neg>)>) -> Ty<'static, Neg> {
    intern(TyKind::Struct(Fields::new(fields)))
}

pub fn recursive_pos(ty: Ty<'static, Pos>) -> Ty<'static, Pos> {
    intern(TyKind::Recursive(ty))
}

pub fn recursive_neg(ty: Ty<'static, Neg>) -> Ty<'static, Neg> {
    intern(TyKind::Recursive(ty))
}

pub fn var_pos(idx: usize) -> Ty<'static, Pos> {
    intern(TyKind::Var(Var(idx)))
}

pub fn var_neg(idx: usize) -> Ty<'static, Neg> {
    intern(TyKind::Var(Var(idx)))
}

trait Intern: AsPolarity + Default + Eq + Hash + 'static {
    fn context() -> &'static Mutex<HashSet<&'static TyKind<'static, Self>>>;
}

impl Intern for Pos {
    fn context() -> &'static Mutex<HashSet<&'static TyKind<'static, Self>>> {
        lazy_static! {
            static ref CONTEXT: Mutex<HashSet<&'static TyKind<'static, Pos>>> = Default::default();
        }

        &CONTEXT
    }
}

impl Intern for Neg {
    fn context() -> &'static Mutex<HashSet<&'static TyKind<'static, Self>>> {
        lazy_static! {
            static ref CONTEXT: Mutex<HashSet<&'static TyKind<'static, Neg>>> = Default::default();
        }

        &CONTEXT
    }
}

fn intern<P>(ty: TyKind<'static, P>) -> Ty<'static, P>
where
    P: Intern,
    P::Neg: Intern<Neg = P>,
{
    let mut ctx = P::context().lock().unwrap();
    let kind = if let Some(&interned) = ctx.get(&ty) {
        interned
    } else {
        Box::leak(ty.into())
    };
    ctx.insert(kind);
    Ty {
        kind,
        pol: P::default(),
    }
}

fn negate<P>(ty: Ty<'static, P>) -> Ty<'static, P::Neg>
where
    P: Intern,
    P::Neg: Intern<Neg = P>,
{
    intern(match *ty.kind {
        TyKind::Zero => TyKind::Zero,
        TyKind::I32 => TyKind::I32,
        TyKind::Add(l, r) => TyKind::Add(negate(l), negate(r)),
        TyKind::Fn(l, r) => TyKind::Fn(negate(l), negate(r)),
        TyKind::Struct(ref fields) => TyKind::Struct(
            fields
                .get()
                .iter()
                .cloned()
                .map(|(l, t)| (l, negate(t)))
                .collect(),
        ),
        TyKind::Recursive(t) => TyKind::Recursive(negate(t)),
        TyKind::Var(var) => TyKind::Var(var),
    })
}

fn arb_ty<P>() -> impl Strategy<Value = Ty<'static, P>>
where
    P: Intern,
    P::Neg: Intern<Neg = P>,
{
    prop_oneof![
        Just(intern(TyKind::Zero)),
        Just(intern(TyKind::I32)),
        (0usize..8).prop_map(|idx| intern(TyKind::Var(Var(idx)))),
    ].prop_recursive(8, 32, 4, move |inner| {
        prop_oneof! {
            (inner.clone(), inner.clone()).prop_map(|(l, r)| intern(TyKind::Add(l, r))),
            (inner.clone().prop_map(negate), inner.clone()).prop_map(|(d, r)| intern(TyKind::Fn(d, r))),
            vec((arb_symbol(), inner.clone()), 0..8).prop_map(|fields| intern(TyKind::Struct(Fields::new(fields)))),
            inner.prop_map(|ty| intern(TyKind::Recursive(ty))),
        }
    }).prop_filter("invalid polar type", |ty| ty.check())
}

#[test]
fn guardedness() {
    assert!(recursive_pos(fn_pos(i32_neg(), var_pos(0))).check());
    assert!(!recursive_pos(add_pos(i32_pos(), var_pos(0))).check());
}

#[test]
fn covariance() {
    assert!(recursive_pos(fn_pos(fn_neg(var_pos(0), i32_neg()), i32_pos())).check());
    assert!(!recursive_pos(fn_pos(var_neg(0), i32_pos())).check());
}

proptest!{
    #[test]
    fn proptest_construct_nfa_pos(ty in arb_ty_pos()) {
        automaton::Ty::new(ty);
    }

    #[test]
    fn proptest_construct_nfa_neg(ty in arb_ty_neg()) {
        automaton::Ty::new(ty);
    }
}
