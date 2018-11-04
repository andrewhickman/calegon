use std::str::FromStr;

use lazy_static::lazy_static;
use proptest::collection::vec;
use proptest::num::i32;
use proptest::option;
use proptest::prelude::*;
use proptest::strategy::LazyJust;
use proptest::string::string_regex;
use proptest_recurse::{StrategyExt, StrategySet};

use {ast, Symbol};

pub fn arb_symbol() -> impl Strategy<Value = Symbol> {
    string_regex("_?[[:alpha:]](?:_?[[:alnum:]]){0,5}")
        .unwrap()
        .prop_filter_map("invalid symbol", |string| Symbol::from_str(&string).ok())
}

lazy_static! {
    pub static ref ARB_FILE: SBoxedStrategy<ast::File> = arb_file(&mut StrategySet::default());
}

fn arb_file(set: &mut StrategySet) -> SBoxedStrategy<ast::File> {
    vec(set.get::<ast::Item, _>(arb_item), 0..8)
        .prop_map(|items| ast::File { items })
        .sboxed()
}

lazy_static! {
    pub static ref ARB_ITEM: SBoxedStrategy<ast::Item> = arb_item(&mut StrategySet::default());
}

fn arb_item(set: &mut StrategySet) -> SBoxedStrategy<ast::Item> {
    prop_oneof![
        set.get::<ast::Comp, _>(arb_comp).prop_map(ast::Item::Comp),
        set.get::<ast::TyDef, _>(arb_ty_def)
            .prop_map(ast::Item::TyDef),
    ].prop_mutually_recursive(3, 16, 30, set, |set| {
        set.get::<ast::Sys, _>(arb_sys)
            .prop_map(ast::Item::Sys)
            .sboxed()
    })
}

lazy_static! {
    pub static ref ARB_SYS: SBoxedStrategy<ast::Sys> = arb_sys(&mut StrategySet::default());
}

fn arb_sys(set: &mut StrategySet) -> SBoxedStrategy<ast::Sys> {
    (arb_symbol(), vec(set.get::<ast::Stmt, _>(arb_stmt), 0..30))
        .prop_map(|(name, stmts)| ast::Sys { name, stmts })
        .sboxed()
}

lazy_static! {
    pub static ref ARB_STMT: SBoxedStrategy<ast::Stmt> = arb_stmt(&mut StrategySet::default());
}

fn arb_stmt(set: &mut StrategySet) -> SBoxedStrategy<ast::Stmt> {
    prop_oneof![
        15 => set.get::<ast::Read, _>(arb_read).prop_map(ast::Stmt::Read),
        15 => set.get::<ast::Write, _>(arb_write).prop_map(ast::Stmt::Write),
    ].prop_mutually_recursive(4, 32, 8, set, |set| {
        prop_oneof![
            30 => set.get::<ast::Expr, _>(arb_expr).prop_map(ast::Stmt::Expr),
            30 => set.get::<ast::Binding, _>(arb_binding).prop_map(ast::Stmt::Binding),
            10 => set.get::<ast::Item, _>(arb_item).prop_map(ast::Stmt::Item),
        ].sboxed()
    })
}

lazy_static! {
    pub static ref ARB_READ: SBoxedStrategy<ast::Read> = arb_read(&mut StrategySet::default());
}

fn arb_read(_: &mut StrategySet) -> SBoxedStrategy<ast::Read> {
    vec(arb_symbol(), 0..8)
        .prop_map(|comps| ast::Read { comps })
        .sboxed()
}

lazy_static! {
    pub static ref ARB_WRITE: SBoxedStrategy<ast::Write> = arb_write(&mut StrategySet::default());
}

fn arb_write(_: &mut StrategySet) -> SBoxedStrategy<ast::Write> {
    vec(arb_symbol(), 0..8)
        .prop_map(|comps| ast::Write { comps })
        .sboxed()
}

lazy_static! {
    pub static ref ARB_COMP: SBoxedStrategy<ast::Comp> = arb_comp(&mut StrategySet::default());
}

