use std::cmp::Ordering::*;
use std::collections::HashSet;

use ty::automaton::state::constructor::Constructor;
use ty::automaton::state::transition::Symbol;
use ty::automaton::state::StateId;
use ty::automaton::Automaton;
use variance::Polarity;

impl Automaton {
    pub fn subsume(
        &mut self,
        seen: &mut HashSet<(StateId, StateId)>,
        q1: StateId,
        q2: StateId,
    ) -> Result<(), ()> {
        debug_assert_eq!(self[q1].polarity(), self[q2].polarity());

        if seen.insert((q1, q2)) {
            if self[q1].constructors().has(&Constructor::I32)
                && !self[q2].constructors().has(&Constructor::I32)
            {
                return Err(());
            }

            if self[q1].constructors().has(&Constructor::Fn) {
                if self[q2].constructors().has(&Constructor::Fn) {
                    let d1 = self[q1].transitions().getd(Symbol::Domain);
                    let d2 = self[q2].transitions().getd(Symbol::Domain);
                    self.subsume(seen, d1, d2)?;
                    let r1 = self[q1].transitions().getd(Symbol::Range);
                    let r2 = self[q2].transitions().getd(Symbol::Range);
                    self.subsume(seen, r1, r2)?;
                } else {
                    return Err(());
                }
            }

            if let Some(l1) = self[q1].constructors().has_struct() {
                if let Some(l2) = self[q2].constructors().has_struct() {
                    match l2.cmp_labels(&l1) {
                        Some(Equal) | Some(Less) => {
                            for &(l, _) in l2.get() {
                                let l1 = self[q1].transitions().getd(Symbol::Label(l));
                                let l2 = self[q2].transitions().getd(Symbol::Label(l));
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
        debug_assert_eq!(self[qn].polarity(), Polarity::Neg);
        debug_assert_eq!(self[qp].polarity(), Polarity::Pos);

        if self[qn].flow.contains(&qp) {
            true
        } else {
            self[qn].flow.insert(qp);
            self[qp].flow.insert(qn);

            if self[qn].constructors().has(&Constructor::I32)
                && self[qp].constructors().has(&Constructor::I32)
            {
                return true;
            }

            if self[qn].constructors().has(&Constructor::Fn)
                && self[qp].constructors().has(&Constructor::Fn)
            {
                let dp = self[qn].transitions().getd(Symbol::Domain);
                let dn = self[qp].transitions().getd(Symbol::Domain);
                let rn = self[qn].transitions().getd(Symbol::Range);
                let rp = self[qp].transitions().getd(Symbol::Range);
                if self.admissible(dn, dp) && self.admissible(rn, rp) {
                    return true;
                }
            }

            if let (Some(ln), Some(lp)) = (
                self[qn].constructors().has_struct(),
                self[qp].constructors().has_struct(),
            ) {
                match lp.cmp_labels(&ln) {
                    Some(Less) | Some(Equal) => if lp.get().iter().all(|&(l, _)| {
                        let ln = self[qn].transitions().getd(Symbol::Label(l));
                        let lp = self[qp].transitions().getd(Symbol::Label(l));
                        self.admissible(ln, lp)
                    }) {
                        return true;
                    },
                    Some(Greater) | None => (),
                }
            }

            self[qn].flow.remove(&qp);
            self[qp].flow.remove(&qn);

            false
        }
    }
}
