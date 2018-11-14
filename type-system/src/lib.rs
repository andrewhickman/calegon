#![allow(dead_code)]

extern crate calegon_syntax as syntax;
extern crate hash_hasher;
extern crate im;
extern crate iter_set;
#[macro_use]
extern crate itertools;
#[cfg(test)]
#[macro_use]
extern crate proptest;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
extern crate seahash;
extern crate typed_arena;

pub mod automaton;
pub mod check;
pub mod polar;

mod fields;
mod var;
mod variance;

pub use self::fields::Fields;
pub use self::var::Var;

type Label = syntax::Symbol;
