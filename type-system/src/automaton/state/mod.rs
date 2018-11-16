pub(in automaton) mod constructor;
pub(in automaton) mod flow;
pub(in automaton) mod transition;

use automaton::state::constructor::{Constructor, ConstructorSet};
use automaton::state::flow::FlowSet;
use automaton::state::transition::TransitionSet;
use variance::Polarity;

pub(crate) type StateId = usize;

pub(in automaton) const REJECT: StateId = usize::max_value();

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) struct State {
    pol: Polarity,
    cons: ConstructorSet,
    trans: TransitionSet,
    pub(in automaton) flow: FlowSet,
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
        let mut result = states.next().expect("cannot merge 0 states").to_dstate();
        for state in states {
            result.merge(state);
        }
        result
    }

    pub fn polarity(&self) -> Polarity {
        self.pol
    }

    pub(in automaton) fn add_constructor(&mut self, con: &Constructor) {
        self.cons.add(self.pol, con)
    }

    pub(in automaton) fn add_transition(&mut self, symbol: transition::Symbol, to: StateId) {
        self.trans.add(symbol, to)
    }

    pub fn merge(&mut self, other: &Self) {
        debug_assert_eq!(self.polarity(), other.polarity());
        self.trans.union(&other.trans);
        self.cons.add_set(self.pol, &other.cons);
    }

    pub(in automaton) fn constructors(&self) -> &ConstructorSet {
        &self.cons
    }

    pub(in automaton) fn transitions(&self) -> &TransitionSet {
        &self.trans
    }

    pub(in automaton) fn transitions_mut(&mut self) -> &mut TransitionSet {
        &mut self.trans
    }

    pub fn shift(&mut self, n: StateId) {
        self.trans.shift(n);
        self.flow.shift(n);
    }

    pub fn to_dstate(&self) -> Self {
        State {
            pol: self.pol,
            cons: self.cons.clone(),
            trans: self.trans.clone(),
            flow: FlowSet::new(),
        }
    }
}
