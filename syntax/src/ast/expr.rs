use std::fmt;

use ast::{Map, Scope, Stmt};
use Symbol;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Expr {
    Scope(Scope<Stmt, Box<Expr>>),
    Lit(Lit),
    Proj(Box<Proj>),
    If(Box<If>),
    App(Box<App>),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Lit {
    Int(i32),
    Struct(Map<Expr>),
    Var(Symbol),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Proj {
    pub expr: Expr,
    pub label: Symbol,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct If {
    pub cond: Expr,
    pub cons: Expr,
    pub alt: Expr,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct App {
    pub fun: Expr,
    pub param: Expr,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Scope(scope) => scope.fmt(f),
            Expr::Lit(lit) => lit.fmt(f),
            Expr::Proj(proj) => proj.fmt(f),
            Expr::If(i) => i.fmt(f),
            Expr::App(app) => app.fmt(f),
        }
    }
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lit::Int(i) => i.fmt(f),
            Lit::Struct(map) => map.fmt(f),
            Lit::Var(name) => name.fmt(f),
        }
    }
}

impl fmt::Display for Proj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.expr, self.label)
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "if {} then {} else {}", self.cond, self.cons, self.alt)
    }
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.fun, self.param)
    }
}
