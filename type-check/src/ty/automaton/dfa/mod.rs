mod reduce;
mod scheme;
#[cfg(test)]
mod tests;
mod ty;

pub use self::ty::Ty;

pub(in ty::automaton) use self::reduce::reduce;
