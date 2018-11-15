use std::collections::HashSet;

use automaton::state::StateId;
use automaton::Automaton;
use variance::Polarity;

impl Automaton {
    pub fn biunify(
        &mut self,
        seen: &mut HashSet<(StateId, StateId)>,
        qp: StateId,
        qn: StateId,
    ) -> Result<(), ()> {
        debug_assert_eq!(self[qp].polarity(), Polarity::Pos);
        debug_assert_eq!(self[qn].polarity(), Polarity::Neg);

        if seen.insert((qp, qn)) {
            if !self[qp].constructors().lub_le_glb(self[qn].constructors()) {
                return Err(());
            }
            for to in self[qn].flow.iter() {
                self.merge(to, qp);
            }
            for from in self[qp].flow.iter() {
                self.merge(from, qn)
            }
            let (domn, restp) = self[qp].transitions().split_at_domain();
            let (domp, restn) = self[qn].transitions().split_at_domain();
            for &dn in &domn {
                for &dp in &domp {
                    self.biunify(seen, dp, dn)?;
                }
            }
            for &rp in &restp {
                for &rn in &restn {
                    self.biunify(seen, rp, rn)?;
                }
            }
        }
        Ok(())
    }

    pub fn merge(&mut self, q1: StateId, q2: StateId) {
        if q1 != q2 {
            debug_assert_eq!(self[q1].polarity(), self[q2].polarity());
            self.merge_flow(q1, q2);
            let (s1, s2) = index2(self, q1, q2);
            s1.merge(s2);
        }
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
