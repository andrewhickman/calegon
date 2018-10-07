mod build;
mod head;
mod state;

pub use self::state::{State, StateId};

use ty::polar::{TyNeg, TyPos};

use syntax::Symbol;

enum FieldAlphabet {
    Symbol(Symbol),
    Domain,
    Range,
}

pub struct Automaton {
    states: Vec<State>,
}

impl Automaton {
    pub fn start(&self) -> &State {
        &self.states[0]
    }
}
