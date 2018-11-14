#[cfg(test)]
mod tests;

#[cfg(test)]
pub use self::tests::*;

use automaton::nfa::build;
use automaton::state::StateId;
use automaton::Automaton;
use polar;
use variance::AsPolarity;

#[derive(Debug, Clone)]
pub struct Ty<P> {
    auto: Automaton,
    start: StateId,
    pol: P,
}

impl<P: AsPolarity> Ty<P> {
    pub(in automaton) fn start(&self) -> StateId {
        self.start
    }

    pub fn new(ty: polar::Ty<P>) -> Self {
        let mut auto = Automaton::new();
        let start = build(&mut auto, ty);
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
