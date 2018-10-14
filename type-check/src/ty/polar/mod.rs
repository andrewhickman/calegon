mod context;
mod visitor;

pub use self::visitor::Visitor;

use std::hash::{Hash, Hasher};

use ty::Fields;
use variance::{AsPolarity, Polarity};

#[derive(Copy, Clone, Debug)]
pub struct Ty<'c, P: AsPolarity + 'c> {
    kind: &'c TyKind<'c, P>,
    pol: P,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum TyKind<'c, P: AsPolarity + 'c> {
    Add(Ty<'c, P>, Ty<'c, P>),
    Zero,
    I32,
    Fn(Ty<'c, P::Neg>, Ty<'c, P>),
    Struct(Fields<Ty<'c, P>>),
    Recursive(Ty<'c, P>),
    Var(u32),
}

impl<'c, P: AsPolarity + 'c> Ty<'c, P> {
    pub fn polarity(&self) -> Polarity {
        self.pol.as_polarity()
    }

    fn as_ptr(&self) -> *const TyKind<'c, P> {
        self.kind
    }
}

impl<'c, P: AsPolarity + 'c> Hash for Ty<'c, P> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ptr().hash(state)
    }
}

impl<'c, P: AsPolarity + 'c> PartialEq for Ty<'c, P> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

impl<'c, P: AsPolarity + 'c> Eq for Ty<'c, P> {}
