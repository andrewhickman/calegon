#[cfg(test)]
mod tests;

#[cfg(test)]
pub use self::tests::*;

use ty::automaton::nfa::build;
use ty::automaton::state::{State, StateId};
use ty::polar;
use variance::AsPolarity;

#[derive(Debug)]
pub struct Ty {
    states: Vec<State>,
    start: StateId,
}

impl Ty {
    pub(in ty::automaton) fn start(&self) -> &State {
        &self.states[self.start]
    }

    pub(in ty::automaton) fn start_id(&self) -> StateId {
        self.start
    }

    pub(in ty::automaton) fn states(&self) -> &[State] {
        &self.states
    }

    pub fn new<P: AsPolarity>(ty: polar::Ty<P>) -> Self {
        let mut states = Vec::new();
        let start = build(&mut states, ty);
        Ty { states, start }
    }
}
