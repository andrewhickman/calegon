use std::collections::HashMap;
use std::mem::replace;

use itertools::Itertools;

use ty::automaton::state::transition::TransitionSet;
use ty::automaton::state::{State, StateId};
use ty::automaton::Automaton;

impl Automaton {
    pub fn reduce<A: AsRef<Self>>(&mut self, nfa_states: A, nfa_start: StateId) -> StateId {
        let nfa_states = nfa_states.as_ref();

        self.reserve(nfa_states.len());

        let dfa_start = self.add(nfa_states[nfa_start].to_dstate());
        let mut stack = vec![dfa_start];
        let mut cache = HashMap::new();
        cache.insert(vec![nfa_start], dfa_start);

        let mut children: Vec<StateId> = Vec::new();
        while let Some(id) = stack.pop() {
            let mut new_trans = TransitionSet::new();
            for (symbol, group) in &replace(self[id].transitions_mut(), TransitionSet::new())
                .get()
                .iter()
                .group_by(|tr| tr.symbol)
            {
                children.extend(group.map(|tr| tr.to));
                let id = if let Some(&id) = cache.get(&children) {
                    id
                } else {
                    let state = State::merged(children.iter().map(|&id| &nfa_states[id]));
                    let id = self.add(state);
                    cache.insert(children.clone(), id);
                    stack.push(id);
                    id
                };
                children.clear();

                new_trans.add(symbol, id);
            }
            replace(self[id].transitions_mut(), new_trans);
        }
        dfa_start
    }
}
