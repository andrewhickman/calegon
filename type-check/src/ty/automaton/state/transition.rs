use std::cmp::Ordering;
use std::fmt;

#[cfg(test)]
use iter_set;
use syntax;

use ty::automaton::state::StateId;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(in ty::automaton) enum Symbol {
    Domain,
    Range,
    Label(syntax::Symbol),
}

#[derive(Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub(in ty::automaton) struct Transition {
    pub symbol: Symbol,
    pub to: StateId,
}

pub(in ty::automaton) struct TransitionSet {
    inner: Vec<Transition>,
}

impl TransitionSet {
    pub fn new() -> Self {
        TransitionSet { inner: Vec::new() }
    }

    pub fn add(&mut self, symbol: Symbol, to: StateId) {
        let tr = Transition { symbol, to };
        if let Err(idx) = self.inner.binary_search(&tr) {
            self.inner.insert(idx, tr);
        }
    }

    pub fn union(&mut self, other: &Self) {
        self.inner.extend(&other.inner);
        self.inner.sort();
        self.inner.dedup();
    }

    pub fn split_at_domain(&self) -> (Vec<StateId>, Vec<StateId>) {
        let split = self
            .inner
            .binary_search_by(|tr| Ord::cmp(&tr.symbol, &Symbol::Domain).then(Ordering::Less))
            .unwrap_err();
        let (domain, rest) = self.inner.split_at(split);
        (
            domain.iter().map(|tr| tr.to).collect(),
            rest.iter().map(|tr| tr.to).collect(),
        )
    }

    #[cfg(test)]
    pub fn get_for(&self, symbol: Symbol) -> &[Transition] {
        let hi = self
            .inner
            .binary_search_by(|tr| Ord::cmp(&tr.symbol, &symbol).then(Ordering::Less))
            .unwrap_err();
        let lo = self.inner[..hi]
            .binary_search_by(|tr| Ord::cmp(&tr.symbol, &symbol).then(Ordering::Greater))
            .unwrap_err();
        &self.inner[lo..hi]
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Label(l) => write!(f, "{}", l),
            Symbol::Domain => write!(f, "𝒹"),
            Symbol::Range => write!(f, "𝓇"),
        }
    }
}

impl fmt::Debug for Transition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} → {:?}", self.symbol, self.to)
    }
}

impl fmt::Debug for TransitionSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_set().entries(self.inner.iter()).finish()
    }
}
