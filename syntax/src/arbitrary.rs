use std::iter::FromIterator;
use std::str::FromStr;

use lazy_static::lazy_static;
use proptest::collection::vec;
use proptest::option;
use proptest::prelude::*;
use proptest::strategy::LazyJust;
use proptest::string::string_regex;
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

    fn arbitrary() -> Self::Strategy {
        lazy_static! {
            static ref STRATEGY: SBoxedStrategy<ast::File> =
                any_with::<ast::File>(StrategySet::default());
        }

        STRATEGY.clone()
    }
}

impl Arbitrary for ast::Stmt {
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            prop_oneof![
                any_with::<ast::stmt::Let>(set.clone()).prop_map(ast::Stmt::Let),
                any_with::<ast::stmt::Fun>(set.clone()).prop_map(ast::Stmt::Fun),
                any_with::<ast::Expr>(set.clone()).prop_map(ast::Stmt::Expr),
                any_with::<ast::stmt::If>(set.clone()).prop_map(ast::Stmt::If),
            ].sboxed()
        })
    }
}

impl Arbitrary for ast::stmt::Let {
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            (
                any::<Symbol>(),
                option::of(any_with::<ast::Ty>(set.clone())),
                any_with::<ast::Expr>(set.clone()),
            )
                .prop_map(|(name, ty, val)| ast::stmt::Let { name, ty, val })
                .sboxed()
        })
    }
}

impl Arbitrary for ast::stmt::Fun {
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            (
                any::<Symbol>(),
                any::<Symbol>(),
                any_with::<ast::Expr>(set.clone()),
            )
                .prop_map(|(name, arg, val)| ast::stmt::Fun { name, arg, val })
                .sboxed()
        })
    }
}

impl Arbitrary for ast::stmt::If {
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            let expr = any_with::<ast::Expr>(set.clone());
            (expr.clone(), expr.clone())
                .prop_map(|(cond, cons)| ast::stmt::If { cond, cons })
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
            ].prop_mutually_recursive(2, 16, 8, set, |set| {
                prop_oneof![
                    any_with::<ast::Tuple<ast::Ty>>(set.clone()).prop_map(ast::Ty::Tuple),
                    any_with::<ast::Map<ast::Ty>>(set.clone()).prop_map(ast::Ty::Struct),
                    any_with::<Box<ast::ty::Fun>>(set.clone()).prop_map(ast::Ty::Fun),
                ].sboxed()
            })
        })
    }
}

impl Arbitrary for ast::ty::Fun {
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            (
                any_with::<ast::Ty>(set.clone()),
                any_with::<ast::Ty>(set.clone()),
            )
                .prop_map(|(domain, range)| ast::ty::Fun { domain, range })
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
                .prop_mutually_recursive(2, 16, 8, set, |set| {
                    prop_oneof![
                        any_with::<ast::Scope<ast::Stmt, Box<ast::Expr>>>(set.clone())
                            .prop_map(ast::Expr::Scope),
                        any_with::<Box<ast::expr::Proj>>(set.clone()).prop_map(ast::Expr::Proj),
                        any_with::<Box<ast::expr::If>>(set.clone()).prop_map(ast::Expr::If),
                        any_with::<Box<ast::expr::App>>(set.clone()).prop_map(ast::Expr::App),
                    ].sboxed()
                })
        })
    }
}

impl Arbitrary for ast::expr::Lit {
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            prop_oneof![
                any::<i32>().prop_map(ast::expr::Lit::Int),
                any::<Symbol>().prop_map(ast::expr::Lit::Var)
            ].prop_mutually_recursive(2, 8, 8, set, |set| {
                any_with::<ast::Map<ast::Expr>>(set.clone())
                    .prop_map(ast::expr::Lit::Struct)
                    .sboxed()
            })
        })
    }
}

impl Arbitrary for ast::expr::Proj {
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            (any_with::<ast::Expr>(set.clone()), any::<Symbol>())
                .prop_map(|(expr, label)| ast::expr::Proj { expr, label })
                .sboxed()
        })
    }
}

impl Arbitrary for ast::expr::If {
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            let expr = any_with::<ast::Expr>(set.clone());
            (expr.clone(), expr.clone(), expr.clone())
                .prop_map(|(cond, cons, alt)| ast::expr::If { cond, cons, alt })
                .sboxed()
        })
    }
}

impl Arbitrary for ast::expr::App {
    type Parameters = StrategySet;
    type Strategy = SBoxedStrategy<Self>;

    fn arbitrary_with(mut set: Self::Parameters) -> Self::Strategy {
        set.get(|set| {
            let expr = any_with::<ast::Expr>(set.clone());
            (expr.clone(), expr.clone())
                .prop_map(|(fun, param)| ast::expr::App { fun, param })
                .sboxed()
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
        lazy_static! {
            static ref STRATEGY: SBoxedStrategy<Symbol> =
                string_regex("_?[[:alpha:]](?:_?[[:alnum:]]){0,5}")
                    .unwrap()
                    .prop_filter_map("invalid symbol", |string| Symbol::from_str(&string).ok())
                    .sboxed();
        }

        STRATEGY.clone()
    }
}
