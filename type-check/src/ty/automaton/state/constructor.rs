use std::cmp::Ordering;
use std::fmt;

use iter_set;

use ty::{Fields, Var};
use variance::Polarity;

#[derive(Clone, Debug)]
pub(in ty::automaton) enum Constructor {
    Fn,
    I32,
    Struct(Fields<()>),
    Var(Var),
}

pub(in ty::automaton::state) struct ConstructorSet {
    inner: Vec<Constructor>,
}

impl ConstructorSet {
    pub fn new() -> Self {
        ConstructorSet { inner: Vec::new() }
    }

    fn singleton(con: Constructor) -> Self {
        ConstructorSet { inner: vec![con] }
    }

    pub fn add(&mut self, pol: Polarity, con: Constructor) {
        match pol {
            Polarity::Pos => self.join(con),
            Polarity::Neg => self.meet(con),
        }
    }

    pub fn add_set(&self, pol: Polarity, other: &Self) -> Self {
        match pol {
            Polarity::Pos => self.join_set(other),
            Polarity::Neg => self.meet_set(other),
        }
    }

    fn join(&mut self, other: Constructor) {
        // TODO: optimize
        *self = self.join_set(&ConstructorSet::singleton(other))
    }

    fn join_set(&self, other: &Self) -> Self {
        ConstructorSet {
            inner: iter_set::union_by(
                self.inner.iter().cloned(),
                other.inner.iter().cloned(),
                Constructor::join,
            ).collect(),
        }
    }

    fn meet(&mut self, other: Constructor) {
        // TODO: optimize
        *self = self.meet_set(&ConstructorSet::singleton(other))
    }

    fn meet_set(&self, other: &Self) -> Self {
        ConstructorSet {
            inner: iter_set::union_by(
                self.inner.iter().cloned(),
                other.inner.iter().cloned(),
                Constructor::meet,
            ).collect(),
        }
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

impl fmt::Debug for ConstructorSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_set().entries(self.inner.iter()).finish()
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
            (Var(a), Var(b)) if a == b => Some(Ordering::Equal),
            (I32, I32) => Some(Ordering::Equal),
            (Fn, Fn) => Some(Ordering::Equal),
            (Struct(l), Struct(r)) => l.cmp_labels(r),
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
