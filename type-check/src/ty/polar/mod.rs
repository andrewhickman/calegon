//mod constraint;
mod context;
mod visitor;

pub use self::visitor::Visitor;

use std::hash::{Hash, Hasher};
use std::ptr;

use ty::Fields;

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
