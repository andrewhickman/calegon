use syntax::Symbol;

pub struct Fields<T> {
    inner: Vec<(Symbol, T)>,
}

impl<T> Fields<T> {
    pub fn new(mut inner: Vec<(Symbol, T)>) -> Self {
        inner.sort_by_key(|field| field.0);
        Fields { inner }
    }

    pub fn contains<U>(&self, other: &Fields<U>) -> bool {
        self.get().iter().zip(other.get()).all(|(l, r)| l.0 == r.0)
    }

    pub fn get(&self) -> &[(Symbol, T)] {
        &self.inner
    }
}
