mod build;
mod head;

use syntax::Symbol;

use variance::Polarity;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum FieldAlphabet {
    Label(Symbol),
    Domain,
    Range,
}

type StateId = u32;

pub struct State {
    pol: Polarity,
    cons: head::ConstructorSet,
    trans: Vec<(FieldAlphabet, StateId)>,
}

impl State {
    fn empty(pol: Polarity) -> Self {
        State {
            pol,
            cons: head::ConstructorSet::empty(),
            trans: Vec::new(),
        }
    }
}

pub struct Automaton {
    states: Vec<State>,
}

impl Automaton {
    pub fn start(&self) -> &State {
        &self.states[0]
    }
}
