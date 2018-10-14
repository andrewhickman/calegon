use std::fmt;

mod build;
mod state;

use ty::automaton::state::State;

pub struct Automaton {
    states: Vec<State>,
}

impl fmt::Debug for Automaton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.states.fmt(f)
    }
}

impl Automaton {
    fn start(&self) -> &State {
        &self.states[0]
    }
}
