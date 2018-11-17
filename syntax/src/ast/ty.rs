use std::fmt;

use ast::{Map, Tuple};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Ty {
    Tuple(Tuple<Ty>),
    Struct(Map<Ty>),
    Fn(Box<Fn>),
    I32,
    Unit,
    Never,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Fn {
    pub domain: Ty,
    pub range: Ty,
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ty::Tuple(t) => t.fmt(f),
            Ty::Struct(s) => s.fmt(f),
            Ty::Fn(x) => x.fmt(f),
            Ty::I32 => "i32".fmt(f),
            Ty::Unit => "unit".fmt(f),
            Ty::Never => "never".fmt(f),
        }
    }
}

impl fmt::Display for Fn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.domain, self.range)
    }
}
