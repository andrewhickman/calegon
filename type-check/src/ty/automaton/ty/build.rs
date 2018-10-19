use std::mem::replace;

use ty::automaton::state::constructor::Constructor;
use ty::automaton::state::{transition, State, StateId};
use ty::automaton::Ty;
use ty::{polar, Fields, Var};
use variance::{AsPolarity, Polarity};

struct BuildVisitor {
    states: Vec<State>,
    recs: Vec<StateId>,
    cur: StateId,
    cur_pol: Polarity,
}

impl BuildVisitor {
    fn new(cur_pol: Polarity) -> Self {
        BuildVisitor {
            states: Vec::new(),
            recs: Vec::new(),
            cur: 0,
            cur_pol,
        }
    }

    fn current(&mut self) -> &mut State {
        &mut self.states[self.cur]
    }

    fn merge_state<P: AsPolarity>(&mut self, pol: &P) {
        if self.cur == self.states.len() {
            self.states.push(State::new(pol.as_polarity()));
        } else {
            debug_assert_eq!(self.current().polarity(), pol.as_polarity());
        }
    }

    fn visit<P: AsPolarity>(&mut self, ty: polar::Ty<P>) -> StateId {
        let cur = replace(&mut self.cur, self.states.len());
        self.cur_pol = ty.polarity();
        ty.accept(self);
        debug_assert!(self.cur != self.states.len());
        replace(&mut self.cur, cur)
    }
}

impl polar::Visitor for BuildVisitor {
    fn visit_add<P: AsPolarity>(&mut self, pol: &P, lhs: polar::Ty<P>, rhs: polar::Ty<P>) {
        self.merge_state(pol);
        lhs.accept(self);
        rhs.accept(self);
    }

    fn visit_zero<P: AsPolarity>(&mut self, pol: &P) {
        self.merge_state(pol);
    }

    fn visit_i32<P: AsPolarity>(&mut self, pol: &P) {
        self.merge_state(pol);
        self.current().add_constructor(Constructor::I32);
    }

    fn visit_fn<P: AsPolarity>(&mut self, pol: &P, domain: polar::Ty<P::Neg>, range: polar::Ty<P>) {
        self.merge_state(pol);
        self.current().add_constructor(Constructor::Fn);

        let d = self.visit(domain);
        self.current().add_transition(transition::Symbol::Domain, d);

        let r = self.visit(range);
        self.current().add_transition(transition::Symbol::Range, r);
    }

    fn visit_struct<P: AsPolarity>(&mut self, pol: &P, fields: &Fields<polar::Ty<P>>) {
        self.merge_state(pol);
        self.current()
            .add_constructor(Constructor::Struct(fields.labels()));

        for &(label, ty) in fields.get() {
            let l = self.visit(ty);
            self.current()
                .add_transition(transition::Symbol::Label(label), l);
        }
    }

    fn visit_recursive<P: AsPolarity>(&mut self, pol: &P, ty: polar::Ty<P>) {
        self.merge_state(pol);
        self.recs.push(self.cur);
        ty.accept(self);
        self.recs.pop();
    }

    fn visit_var<P: AsPolarity>(&mut self, pol: &P, var: Var) {
        if let Some(binding) = var.binding(self.recs.len()) {
            self.cur = self.recs[binding];
            debug_assert_eq!(self.current().polarity(), pol.as_polarity());
        } else {
            self.merge_state(pol);
            self.current().add_constructor(Constructor::Var(var));
        }
    }
}

impl From<BuildVisitor> for Ty {
    fn from(builder: BuildVisitor) -> Self {
        Ty {
            states: builder.states,
        }
    }
}

impl Ty {
    pub fn new<P: AsPolarity>(ty: polar::Ty<P>) -> Self {
        let mut builder = BuildVisitor::new(ty.polarity());
        builder.visit(ty);
        builder.into()
    }
}
