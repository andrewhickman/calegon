mod biunify;
#[cfg(test)]
mod tests;

#[cfg(test)]
pub use self::tests::*;

use ty::automaton::nfa::build::build;
use ty::automaton::state::StateId;
use ty::automaton::state::{flow, State};
use ty::polar;
use variance::{Neg, Pos};

#[derive(Debug)]
pub struct Scheme {
    states: Vec<State>,
    env: Vec<StateId>,
    expr: StateId,
}

impl Scheme {
    pub fn new<'c, I>(env: I, expr: polar::Ty<'c, Pos>) -> Self
    where
        I: IntoIterator<Item = polar::Ty<'c, Neg>>,
    {
        let mut states = Vec::new();
        let expr = build(&mut states, expr);
        let env = env.into_iter().map(|ty| build(&mut states, ty)).collect();
        flow::populate(&mut states);
        Scheme { states, expr, env }
    }

    pub fn merge(&mut self, q1: StateId, q2: StateId) {
        if q1 != q2 {
            debug_assert_eq!(self.states[q1].polarity(), self.states[q2].polarity());
            flow::merge(&mut self.states, q1, q2);
            let (s1, s2) = index2(&mut self.states, q1, q2);
            s1.merge(s2);
        }
    }

    pub(in ty::automaton) fn states(&self) -> &[State] {
        &self.states
    }

    pub fn expr(&self) -> StateId {
        self.expr
    }

    pub fn env(&self) -> &[StateId] {
        &self.env
    }
}

fn index2<T>(slice: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    if i < j {
        let (l, r) = slice.split_at_mut(j);
        (&mut l[i], &mut r[0])
    } else {
        let (l, r) = slice.split_at_mut(i);
        (&mut r[0], &mut l[j])
    }
}