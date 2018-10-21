use im::HashSet;

use ty::automaton::state::{State, StateId};
use variance::Polarity;

pub(in ty::automaton) type FlowSet = HashSet<StateId>;

pub(in ty::automaton) fn populate(states: &mut [State]) {
    let mut map: Vec<(Vec<StateId>, Vec<StateId>)> = Vec::new();
    for (id, state) in states.iter_mut().enumerate() {
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
            states[from].flow.extend(pos.iter().cloned());
        }
        for &to in &pos {
            states[to].flow.extend(neg.iter().cloned());
        }
    }
}

pub(in ty::automaton) fn merge(states: &mut [State], q1: StateId, q2: StateId) {
    let flow1 = states[q1].flow.clone();
    let flow2 = states[q2].flow.clone();
    for &q in &flow2 {
        states[q].flow.insert(q1);
    }
    states[q1].flow = flow1.union(flow2);
}
