mod build;
mod state;

use ty::automaton::state::State;

pub struct Automaton {
    states: Vec<State>,
}

impl Automaton {
    fn start(&self) -> &State {
        &self.states[0]
    }
}
