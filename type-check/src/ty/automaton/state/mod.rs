pub(in ty::automaton) mod constructor;
pub(in ty::automaton) mod transition;

use ty::automaton::state::constructor::{Constructor, ConstructorSet};
use ty::automaton::state::transition::TransitionSet;
use ty::Var;
use variance::Polarity;

pub(in ty::automaton) type StateId = usize;

#[derive(Debug)]
pub(in ty::automaton) struct State<T> {
    pol: Polarity,
    cons: ConstructorSet,
    trans: TransitionSet,
    pub flow: T,
}

impl<T: Default> State<T> {
    pub fn new(pol: Polarity) -> Self {
        State {
            pol,
            cons: ConstructorSet::new(),
            trans: TransitionSet::new(),
            flow: T::default(),
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

    pub fn take_vars(&mut self) -> Vec<Var> {
        self.cons.take_vars()
    }

    pub fn merge(&mut self, other: &Self) {
        debug_assert_eq!(self.polarity(), other.polarity());
        self.trans.union(&other.trans);
        self.cons.add_set(self.pol, &other.cons);
    }

    pub fn constructors(&self) -> &ConstructorSet {
        &self.cons
    }

    pub fn transitions(&self) -> &TransitionSet {
        &self.trans
    }
}
