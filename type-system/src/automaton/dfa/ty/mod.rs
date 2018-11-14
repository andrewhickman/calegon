#[cfg(test)]
mod tests;

#[cfg(test)]
pub use self::tests::*;

use automaton::state::StateId;
use automaton::{nfa, Automaton};
use variance::AsPolarity;

#[derive(Debug, Clone)]
pub struct Ty<P> {
    auto: Automaton,
    start: StateId,
    pol: P,
}

impl<P: AsPolarity> Ty<P> {
    fn start(&self) -> StateId {
        self.start
    }

    pub fn new(nfa: &nfa::Ty<P>) -> Self {
        let mut auto = Automaton::new();
        let start = auto.reduce(&nfa, nfa.start());
        Ty {
            auto,
            start,
            pol: P::default(),
        }
    }
}

impl<P> AsRef<Automaton> for Ty<P> {
    fn as_ref(&self) -> &Automaton {
        &self.auto
    }
}

impl<P> AsMut<Automaton> for Ty<P> {
    fn as_mut(&mut self) -> &mut Automaton {
        &mut self.auto
    }
}
