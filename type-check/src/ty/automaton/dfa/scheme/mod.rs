#[cfg(test)]
mod tests;

use std::cmp::Ordering::*;
use std::collections::HashSet;

use ty::automaton::state::constructor::Constructor;
use ty::automaton::state::transition::Symbol;
use ty::automaton::state::{State, StateId};
use ty::automaton::{dfa, nfa};
use variance::Polarity;

#[derive(Debug, Clone)]
pub struct Scheme {
    states: Vec<State>,
    env: Vec<StateId>,
    expr: StateId,
}

impl Scheme {
    pub fn new(nfa: &nfa::Scheme) -> Self {
        let mut states = Vec::new();
        let expr = dfa::reduce(&mut states, nfa.states(), nfa.expr());
        let env = nfa
            .env()
            .iter()
            .map(|&id| dfa::reduce(&mut states, nfa.states(), id))
            .collect();
        Scheme { states, expr, env }
    }

    pub fn subsume(
        &mut self,
        seen: &mut HashSet<(StateId, StateId)>,
        q1: StateId,
        q2: StateId,
    ) -> Result<(), ()> {
        debug_assert_eq!(self.states[q1].polarity(), self.states[q2].polarity());

        if seen.insert((q1, q2)) {
            if self.states[q1].constructors().has(&Constructor::I32)
                && !self.states[q2].constructors().has(&Constructor::I32)
            {
                return Err(());
            }

            if self.states[q1].constructors().has(&Constructor::Fn) {
                if self.states[q2].constructors().has(&Constructor::Fn) {
                    let d1 = self.states[q1].transitions().getd(Symbol::Domain);
                    let d2 = self.states[q2].transitions().getd(Symbol::Domain);
                    self.subsume(seen, d1, d2)?;
                    let r1 = self.states[q1].transitions().getd(Symbol::Range);
                    let r2 = self.states[q2].transitions().getd(Symbol::Range);
                    self.subsume(seen, r1, r2)?;
                } else {
                    return Err(());
                }
            }

            if let Some(l1) = self.states[q1].constructors().has_struct() {
                if let Some(l2) = self.states[q2].constructors().has_struct() {
                    match l2.cmp_labels(&l1) {
                        Some(Equal) | Some(Less) => {
                            for &(l, _) in l2.get() {
                                let l1 = self.states[q1].transitions().getd(Symbol::Label(l));
                                let l2 = self.states[q2].transitions().getd(Symbol::Label(l));
                                self.subsume(seen, l1, l2)?;
                            }
                        }
                        None | Some(Greater) => return Err(()),
                    }
                } else {
                    return Err(());
                }
            }

            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn admissible(&mut self, qn: StateId, qp: StateId) -> bool {
        debug_assert_eq!(self.states[qn].polarity(), Polarity::Neg);
        debug_assert_eq!(self.states[qp].polarity(), Polarity::Pos);

        if self.states[qn].flow.contains(&qp) {
            true
        } else {
            self.states[qn].flow.insert(qp);
            self.states[qp].flow.insert(qn);

            if self.states[qn].constructors().has(&Constructor::I32)
                && self.states[qp].constructors().has(&Constructor::I32)
            {
                return true;
            }

            if self.states[qn].constructors().has(&Constructor::Fn)
                && self.states[qp].constructors().has(&Constructor::Fn)
            {
                let dp = self.states[qn].transitions().getd(Symbol::Domain);
                let dn = self.states[qp].transitions().getd(Symbol::Domain);
                let rn = self.states[qn].transitions().getd(Symbol::Range);
                let rp = self.states[qp].transitions().getd(Symbol::Range);
                if self.admissible(dn, dp) && self.admissible(rn, rp) {
                    return true;
                }
            }

            if let (Some(ln), Some(lp)) = (
                self.states[qn].constructors().has_struct(),
                self.states[qp].constructors().has_struct(),
            ) {
                match lp.cmp_labels(&ln) {
                    Some(Less) | Some(Equal) => if lp.get().iter().all(|&(l, _)| {
                        let ln = self.states[qn].transitions().getd(Symbol::Label(l));
                        let lp = self.states[qp].transitions().getd(Symbol::Label(l));
                        self.admissible(ln, lp)
                    }) {
                        return true;
                    },
                    Some(Greater) | None => (),
                }
            }

            self.states[qn].flow.remove(&qp);
            self.states[qp].flow.remove(&qn);

            false
        }
    }
}
