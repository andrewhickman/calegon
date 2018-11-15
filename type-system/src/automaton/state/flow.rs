use im::HashSet;

use int_hash::IntBuildHasher;

use automaton::state::StateId;
use automaton::Automaton;
use variance::Polarity;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(in automaton) struct FlowSet {
    inner: HashSet<StateId, IntBuildHasher>,
}

impl FlowSet {
    pub fn new() -> Self {
        FlowSet {
            inner: HashSet::default(),
        }
    }

    pub fn shift(&mut self, n: StateId) {
        for id in self.inner.iter_mut() {
            *id += n;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = StateId> {
        self.inner.clone().into_iter()
    }

    pub fn insert(&mut self, id: StateId) {
        self.inner.insert(id);
    }

    pub fn contains(&self, id: StateId) -> bool {
        self.inner.contains(&id)
    }

    pub fn remove(&mut self, id: StateId) {
        self.inner.remove(&id);
    }
}

impl Automaton {
    pub fn populate_flow(&mut self) {
        let mut map: Vec<(Vec<StateId>, Vec<StateId>)> = Vec::new();
        for (id, state) in self.iter_mut().enumerate() {
            for var in state.constructors().vars() {
                if map.len() <= var.0 {
                    map.resize(var.0 + 1, (Vec::new(), Vec::new()));
                }
                match state.polarity() {
                    Polarity::Neg => map[var.0].0.push(id),
                    Polarity::Pos => map[var.0].1.push(id),
                }
            }
        }

        for (neg, pos) in map {
            for &from in &neg {
                self[from].flow.inner.extend(pos.iter().cloned());
            }
            for &to in &pos {
                self[to].flow.inner.extend(neg.iter().cloned());
            }
        }
    }

    pub fn merge_flow(&mut self, q1: StateId, q2: StateId) {
        let flow1 = self[q1].flow.inner.clone();
        let flow2 = self[q2].flow.inner.clone();
        for &q in &flow2 {
            self[q].flow.inner.insert(q1);
        }
        self[q1].flow.inner = flow1.union(flow2);
    }
}
