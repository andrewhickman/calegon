pub mod dfa;
pub mod nfa;

mod state;

use std::ops;

use automaton::state::{State, StateId};

#[derive(Debug, Clone)]
pub(crate) struct Automaton {
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
