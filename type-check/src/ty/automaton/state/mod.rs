pub(in ty::automaton) mod constructor;
pub(in ty::automaton) mod transition;

use ty::automaton::state::constructor::{Constructor, ConstructorSet};
use ty::automaton::state::transition::TransitionSet;
use variance::Polarity;

pub(in ty::automaton) type StateId = usize;

pub(in ty::automaton) struct State {
    pol: Polarity,
    cons: ConstructorSet,
    trans: TransitionSet,
}

impl State {
    pub fn new(pol: Polarity) -> Self {
        State {
            pol,
            cons: ConstructorSet::new(),
            trans: TransitionSet::new(),
        }
    }

    pub fn polarity(&self) -> Polarity {
        self.pol
    }

    pub fn combine(&self, other: &Self) -> Self {
        assert_eq!(self.polarity(), other.polarity());
        State {
            pol: self.polarity(),
            cons: self.cons.add_set(self.polarity(), &other.cons),
            trans: self.trans.union(&other.trans),
        }
    }

    pub fn add_constructor(&mut self, con: Constructor) {
        self.cons.add(self.pol, con)
    }

    pub fn add_transition(&mut self, symbol: transition::Symbol, to: StateId) {
        self.trans.add(symbol, to)
    }
}
