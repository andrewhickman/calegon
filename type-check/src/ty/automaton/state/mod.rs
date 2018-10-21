pub(in ty::automaton) mod constructor;
pub(in ty::automaton) mod flow;
pub(in ty::automaton) mod transition;

use ty::automaton::state::constructor::{Constructor, ConstructorSet};
use ty::automaton::state::flow::FlowSet;
use ty::automaton::state::transition::TransitionSet;
use ty::Var;
use variance::Polarity;

pub(in ty::automaton) type StateId = usize;

pub(in ty::automaton) const REJECT: StateId = usize::max_value();

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(in ty::automaton) struct State {
    pol: Polarity,
    pub(in ty::automaton) cons: ConstructorSet,
    pub(in ty::automaton) trans: TransitionSet,
    pub(in ty::automaton) flow: FlowSet,
}

impl State {
    pub fn new(pol: Polarity) -> Self {
        State {
            pol,
            cons: ConstructorSet::new(),
            trans: TransitionSet::new(),
            flow: FlowSet::new(),
        }
    }

    pub fn merged<'a, I>(states: I) -> Self
    where
        I: IntoIterator<Item = &'a Self>,
    {
        let mut states = states.into_iter();
        let mut result = states.next().expect("cannot merge 0 states").clone();
        for state in states {
            result.merge(state);
        }
        result
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
