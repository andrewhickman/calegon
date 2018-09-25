use proptest::collection::vec;
use proptest::prelude::*;
use proptest::strategy::LazyJust;
use proptest::string::string_regex;

use {ast, Symbol};

fn arb_symbol() -> impl Strategy<Value = Symbol> {
    string_regex("[[:alpha:]][[:alnum:]]{0,5}")
        .unwrap()
        .prop_filter("symbols is a keyword", |string| {
            string != "comp"
                && string != "enum"
                && string != "i32"
                && string != "never"
                && string != "read"
                && string != "struct"
                && string != "sys"
                && string != "type"
                && string != "unit"
                && string != "write"
        }).prop_map(|string| Symbol::intern(0, &string).unwrap())
}

prop_compose! {
    [pub] fn arb_file()(items in vec(arb_item(), 0..64)) -> ast::File {
        ast::File { items }
    }
}

pub fn arb_item() -> BoxedStrategy<ast::Item> {
    prop_oneof![
        arb_comp().prop_map(ast::Item::Comp),
        arb_ty_def().prop_map(ast::Item::TyDef),
    ].prop_recursive(3, 32, 16, |inner| {
        arb_sys_impl(inner).prop_map(ast::Item::Sys)
    }).boxed()
}

pub fn arb_sys() -> impl Strategy<Value = ast::Sys> {
    arb_sys_impl(arb_item())
}

prop_compose! {
    fn arb_sys_impl(
        item: impl Strategy<Value = ast::Item>
    )(
        name in arb_symbol(),
        stmts in vec(arb_stmt_impl(item), 0..30)
    ) -> ast::Sys {
        ast::Sys { name, stmts }
    }
}

pub fn arb_stmt() -> impl Strategy<Value = ast::Stmt> {
    arb_stmt_impl(arb_item())
}

fn arb_stmt_impl(item: impl Strategy<Value = ast::Item>) -> impl Strategy<Value = ast::Stmt> {
    prop_oneof![
        45 => arb_read().prop_map(ast::Stmt::Read),
        45 => arb_write().prop_map(ast::Stmt::Write),
        10 => item.prop_map(ast::Stmt::Item),
    ]
}

prop_compose! {
    [pub] fn arb_read()(comps in vec(arb_symbol(), 0..8)) -> ast::Read {
        ast::Read { comps }
    }
}

prop_compose! {
    [pub] fn arb_write()(comps in vec(arb_symbol(), 0..8)) -> ast::Write {
        ast::Write { comps }
    }
}

prop_compose! {
    [pub] fn arb_comp()(name in arb_symbol(), ty in arb_ty()) -> ast::Comp {
        ast::Comp { name, ty }
    }
}

prop_compose! {
    [pub] fn arb_ty_def()(name in arb_symbol(), ty in arb_ty()) -> ast::TyDef {
        ast::TyDef { name, ty }
    }
}

pub fn arb_ty() -> BoxedStrategy<ast::Ty> {
    prop_oneof![
        LazyJust::new(|| ast::Ty::Never),
        LazyJust::new(|| ast::Ty::Unit),
        LazyJust::new(|| ast::Ty::I32),
        arb_symbol().prop_map(ast::Ty::TyDef),
    ].prop_recursive(4, 16, 8, |inner| {
        prop_oneof! {
            arb_struct_impl(inner.clone()).prop_map(ast::Ty::Struct),
            arb_enum_impl(inner.clone()).prop_map(ast::Ty::Enum),
        }
    }).boxed()
}

pub fn arb_struct() -> impl Strategy<Value = ast::Struct> {
    arb_struct_impl(arb_ty())
}

prop_compose! {
    fn arb_struct_impl(
        ty: impl Strategy<Value = ast::Ty>
    )(
        fields in vec((arb_symbol(), ty), 0..8)
    ) -> ast::Struct {
        ast::Struct { fields }
    }
}

pub fn arb_enum() -> impl Strategy<Value = ast::Enum> {
    arb_enum_impl(arb_ty())
}

prop_compose! {
    fn arb_enum_impl(
        ty: impl Strategy<Value = ast::Ty>
    )(
        fields in vec((arb_symbol(), ty), 0..8)
    ) -> ast::Enum {
        ast::Enum { fields }
    }
}
