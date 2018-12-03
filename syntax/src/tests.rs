use std::error::Error;
use std::iter::FromIterator;
use std::str::FromStr;
use std::string::ToString;

use proptest::prelude::*;

use {ast, Symbol};

fn test_roundtrip<T>(value: T) -> Result<(), TestCaseError>
where
    T: FromStr + ToString,
    T::Err: Error,
{
    let lhs = value.to_string();
    let value = T::from_str(&lhs).unwrap();
    let rhs = value.to_string();
    prop_assert_eq!(lhs, rhs);
    Ok(())
}

proptest! {
    #[test]
    fn proptest_file(file in any::<ast::File>()) {
        test_roundtrip(file)
    }

    #[test]
    fn proptest_symbol(symbol in any::<Symbol>()) {
        test_roundtrip(symbol)
    }
}

fn parse_stmt(input: &str) -> ast::Stmt {
    let file_str = format!("{};", input);
    let file = ast::File::from_str(&file_str).unwrap();
    assert_eq!(file.stmts.len(), 1);
    file.stmts.into_iter().next().unwrap()
}

fn parse_expr(input: &str) -> ast::Expr {
    match parse_stmt(input) {
        ast::Stmt::Expr(expr) => expr,
        _ => panic!("wrong stmt type"),
    }
}

fn var(s: &str) -> ast::Expr {
    Symbol::from_str(s).unwrap().into()
}

#[test]
fn expr_precedence() {
    use ast::expr::*;

    assert_eq!(
        parse_expr("f x.a"),
        Expr::App(Box::new(App {
            fun: var("f"),
            param: Expr::Proj(Box::new(Proj {
                expr: var("x"),
                label: Symbol::from_str("a").unwrap(),
            }))
        }))
    );

    assert_eq!(
        parse_expr("f g x"),
        Expr::App(Box::new(App {
            fun: var("f"),
            param: Expr::App(Box::new(App {
                fun: var("g"),
                param: var("x"),
            }))
        }))
    );

    assert_eq!(
        parse_expr("if a then b else f c"),
        Expr::If(Box::new(If {
            cond: var("a"),
            cons: var("b"),
            alt: Expr::App(Box::new(App {
                fun: var("f"),
                param: var("c"),
            })),
        })),
    );

    assert_eq!(
        parse_expr("if a then b else c.x"),
        Expr::If(Box::new(If {
            cond: var("a"),
            cons: var("b"),
            alt: Expr::Proj(Box::new(Proj {
                expr: var("c"),
                label: Symbol::from_str("x").unwrap(),
            })),
        })),
    );
}

#[test]
fn stmt_precedence() {
    use ast::*;

    assert_eq!(
        parse_stmt("if a then if b then s else s2"),
        ast::Stmt::If(stmt::If {
            cond: var("a"),
            cons: Expr::If(Box::new(expr::If {
                cond: var("b"),
                cons: var("s"),
                alt: var("s2"),
            })),
        })
    );
}

#[test]
fn map_sugar() {
    use ast::*;
    use SymbolMap;

    assert_eq!(
        parse_expr("{ x: a, y }"),
        Expr::Lit(expr::Lit::Struct(Map(SymbolMap::from_iter(vec![
            (Symbol::from_str("x").unwrap(), var("a")),
            (Symbol::from_str("y").unwrap(), var("y")),
        ]))))
    )
}
