use std::cmp::Ordering;

use iter_set;

use ty::automaton::StateId;
use ty::polar::{TyNeg, TyPos};
use ty::Fields;

pub(in ty::automaton) enum Constructor {
    Fn,
    I32,
    Struct(Fields<()>),
    Var(StateId),
}

pub(in ty::automaton) struct ConstructorSet {
    inner: Vec<Constructor>,
}

impl ConstructorSet {
    pub fn empty() -> Self {
        ConstructorSet { inner: vec![] }
    }

    pub fn singleton(con: Constructor) -> Self {
        ConstructorSet { inner: vec![con] }
    }

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

    pub fn join(&mut self, other: &mut Self) {
        let result = ConstructorSet {
            inner: iter_set::union_by(
                self.inner.drain(..),
                other.inner.drain(..),
                Constructor::join,
            ).collect(),
        };
        *self = result;
    }

    pub fn meet(&mut self, other: &mut Self) {
        let result = ConstructorSet {
            inner: iter_set::intersection_by(
                self.inner.drain(..),
                other.inner.drain(..),
                Constructor::meet,
            ).collect(),
        };
        *self = result;
    }
}

impl Constructor {
    fn join(&mut self, other: &mut Self) -> Ordering {
        use self::Constructor::*;

        match (self, other) {
            (Var(a), Var(b)) => Ord::cmp(a, b),
            (Struct(a), Struct(b)) => {
                a.intersection(b);
                Ordering::Equal
            }
            (l, r) => compare_discriminants(l, r),
        }
    }

    fn meet(&mut self, other: &mut Self) -> Ordering {
        use self::Constructor::*;

        match (self, other) {
            (Var(a), Var(b)) => Ord::cmp(a, b),
            (Struct(a), Struct(b)) => {
                a.union(b);
                Ordering::Equal
            }
            (l, r) => compare_discriminants(l, r),
        }
    }
}

impl PartialEq for Constructor {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Constructor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use self::Constructor::*;

        match (self, other) {
            (Var(l), Var(r)) if l == r => Some(Ordering::Equal),
            (I32, I32) => Some(Ordering::Equal),
            (Fn, Fn) => Some(Ordering::Equal),
            (Struct(l), Struct(r)) => l.partial_cmp(r),
            _ => None,
        }
    }
}

fn compare_discriminants<T>(lhs: &T, rhs: &T) -> Ordering {
    use std::hash::{Hash, Hasher};
    use std::mem::{discriminant, Discriminant};

    fn hash<T>(discr: Discriminant<T>) -> u64 {
        let mut hasher = ::hash_hasher::HashHasher::default();
        discr.hash(&mut hasher);
        hasher.finish()
    }

    Ord::cmp(&hash(discriminant(lhs)), &hash(discriminant(rhs)))
}
