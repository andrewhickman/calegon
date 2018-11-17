use std::fmt;

use ast::{Bind, Expr};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Stmt {
    Bind(Bind),
    Expr(Expr),
    If(If),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct If {
    pub cond: Expr,
    pub cons: Expr,
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Bind(bind) => bind.fmt(f),
            Stmt::Expr(expr) => expr.fmt(f),
            Stmt::If(i) => i.fmt(f),
        }
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "if {} then {}", self.cond, self.cons)
    }
}