fn arb_comp(set: &mut StrategySet) -> SBoxedStrategy<ast::Comp> {
    (arb_symbol(), set.get::<ast::Ty, _>(arb_ty))
        .prop_map(|(name, ty)| ast::Comp { name, ty })
        .sboxed()
}

lazy_static! {
    pub static ref ARB_TY_DEF: SBoxedStrategy<ast::TyDef> = arb_ty_def(&mut StrategySet::default());
}

fn arb_ty_def(set: &mut StrategySet) -> SBoxedStrategy<ast::TyDef> {
    (arb_symbol(), set.get::<ast::Ty, _>(arb_ty))
        .prop_map(|(name, ty)| ast::TyDef { name, ty })
        .sboxed()
}

lazy_static! {
    pub static ref ARB_TY: SBoxedStrategy<ast::Ty> = arb_ty(&mut StrategySet::default());
}

fn arb_ty(set: &mut StrategySet) -> SBoxedStrategy<ast::Ty> {
    prop_oneof![
        LazyJust::new(|| ast::Ty::Never),
        LazyJust::new(|| ast::Ty::Unit),
        LazyJust::new(|| ast::Ty::I32),
        arb_symbol().prop_map(ast::Ty::TyDef),
    ].prop_mutually_recursive(4, 16, 8, set, |set| {
        prop_oneof![
            set.get::<ast::Struct, _>(arb_struct)
                .prop_map(ast::Ty::Struct),
            set.get::<ast::Enum, _>(arb_enum).prop_map(ast::Ty::Enum),
        ].sboxed()
    })
}

lazy_static! {
    pub static ref ARB_STRUCT: SBoxedStrategy<ast::Struct> =
        arb_struct(&mut StrategySet::default());
}

fn arb_struct(set: &mut StrategySet) -> SBoxedStrategy<ast::Struct> {
    vec((arb_symbol(), set.get::<ast::Ty, _>(arb_ty)), 0..8)
        .prop_map(|fields| ast::Struct { fields })
        .sboxed()
}

lazy_static! {
    pub static ref ARB_ENUM: SBoxedStrategy<ast::Enum> = arb_enum(&mut StrategySet::default());
}

fn arb_enum(set: &mut StrategySet) -> SBoxedStrategy<ast::Enum> {
    vec((arb_symbol(), set.get::<ast::Ty, _>(arb_ty)), 0..8)
        .prop_map(|fields| ast::Enum { fields })
        .sboxed()
}

lazy_static! {
    pub static ref ARB_BINDING: SBoxedStrategy<ast::Binding> =
        arb_binding(&mut StrategySet::default());
}

fn arb_binding(set: &mut StrategySet) -> SBoxedStrategy<ast::Binding> {
    (
        arb_symbol(),
        option::of(set.get::<ast::Ty, _>(arb_ty)),
        option::of(set.get::<ast::Expr, _>(arb_expr)),
    )
        .prop_map(|(name, ty, val)| ast::Binding { name, ty, val })
        .sboxed()
}

lazy_static! {
    pub static ref ARB_EXPR: SBoxedStrategy<ast::Expr> = arb_expr(&mut StrategySet::default());
}

fn arb_expr(set: &mut StrategySet) -> SBoxedStrategy<ast::Expr> {
    prop_oneof![
        i32::ANY.prop_map(ast::Expr::Literal),
        arb_symbol().prop_map(ast::Expr::Var),
    ].prop_mutually_recursive(4, 16, 8, set, |set| {
        let expr = set.get::<ast::Expr, _>(arb_expr);
        prop_oneof![
            vec((arb_symbol(), expr.clone()), 0..8).prop_map(ast::Expr::Struct),
            (expr.clone(), expr.clone())
                .prop_map(|(f, a)| ast::Expr::FnCall(Box::new(f), Box::new(a))),
            (
                vec(set.get::<ast::Stmt, _>(arb_stmt), 0..8),
                option::of(expr.clone())
            )
                .prop_map(|(stmts, tail)| ast::Expr::Scope(stmts, tail.map(Box::new))),
            (expr, arb_symbol()).prop_map(|(expr, name)| ast::Expr::Dot(Box::new(expr), name)),
        ].sboxed()
    })
}
