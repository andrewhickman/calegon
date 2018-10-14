use std::fmt;

use iter_set;
use syntax;

use ty::automaton::state::StateId;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(in ty::automaton) enum Symbol {
    Label(syntax::Symbol),
    Domain,
    Range,
}

#[derive(Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
struct Transition {
    symbol: Symbol,
    to: StateId,
}

pub(in ty::automaton::state) struct TransitionSet {
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

    pub fn union(&self, other: &Self) -> Self {
        TransitionSet {
            inner: iter_set::union(&self.inner, &other.inner)
                .cloned()
                .collect(),
        }
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
