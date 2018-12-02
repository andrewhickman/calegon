use std::fmt;

use ast::{Expr, Ty};
use symbol::Symbol;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Stmt {
    Let(Let),
    Fun(Fun),
    Expr(Expr),
    If(If),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Let {
    pub name: Symbol,
    pub ty: Option<Ty>,
    pub val: Expr,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Fun {
    pub name: Symbol,
    pub arg: Symbol,
    pub val: Expr,
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
            Stmt::Let(l) => l.fmt(f),
            Stmt::Fun(fun) => fun.fmt(f),
            Stmt::Expr(expr) => expr.fmt(f),
            Stmt::If(i) => i.fmt(f),
        }
    }
}

impl fmt::Display for Let {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "let {}", self.name)?;
        if let Some(ty) = &self.ty {
            write!(f, ": {}", ty)?;
        }
        write!(f, " = {}", self.val)
    }
}

impl fmt::Display for Fun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fun {} {} = {}", self.name, self.arg, self.name)
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "if {} then {}", self.cond, self.cons)
    }
}
