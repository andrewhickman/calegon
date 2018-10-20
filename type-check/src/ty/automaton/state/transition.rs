use std::cmp::Ordering;
use std::fmt;

use syntax;

use ty::automaton::state::StateId;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(in ty::automaton) enum Symbol {
    Label(syntax::Symbol),
    Domain,
    Range,
}

#[derive(Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub(in ty::automaton) struct Transition {
    pub symbol: Symbol,
    pub to: StateId,
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

    #[cfg(test)]
    pub fn get(&self, symbol: Symbol) -> &[Transition] {
        let hi = match self
            .inner
            .binary_search_by(|tr| Ord::cmp(&tr.symbol, &symbol).then(Ordering::Less))
        {
            Ok(_) => unreachable!(),
            Err(idx) => idx,
        };
        let lo = match self.inner[..hi]
            .binary_search_by(|tr| Ord::cmp(&tr.symbol, &symbol).then(Ordering::Greater))
        {
            Ok(_) => unreachable!(),
            Err(idx) => idx,
        };
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
