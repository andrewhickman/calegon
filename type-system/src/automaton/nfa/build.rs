use std::mem::replace;

use automaton::state::constructor::Constructor;
use automaton::state::State;
use automaton::state::{transition, StateId};
use automaton::Automaton;
use variance::AsPolarity;
use {polar, Fields, Var};

pub(in automaton) fn build<P: AsPolarity>(states: &mut Automaton, ty: polar::Ty<P>) -> StateId {
    BuildVisitor::new(states).visit(ty)
}

struct BuildVisitor<'a> {
    states: &'a mut Automaton,
    recs: Vec<StateId>,
    cur: StateId,
}

impl<'a> BuildVisitor<'a> {
    fn new(states: &'a mut Automaton) -> Self {
        BuildVisitor {
            states,
            recs: Vec::new(),
            cur: 0,
        }
    }

    fn current(&mut self) -> &mut State {
        &mut self.states[self.cur]
    }

    fn set_state<P: AsPolarity>(&mut self, pol: &P) {
        if self.cur == self.states.len() {
            self.states.add(State::new(pol.as_polarity()));
        } else {
            debug_assert_eq!(self.current().polarity(), pol.as_polarity());
        }
    }

    fn visit<P: AsPolarity>(&mut self, ty: polar::Ty<P>) -> StateId {
        let cur = replace(&mut self.cur, self.states.len());
        ty.accept(self);
        debug_assert_ne!(self.cur, self.states.len());
        replace(&mut self.cur, cur)
    }
}

impl<'a> polar::Visitor for BuildVisitor<'a> {
    fn visit_add<P: AsPolarity>(&mut self, pol: &P, lhs: polar::Ty<P>, rhs: polar::Ty<P>) {
        self.set_state(pol);
        lhs.accept(self);
        rhs.accept(self);
    }

    fn visit_zero<P: AsPolarity>(&mut self, pol: &P) {
        self.set_state(pol);
    }

    fn visit_i32<P: AsPolarity>(&mut self, pol: &P) {
        self.set_state(pol);
        self.current().add_constructor(&Constructor::I32);
    }

    fn visit_fn<P: AsPolarity>(&mut self, pol: &P, domain: polar::Ty<P::Neg>, range: polar::Ty<P>) {
        self.set_state(pol);
        self.current().add_constructor(&Constructor::Fn);

        let d = self.visit(domain);
        self.current().add_transition(transition::Symbol::Domain, d);

        let r = self.visit(range);
        self.current().add_transition(transition::Symbol::Range, r);
    }

    fn visit_struct<P: AsPolarity>(&mut self, pol: &P, fields: &Fields<polar::Ty<P>>) {
        self.set_state(pol);
        self.current()
            .add_constructor(&Constructor::Struct(fields.labels()));

        for &(label, ty) in fields.get() {
            let l = self.visit(ty);
            self.current()
                .add_transition(transition::Symbol::Label(label), l);
        }
    }

    fn visit_recursive<P: AsPolarity>(&mut self, pol: &P, ty: polar::Ty<P>) {
        self.set_state(pol);
        self.recs.push(self.cur);
        ty.accept(self);
        self.recs.pop();
    }

    fn visit_var<P: AsPolarity>(&mut self, pol: &P, var: Var) {
        if let Some(binding) = var.binding(self.recs.len()) {
            self.cur = self.recs[binding];
            debug_assert_eq!(self.current().polarity(), pol.as_polarity());
        } else {
            self.set_state(pol);
            self.current().add_constructor(&Constructor::Var(var));
        }
    }
}
