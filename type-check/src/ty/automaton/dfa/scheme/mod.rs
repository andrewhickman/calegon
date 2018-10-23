#[cfg(test)]
mod tests;

use ty::automaton::state::StateId;
use ty::automaton::{nfa, Automaton};

#[derive(Debug, Clone)]
pub struct Scheme {
    auto: Automaton,
    env: Vec<StateId>,
    expr: StateId,
}

impl Scheme {
    pub fn new(nfa: &nfa::Scheme) -> Self {
        let mut auto = Automaton::new();
        let expr = auto.reduce(nfa, nfa.expr());
        let env = nfa.env().iter().map(|&id| auto.reduce(nfa, id)).collect();
        auto.populate_flow();
        Scheme { auto, expr, env }
    }

    pub fn expr(&self) -> StateId {
        self.expr
    }

    pub fn env(&self) -> &[StateId] {
        &self.env
    }
}

impl AsRef<Automaton> for Scheme {
    fn as_ref(&self) -> &Automaton {
        &self.auto
    }
}

impl AsMut<Automaton> for Scheme {
    fn as_mut(&mut self) -> &mut Automaton {
        &mut self.auto
    }
}
