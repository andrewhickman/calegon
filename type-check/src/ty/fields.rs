use std::cmp::Ordering;
use std::iter::FromIterator;
use std::rc::Rc;

use iter_set;

use syntax::Symbol;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Fields<T> {
    inner: Rc<[(Symbol, T)]>,
}

impl<T> Fields<T> {
    pub fn new(mut fields: Vec<(Symbol, T)>) -> Self {
        fields.sort_by_key(key);
        Fields {
            inner: fields.into(),
        }
    }

    pub fn get(&self) -> &[(Symbol, T)] {
        &self.inner
    }

    pub fn get_value(&self, k: Symbol) -> Option<&T> {
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

impl<T> FromIterator<(Symbol, T)> for Fields<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Symbol, T)>,
    {
        Fields::new(Vec::from_iter(iter))
    }
}

fn key<T>(&(key, _): &(Symbol, T)) -> Symbol {
    key
}
