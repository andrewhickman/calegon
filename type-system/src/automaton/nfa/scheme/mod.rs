#[cfg(test)]
mod tests;

#[cfg(test)]
pub use self::tests::*;

use automaton::state::StateId;
use automaton::Automaton;
use polar;
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
        let expr = auto.build(expr);
        let env = env.into_iter().map(|ty| auto.build(ty)).collect();
        Scheme::from_parts(auto, env, expr)
    }

    pub(crate) fn from_parts(mut auto: Automaton, env: Vec<StateId>, expr: StateId) -> Self {
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
