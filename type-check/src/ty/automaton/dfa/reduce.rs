use std::collections::HashMap;
use std::mem::replace;

use itertools::Itertools;

use ty::automaton::state::transition::TransitionSet;
use ty::automaton::state::{flow, State, StateId};
use ty::automaton::{dfa, nfa};

impl nfa::Ty {
    pub fn reduce(&self) -> dfa::Ty {
        let mut states: Vec<State> = vec![self.start().clone()];
        let mut stack: Vec<StateId> = vec![0];
        let mut cache: HashMap<Vec<StateId>, StateId> = HashMap::new();
        cache.insert(vec![self.start_id()], 0);
        let mut children: Vec<StateId> = Vec::new();
        while let Some(id) = stack.pop() {
            let mut new_trans = TransitionSet::new();
            for (symbol, group) in &replace(&mut states[id].trans, TransitionSet::new())
                .get()
                .iter()
                .group_by(|tr| tr.symbol)
            {
                children.extend(group.map(|tr| tr.to));
                let id = if let Some(&id) = cache.get(&children) {
                    id
                } else {
                    let id = states.len();
                    cache.insert(children.clone(), id);
                    let state = State::merged(children.iter().map(|&id| &self.states()[id]));
                    states.push(state);
                    stack.push(id);
                    id
                };
                children.clear();

                new_trans.add(symbol, id);
            }
            replace(&mut states[id].trans, new_trans);
        }
        flow::populate(&mut states);
        dfa::Ty { states, start: 0 }
    }
}
