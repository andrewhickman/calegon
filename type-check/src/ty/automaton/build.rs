use ty::automaton::{Automaton, State, StateId};
use ty::polar::{Ty, TyNeg, TyPos, Visitor};
use ty::Fields;
use variance::Polarity;

struct BuildVisitor {
    states: Vec<State>,
    recs: Vec<StateId>,
}

impl Visitor for BuildVisitor {
    fn visit_var(&mut self, pol: Polarity, var: u32) {}

    fn visit_join(&mut self, lhs: &TyPos, rhs: &TyPos) {
        lhs.accept(self);
        rhs.accept(self);
    }

    fn visit_meet(&mut self, lhs: &TyNeg, rhs: &TyNeg) {
        lhs.accept(self);
        rhs.accept(self);
    }

    fn visit_never(&mut self) {}

    fn visit_unit(&mut self) {}

    fn visit_i32(&mut self, _: Polarity) {}

    fn visit_fn_pos(&mut self, domain: &TyNeg, range: &TyPos) {
        domain.accept(self);
        range.accept(self);
    }

    fn visit_fn_neg(&mut self, domain: &TyPos, range: &TyNeg) {
        domain.accept(self);
        range.accept(self);
    }

    fn visit_struct_pos(&mut self, fields: &Fields<TyPos>) {
        for &(_, ref ty) in fields.get() {
            ty.accept(self);
        }
    }

    fn visit_struct_neg(&mut self, fields: &Fields<TyNeg>) {
        for &(_, ref ty) in fields.get() {
            ty.accept(self);
        }
    }

    fn visit_recursive_pos(&mut self, ty: &TyPos) {
        ty.accept(self);
    }

    fn visit_recursive_neg(&mut self, ty: &TyNeg) {
        ty.accept(self);
    }
}

impl Automaton {
    pub fn new(ty: &Ty) -> Self {
        let mut visitor = BuildVisitor {
            recs: Vec::new(),
            states: Vec::new(),
        };

        ty.accept(&mut visitor);

        Automaton {
            states: visitor.states,
        }
    }
}
