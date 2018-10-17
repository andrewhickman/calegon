use std::fmt;

mod build;

use ty::automaton::state::State;

pub struct Ty {
    states: Vec<State>,
}

impl fmt::Debug for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.states.fmt(f)
    }
}

impl Ty {
    fn start(&self) -> &State {
        &self.states[0]
    }
}
