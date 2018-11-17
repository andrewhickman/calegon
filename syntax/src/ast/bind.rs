use std::fmt;

use ast::{Expr, Ty};
use Symbol;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Bind {
    pub name: Symbol,
    pub ty: Option<Ty>,
    pub val: Expr,
}

impl fmt::Display for Bind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "let {}", self.name)?;
        if let Some(ty) = &self.ty {
            write!(f, ": {}", ty)?;
        }
        write!(f, " = {}", self.val)
    }
}
