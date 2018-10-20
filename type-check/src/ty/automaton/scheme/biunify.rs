use std::collections::HashSet;

use ty::automaton::state::StateId;
use ty::automaton::Scheme;
use variance::Polarity;

impl Scheme {
    pub fn biunify(
        &mut self,
        seen: &mut HashSet<(StateId, StateId)>,
        qp: StateId,
        qn: StateId,
    ) -> Result<(), ()> {
        debug_assert_eq!(self.states[qp].polarity(), Polarity::Pos);
        debug_assert_eq!(self.states[qn].polarity(), Polarity::Neg);

        if seen.insert((qp, qn)) {
            if !self.states[qp]
                .constructors()
                .lub_le_glb(self.states[qn].constructors())
            {
                return Err(());
            }
            for to in self.states[qn].flow.clone() {
                self.merge(to, qp);
            }
            for from in self.states[qp].flow.clone() {
                self.merge(from, qn)
            }
            let (domn, restp) = self.states[qp].transitions().split_at_domain();
            let (domp, restn) = self.states[qn].transitions().split_at_domain();
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
}
