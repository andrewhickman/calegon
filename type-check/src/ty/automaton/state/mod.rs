pub(in ty::automaton) mod constructor;
pub(in ty::automaton) mod transition;

use ty::automaton::state::constructor::{Constructor, ConstructorSet};
use ty::automaton::state::transition::{Transition, TransitionSet};
use variance::Polarity;

pub(in ty::automaton) type StateId = usize;

#[derive(Debug)]
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

    pub fn add_constructor(&mut self, con: &Constructor) {
        self.cons.add(self.pol, con)
    }

    pub fn add_transition(&mut self, symbol: transition::Symbol, to: StateId) {
        self.trans.add(symbol, to)
    }

    #[cfg(test)]
    pub fn get_transitions(&self, symbol: transition::Symbol) -> &[Transition] {
        self.trans.get(symbol)
    }

    #[cfg(test)]
    pub fn has_constructor(&self, con: &Constructor) -> bool {
        self.cons.has(con)
    }
}
