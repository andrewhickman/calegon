use std::iter::FromIterator;
use std::str::FromStr;

use proptest::collection::vec;
use proptest::option;
use proptest::prelude::*;
use proptest::strategy::LazyJust;
use proptest_recurse::{StrategyExt, StrategySet};

use ast;
use symbol::{Symbol, SymbolMap};

impl Arbitrary for ast::File {
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            vec(any_with::<ast::Stmt>(set.clone()), 0..8)
                .prop_map(|stmts| ast::File { stmts })
                .sboxed()
        })
    }
}

impl Arbitrary for ast::Stmt {
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            prop_oneof![
                any_with::<ast::Bind>(set.clone()).prop_map(ast::Stmt::Bind),
                any_with::<ast::Expr>(set.clone()).prop_map(ast::Stmt::Expr),
            ].sboxed()
        })
    }
}

impl Arbitrary for ast::Bind {
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            (
                any::<Symbol>(),
                option::of(any_with::<ast::Ty>(set.clone())),
                any_with::<ast::Expr>(set.clone()),
            )
                .prop_map(|(name, ty, val)| ast::Bind { name, ty, val })
                .sboxed()
        })
    }
}

impl Arbitrary for ast::Ty {
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            prop_oneof![
                LazyJust::new(|| ast::Ty::I32),
                LazyJust::new(|| ast::Ty::Unit),
                LazyJust::new(|| ast::Ty::Never),
            ].prop_mutually_recursive(4, 32, 8, set, |set| {
                prop_oneof![
                    any_with::<ast::Tuple<ast::Ty>>(set.clone()).prop_map(ast::Ty::Tuple),
                    any_with::<ast::Map<ast::Ty>>(set.clone()).prop_map(ast::Ty::Struct),
                    any_with::<Box<ast::ty::Fn>>(set.clone()).prop_map(ast::Ty::Fn),
                ].sboxed()
            })
        })
    }
}

impl Arbitrary for ast::ty::Fn {
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            (
                any_with::<ast::Ty>(set.clone()),
                any_with::<ast::Ty>(set.clone()),
            )
                .prop_map(|(domain, range)| ast::ty::Fn { domain, range })
                .sboxed()
        })
    }
}

impl Arbitrary for ast::Expr {
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            any_with::<ast::expr::Lit>(set.clone())
                .prop_map(ast::Expr::Lit)
                .prop_mutually_recursive(4, 32, 8, set, |set| {
                    any_with::<ast::Scope<ast::Stmt, Box<ast::Expr>>>(set.clone())
                        .prop_map(ast::Expr::Scope)
                        .sboxed()
                })
        })
    }
}

impl Arbitrary for ast::expr::Lit {
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            any::<i32>()
                .prop_map(ast::expr::Lit::Int)
                .prop_mutually_recursive(4, 32, 8, set, |set| {
                    any_with::<ast::Map<ast::Expr>>(set.clone())
                        .prop_map(ast::expr::Lit::Struct)
                        .sboxed()
                })
        })
    }
}

impl<B, T> Arbitrary for ast::Scope<B, T>
where
    B: Arbitrary<Parameters = StrategySet> + Send + Sync + 'static,
    B::Strategy: Send + Sync,
    T: Arbitrary<Parameters = StrategySet> + Send + Sync + 'static,
    T::Strategy: Send + Sync,
{
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            (
                vec(any_with::<B>(set.clone()), 0..8),
                option::of(any_with::<T>(set.clone())),
            )
                .prop_map(|(body, tail)| ast::Scope { body, tail })
                .sboxed()
        })
    }
}

impl<T> Arbitrary for ast::Tuple<T>
where
    T: Arbitrary<Parameters = StrategySet> + Send + Sync + 'static,
    T::Strategy: Send + Sync,
{
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            vec(any_with::<T>(set.clone()), 0..8)
                .prop_map(ast::Tuple)
                .sboxed()
        })
    }
}

impl<T> Arbitrary for ast::Map<T>
where
    T: Arbitrary<Parameters = StrategySet> + Send + Sync + 'static,
    T::Strategy: Send + Sync,
{
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            vec((any::<Symbol>(), any_with::<T>(set.clone())), 0..8)
                .prop_map(SymbolMap::from_iter)
                .prop_map(ast::Map)
                .sboxed()
        })
    }
}

impl Arbitrary for Symbol {
    type Parameters = ();
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        "_?[[:alpha:]](?:_?[[:alnum:]]){0,5}"
            .prop_filter_map("invalid symbol", |string| Symbol::from_str(&string).ok())
            .sboxed()
    }
}
