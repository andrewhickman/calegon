pub(in ty::automaton) mod constructor;
pub(in ty::automaton) mod flow;
pub(in ty::automaton) mod transition;

use std::ops;

use ty::automaton::state::constructor::{Constructor, ConstructorSet};
use ty::automaton::state::flow::FlowSet;
use ty::automaton::state::transition::TransitionSet;
use variance::Polarity;

pub(in ty::automaton) type StateId = usize;

pub(in ty::automaton) const REJECT: StateId = usize::max_value();

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(in ty::automaton) struct State {
    pol: Polarity,
    cons: ConstructorSet,
    trans: TransitionSet,
    pub(in ty::automaton) flow: FlowSet,
}

#[derive(Debug, Clone)]
pub(in ty::automaton) struct Automaton {
    states: Vec<State>,
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

    pub fn add_constructor(&mut self, con: &Constructor) {
        self.cons.add(self.pol, con)
    }

    pub fn add_transition(&mut self, symbol: transition::Symbol, to: StateId) {
        self.trans.add(symbol, to)
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

    pub fn transitions_mut(&mut self) -> &mut TransitionSet {
        &mut self.trans
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

impl Automaton {
    pub fn new() -> Self {
        Automaton { states: Vec::new() }
    }

    pub fn add(&mut self, state: State) -> StateId {
        let id = self.states.len();
        self.states.push(state);
        id
    }

    pub fn reserve(&mut self, additional: usize) {
        self.states.reserve(additional)
    }
}

impl AsRef<Automaton> for Automaton {
    fn as_ref(&self) -> &Automaton {
        self
    }
}

impl AsMut<Automaton> for Automaton {
    fn as_mut(&mut self) -> &mut Automaton {
        self
    }
}

impl ops::Deref for Automaton {
    type Target = [State];

    fn deref(&self) -> &Self::Target {
        &self.states
    }
}

impl ops::DerefMut for Automaton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.states
    }
}
