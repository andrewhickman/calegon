use std::fmt;

use ast::{Map, Tuple};
use Symbol;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Ty {
    Tuple(Tuple<Ty>),
    Struct(Map<Ty>),
    Fun(Box<Fun>),
    Alias(Symbol),
    I32,
    Unit,
    Never,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Fun {
    pub domain: Ty,
    pub range: Ty,
}

impl From<Symbol> for Ty {
    fn from(name: Symbol) -> Self {
        Ty::Alias(name)
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ty::Tuple(t) => t.fmt(f),
            Ty::Struct(s) => s.fmt(f),
            Ty::Fun(fun) => fun.fmt(f),
            Ty::Alias(alias) => alias.fmt(f),
            Ty::I32 => "i32".fmt(f),
            Ty::Unit => "unit".fmt(f),
            Ty::Never => "never".fmt(f),
        }
    }
}

impl fmt::Display for Fun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.domain, self.range)
    }
}
