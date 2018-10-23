#[cfg(test)]
mod tests;

#[cfg(test)]
pub use self::tests::*;

use ty::automaton::nfa::build;
use ty::automaton::state::StateId;
use ty::automaton::Automaton;
use ty::polar;
use variance::AsPolarity;

#[derive(Debug, Clone)]
pub struct Ty {
    auto: Automaton,
    start: StateId,
}

impl Ty {
    pub(in ty::automaton) fn start(&self) -> StateId {
        self.start
    }

    pub fn new<P: AsPolarity>(ty: polar::Ty<P>) -> Self {
        let mut auto = Automaton::new();
        let start = build(&mut auto, ty);
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
