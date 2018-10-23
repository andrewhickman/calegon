#[cfg(test)]
mod tests;

#[cfg(test)]
pub use self::tests::*;

use ty::automaton::nfa::build::build;
use ty::automaton::state::StateId;
use ty::automaton::Automaton;
use ty::polar;
use variance::{Neg, Pos};

#[derive(Debug, Clone)]
pub struct Scheme {
    auto: Automaton,
    env: Vec<StateId>,
    expr: StateId,
}

impl Scheme {
    pub fn new<'c, I>(env: I, expr: polar::Ty<'c, Pos>) -> Self
    where
        I: IntoIterator<Item = polar::Ty<'c, Neg>>,
    {
        let mut auto = Automaton::new();
        let expr = build(&mut auto, expr);
        let env = env.into_iter().map(|ty| build(&mut auto, ty)).collect();
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
