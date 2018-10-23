use im::HashSet;

use ty::automaton::state::StateId;
use ty::automaton::Automaton;
use variance::Polarity;

pub(in ty::automaton) type FlowSet = HashSet<StateId>;

impl Automaton {
    pub fn populate_flow(&mut self) {
        let mut map: Vec<(Vec<StateId>, Vec<StateId>)> = Vec::new();
        for (id, state) in self.iter_mut().enumerate() {
            for var in state.take_vars() {
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
                self[from].flow.extend(pos.iter().cloned());
            }
            for &to in &pos {
                self[to].flow.extend(neg.iter().cloned());
            }
        }
    }

    pub fn merge_flow(&mut self, q1: StateId, q2: StateId) {
        let flow1 = self[q1].flow.clone();
        let flow2 = self[q2].flow.clone();
        for &q in &flow2 {
            self[q].flow.insert(q1);
        }
        self[q1].flow = flow1.union(flow2);
    }
}
