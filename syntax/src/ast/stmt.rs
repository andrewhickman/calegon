use std::fmt;

use ast::{Bind, Expr};

#[derive(Debug)]
pub enum Stmt {
    Bind(Bind),
    Expr(Expr),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Bind(bind) => bind.fmt(f),
            Stmt::Expr(expr) => expr.fmt(f),
        }
    }
}
