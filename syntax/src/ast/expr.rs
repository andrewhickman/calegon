use std::fmt;

use ast::{Map, Scope, Stmt};

#[derive(Debug)]
pub enum Expr {
    Lit(Lit),
    Scope(Scope<Stmt, Box<Expr>>),
}

#[derive(Debug)]
pub enum Lit {
    Int(i32),
    Struct(Map<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Lit(lit) => lit.fmt(f),
            Expr::Scope(scope) => scope.fmt(f),
        }
    }
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lit::Int(i) => i.fmt(f),
            Lit::Struct(map) => map.fmt(f),
        }
    }
}
