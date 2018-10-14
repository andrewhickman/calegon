use std::mem::replace;

use ty::automaton::state::constructor::Constructor;
use ty::automaton::state::{transition, State, StateId};
use ty::automaton::Automaton;
use ty::polar::{Ty, Visitor};
use ty::Fields;
use variance::{AsPolarity, Polarity};

struct BuildVisitor {
    states: Vec<State>,
    recs: Vec<StateId>,
    cur: StateId,
    cur_pol: Polarity,
}

impl BuildVisitor {
    fn new() -> Self {
        BuildVisitor {
            states: Vec::new(),
            recs: Vec::new(),
            cur: 0,
            cur_pol: Polarity::Pos,
        }
    }

    fn current(&mut self) -> &mut State {
        if self.cur == self.states.len() {
            self.states.push(State::new(self.cur_pol));
        }
        &mut self.states[self.cur]
    }

    fn visit<P: AsPolarity>(&mut self, ty: Ty<P>) -> StateId {
        let cur = replace(&mut self.cur, self.states.len());
        self.cur_pol = ty.polarity();
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
        self.current().add_constructor(Constructor::I32);
    }

    fn visit_fn<P: AsPolarity>(&mut self, pol: &P, domain: Ty<P::Neg>, range: Ty<P>) {
        assert_eq!(self.current().polarity(), pol.as_polarity());
        self.current().add_constructor(Constructor::Fn);

        let d = self.visit(domain);
        self.current().add_transition(transition::Symbol::Domain, d);

        let r = self.visit(range);
        self.current().add_transition(transition::Symbol::Range, r);
    }

    fn visit_struct<P: AsPolarity>(&mut self, pol: &P, fields: &Fields<Ty<P>>) {
        assert_eq!(self.current().polarity(), pol.as_polarity());
        self.current()
            .add_constructor(Constructor::Struct(fields.labels()));

        for &(label, ty) in fields.get() {
            let l = self.visit(ty);
            self.current()
                .add_transition(transition::Symbol::Label(label), l);
        }
    }

    fn visit_recursive<P: AsPolarity>(&mut self, pol: &P, ty: Ty<P>) {
        assert_eq!(self.current().polarity(), pol.as_polarity());
        self.recs.push(self.cur);
        ty.accept(self);
        self.recs.pop();
    }

    fn visit_var<P: AsPolarity>(&mut self, pol: &P, idx: i32) {
        //        assert_eq!(self.current().polarity(), pol.as_polarity());
        let idx = (self.recs.len() - 1) as i32 - idx;
        assert!(idx >= 0);
        let con = match self.recs.get(idx as usize) {
            Some(&id) => self.cur = id,
            None => self.current().add_constructor(Constructor::Var(idx)),
        };
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
        let mut builder = BuildVisitor::new();
        builder.visit(ty);
        builder.into()
    }
}
