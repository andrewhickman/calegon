use ty::automaton::state::{State, StateId};
use ty::automaton::{dfa, nfa};

#[derive(Debug)]
pub struct Ty {
    states: Vec<State>,
    start: StateId,
}

impl Ty {
    fn start(&self) -> &State {
        &self.states[self.start]
    }

    pub fn new(nfa: nfa::Ty) -> Self {
        let mut states = Vec::new();
        let start = dfa::reduce(&mut states, nfa.states(), nfa.start_id());
        Ty { states, start }
    }
}
