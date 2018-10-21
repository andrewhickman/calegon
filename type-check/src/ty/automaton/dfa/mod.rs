mod reduce;
mod scheme;
#[cfg(test)]
mod tests;
mod ty;

pub use self::scheme::Scheme;
pub use self::ty::*;

pub(in ty::automaton) use self::reduce::reduce;
