//mod constraint;
mod context;
mod visitor;

pub use self::visitor::Visitor;

use std::hash::{Hash, Hasher};
use std::ptr;

use ty::Fields;
use variance::Polarity;

#[derive(Debug)]
pub enum Ty<'c> {
    Pos(TyPos<'c>),
    Neg(TyNeg<'c>),
}

#[derive(Debug)]
pub enum TyPos<'c> {
    Var(u32),
    Join(&'c TyPos<'c>, &'c TyPos<'c>),
    Never,
    I32,
    Fn(&'c TyNeg<'c>, &'c TyPos<'c>),
    Struct(Fields<TyPos<'c>>),
    Recursive(&'c TyPos<'c>),
}

#[derive(Debug)]
pub enum TyNeg<'c> {
    Var(u32),
    Meet(&'c TyNeg<'c>, &'c TyNeg<'c>),
    Unit,
    I32,
    Fn(&'c TyPos<'c>, &'c TyNeg<'c>),
    Struct(Fields<TyNeg<'c>>),
    Recursive(&'c TyNeg<'c>),
}

impl<'c> Ty<'c> {
    pub fn polarity(&self) -> Polarity {
        match self {
            Ty::Pos(_) => Polarity::Pos,
            Ty::Neg(_) => Polarity::Neg,
        }
    }
}

/*
pub fn from_pos(term: &TyPos) -> Self {
    let inner = match *term {
        TyPos::Var(idx) => vec![Constructor::Var(idx)],
        TyPos::I32 => vec![Constructor::I32],
        TyPos::Fn(_, _) => vec![Constructor::Fn],
        TyPos::Struct(ref fields) => vec![Constructor::Struct(fields.labels())],
        _ => vec![],
    };

    ConstructorSet { inner }
}

pub fn from_neg(term: &TyNeg) -> Self {
    let inner = match *term {
        TyNeg::Var(idx) => vec![Constructor::Var(idx)],
        TyNeg::I32 => vec![Constructor::I32],
        TyNeg::Fn(_, _) => vec![Constructor::Fn],
        TyNeg::Struct(ref fields) => vec![Constructor::Struct(fields.labels())],
        _ => vec![],
    };

    ConstructorSet { inner }
}
*/

/*
impl<'c> Hash for &'c TyPos<'c> {
    fn hash<H: Hasher>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write_usize(*self as *const TyPos as _)
    }
}

impl<'c> PartialEq for &'c TyPos<'c> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq::<TyPos<'c>>(*self, *other)
    }
}

impl<'c> Eq for &'c TyPos<'c> {}

impl<'c> Hash for &'c TyNeg<'c> {
    fn hash<H: Hasher>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write_usize(*self as *const TyNeg as _)
    }
}

impl<'c> PartialEq for &'c TyNeg<'c> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq::<TyNeg<'c>>(*self, *other)
    }
}

impl<'c> Eq for &'c TyNeg<'c> {}
*/
