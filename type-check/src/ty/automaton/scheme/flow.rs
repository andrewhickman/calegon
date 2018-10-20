use std::collections::BTreeSet;
use std::fmt;

use ty::automaton::state::{State, StateId};

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct Flow {
    from: StateId,
    to: StateId,
}

pub struct FlowSet {
    inner: BTreeSet<Flow>,
}

impl FlowSet {
    pub(in ty::automaton::scheme) fn new(states: &[State]) -> Self {
        unimplemented!()
    }
}

impl fmt::Debug for Flow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ⇝ {}", self.from, self.to)
    }
}
