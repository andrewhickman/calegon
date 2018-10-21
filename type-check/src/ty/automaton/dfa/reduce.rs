use std::collections::HashMap;
use std::mem::replace;

use itertools::Itertools;

use ty::automaton::state::transition::TransitionSet;
use ty::automaton::state::{State, StateId};

pub(in ty::automaton) fn reduce(
    dfa_states: &mut Vec<State>,
    nfa_states: &[State],
    nfa_start: StateId,
) -> StateId {
    dfa_states.reserve(nfa_states.len());

    let dfa_start = dfa_states.len();
    let mut stack = vec![dfa_start];
    dfa_states.push(nfa_states[nfa_start].clone());
    let mut cache = HashMap::new();
    cache.insert(vec![nfa_start], dfa_start);

    let mut children: Vec<StateId> = Vec::new();
    while let Some(id) = stack.pop() {
        let mut new_trans = TransitionSet::new();
        for (symbol, group) in &replace(&mut dfa_states[id].trans, TransitionSet::new())
            .get()
            .iter()
            .group_by(|tr| tr.symbol)
        {
            children.extend(group.map(|tr| tr.to));
            let id = if let Some(&id) = cache.get(&children) {
                id
            } else {
                let id = dfa_states.len();
                cache.insert(children.clone(), id);
                let state = State::merged(children.iter().map(|&id| &nfa_states[id]));
                dfa_states.push(state);
                stack.push(id);
                id
            };
            children.clear();

            new_trans.add(symbol, id);
        }
        replace(&mut dfa_states[id].trans, new_trans);
    }
    dfa_start
}
