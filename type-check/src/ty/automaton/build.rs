use std::mem::replace;

use ty::automaton::state::constructor::Constructor;
use ty::automaton::state::{transition, State, StateId};
use ty::automaton::Automaton;
use ty::polar::{Ty, Visitor};
use ty::Fields;
use variance::AsPolarity;

#[derive(Default)]
struct BuildVisitor {
    states: Vec<State>,
    recs: Vec<(StateId, Vec<StateId>)>,
    cur: StateId,
}

impl BuildVisitor {
    fn current(&self) -> &State {
        &self.states[self.cur]
    }

    fn current_mut(&mut self) -> &mut State {
        &mut self.states[self.cur]
    }

    fn visit<P: AsPolarity>(&mut self, ty: Ty<P>) -> StateId {
        let cur = replace(&mut self.cur, self.states.len());
        self.states.push(State::new(ty.polarity()));
        ty.accept(self);
        replace(&mut self.cur, cur)
    }
}

impl Visitor for BuildVisitor {
    fn visit_add<P: AsPolarity>(&mut self, pol: &P, lhs: Ty<P>, rhs: Ty<P>) {
        assert_eq!(self.current().polarity(), pol.as_polarity());
        lhs.accept(self);
        rhs.accept(self);
    }

    fn visit_zero<P: AsPolarity>(&mut self, pol: &P) {
        assert_eq!(self.current().polarity(), pol.as_polarity());
    }

    fn visit_i32<P: AsPolarity>(&mut self, pol: &P) {
        assert_eq!(self.current().polarity(), pol.as_polarity());
        self.current_mut().add_constructor(Constructor::I32);
    }

    fn visit_fn<P: AsPolarity>(&mut self, pol: &P, domain: Ty<P::Neg>, range: Ty<P>) {
        assert_eq!(self.current().polarity(), pol.as_polarity());
        self.current_mut().add_constructor(Constructor::Fn);

        let d = self.visit(domain);
        self.current_mut()
            .add_transition(transition::Symbol::Domain, d);

        let r = self.visit(range);
        self.current_mut()
            .add_transition(transition::Symbol::Range, r);
    }

    fn visit_struct<P: AsPolarity>(&mut self, pol: &P, fields: &Fields<Ty<P>>) {
        assert_eq!(self.current().polarity(), pol.as_polarity());
        self.current_mut()
            .add_constructor(Constructor::Struct(fields.labels()));

        for &(label, ty) in fields.get() {
            let l = self.visit(ty);
            self.current_mut()
                .add_transition(transition::Symbol::Label(label), l);
        }
    }

    fn visit_recursive<P: AsPolarity>(&mut self, pol: &P, ty: Ty<P>) {
        assert_eq!(self.current().polarity(), pol.as_polarity());
        self.recs.push((self.cur, Vec::new()));
        ty.accept(self);
        for id in self.recs.pop().unwrap().1 {
            self.states[id] = self.states[id].combine(self.current())
        }
    }

    fn visit_var<P: AsPolarity>(&mut self, pol: &P, idx: u32) {
        assert_eq!(self.current().polarity(), pol.as_polarity());
        let con = match self.recs.len().checked_sub(idx as usize) {
            Some(idx) => {
                let (id, ref mut uses) = self.recs[idx];
                uses.push(self.cur);
                Constructor::BoundVar(id)
            }
            None => Constructor::UnboundVar(idx),
        };
        self.current_mut().add_constructor(con);
    }
}

impl From<BuildVisitor> for Automaton {
    fn from(builder: BuildVisitor) -> Self {
        Automaton {
            states: builder.states,
        }
    }
}

impl Automaton {
    pub fn new<P: AsPolarity>(ty: Ty<P>) -> Self {
        let mut builder = BuildVisitor::default();
        builder.visit(ty);
        builder.into()
    }
}
