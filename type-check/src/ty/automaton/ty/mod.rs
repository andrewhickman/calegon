#[cfg(test)]
mod tests;

use std::fmt;

use ty::automaton::build::build;
use ty::automaton::state::{State, StateId};
use ty::polar;
use variance::AsPolarity;

pub struct Ty {
    states: Vec<State>,
    start: StateId,
}

impl fmt::Debug for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.states.fmt(f)
    }
}

impl Ty {
    fn start(&self) -> &State {
        &self.states[self.start]
    }

    pub fn new<P: AsPolarity>(ty: polar::Ty<P>) -> Self {
        let mut states = Vec::new();
        let start = build(&mut states, ty);
        Ty { states, start }
    }
}
