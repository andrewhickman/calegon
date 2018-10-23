use ty::automaton::state::StateId;
use ty::automaton::{nfa, Automaton};

#[derive(Debug, Clone)]
pub struct Ty {
    auto: Automaton,
    start: StateId,
}

impl Ty {
    fn start(&self) -> StateId {
        self.start
    }

    pub fn new(nfa: &nfa::Ty) -> Self {
        let mut auto = Automaton::new();
        let start = auto.reduce(&nfa, nfa.start());
        Ty { auto, start }
    }
}

impl AsRef<Automaton> for Ty {
    fn as_ref(&self) -> &Automaton {
        &self.auto
    }
}

impl AsMut<Automaton> for Ty {
    fn as_mut(&mut self) -> &mut Automaton {
        &mut self.auto
    }
}
