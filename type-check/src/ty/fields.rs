use std::cmp::Ordering;
use std::iter::FromIterator;
use std::sync::Arc;

use iter_set;

use ty::polar::{Ty, Visitor};
use variance::AsPolarity;
use Label;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Fields<T> {
    inner: Arc<[(Label, T)]>,
}

impl<T> Default for Fields<T> {
    fn default() -> Self {
        Fields::new(Vec::new())
    }
}

impl<T> Fields<T> {
    pub fn new(fields: impl Into<Arc<[(Label, T)]>>) -> Self {
        let mut inner = fields.into();
        Arc::get_mut(&mut inner).unwrap().sort_by_key(key);
        Fields { inner }
    }

    pub fn get(&self) -> &[(Label, T)] {
        &self.inner
    }

    pub fn get_value(&self, k: Label) -> Option<&T> {
        match self.inner.binary_search_by_key(&k, key) {
            Ok(idx) => Some(&self.inner[idx].1),
            Err(_) => None,
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn labels(&self) -> Fields<()> {
        self.inner.iter().map(|&(l, _)| (l, ())).collect()
    }

    pub fn cmp_labels<U>(&self, other: &Fields<U>) -> Option<Ordering> {
        iter_set::cmp(self.inner.iter().map(key), other.inner.iter().map(key))
    }
}

impl<'c, P: AsPolarity + 'c> Fields<Ty<'c, P>> {
    pub fn accept<V: Visitor + ?Sized>(&self, visitor: &mut V) {
        for &(_, ty) in self.get() {
            ty.accept(visitor)
        }
    }
}

impl<T> Fields<T>
where
    T: Clone,
{
    pub fn union(&self, other: &Self) -> Self {
        iter_set::union_by_key(self.inner.iter().cloned(), other.inner.iter().cloned(), key)
            .collect()
    }

    pub fn intersection(&self, other: &Self) -> Self {
        iter_set::intersection_by_key(self.inner.iter().cloned(), other.inner.iter().cloned(), key)
            .collect()
    }
}

impl<T> Clone for Fields<T> {
    fn clone(&self) -> Self {
        Fields {
            inner: self.inner.clone(),
        }
    }
}

impl<T> FromIterator<(Label, T)> for Fields<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Label, T)>,
    {
        Fields::new(Vec::from_iter(iter))
    }
}

fn key<T>(&(key, _): &(Label, T)) -> Label {
    key
}
