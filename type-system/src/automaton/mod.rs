pub mod dfa;
pub mod nfa;

#[cfg(test)]
mod tests;

mod state;

use std::ops;

use automaton::state::{State, StateId};

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(in automaton) struct Automaton {
    states: Vec<State>,
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

    pub fn append(&mut self, other: &mut Self) -> StateId {
        let n = self.states.len();
        self.states.append(&mut other.states);
        for state in &mut self.states[n..] {
            state.shift(n);
        }
        n
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
