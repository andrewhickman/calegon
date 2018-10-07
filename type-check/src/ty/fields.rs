use std::cmp::Ordering;

use iter_set;

use syntax::Symbol;

#[derive(Debug)]
pub struct Fields<T> {
    inner: Vec<(Symbol, T)>,
}

impl<T> Fields<T> {
    pub fn new(mut inner: Vec<(Symbol, T)>) -> Self {
        inner.sort_by_key(key);
        Fields { inner }
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
        Fields {
            inner: self.inner.iter().map(|&(l, _)| (l, ())).collect(),
        }
    }

    pub fn union(&mut self, other: &mut Self) {
        let result = Fields {
            inner: iter_set::union_by_key(self.inner.drain(..), other.inner.drain(..), key)
                .collect(),
        };
        *self = result;
    }

    pub fn intersection(&mut self, other: &mut Self) {
        let result = Fields {
            inner: iter_set::intersection_by_key(self.inner.drain(..), other.inner.drain(..), key)
                .collect(),
        };
        *self = result;
    }
}

fn key<T>(&(key, _): &(Symbol, T)) -> Symbol {
    key
}

impl<T, U> PartialEq<Fields<U>> for Fields<T> {
    fn eq(&self, other: &Fields<U>) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl<T, U> PartialOrd<Fields<U>> for Fields<T> {
    fn partial_cmp(&self, other: &Fields<U>) -> Option<Ordering> {
        iter_set::cmp(self.inner.iter().map(key), other.inner.iter().map(key))
    }
}
