use ty::automaton::state::{State, StateId};

#[derive(Debug)]
pub struct Ty {
    pub(in ty::automaton::dfa) states: Vec<State>,
    pub(in ty::automaton::dfa) start: StateId,
}

impl Ty {
    fn start(&self) -> &State {
        &self.states[self.start]
    }
}
