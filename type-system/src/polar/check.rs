use std::mem::replace;

use polar::{self, Ty};
use variance::{AsPolarity, Polarity};
use {Fields, Var};

struct RecursiveVisitor {
    ok: bool,
}

struct Visitor {
    guarded: bool,
    pol: Polarity,
    ok: bool,
    idx: usize,
}

impl<'c, P: AsPolarity + 'c> Ty<'c, P> {
    /// Check occurrences of a variable inside a recursive type are underneath at least one
    /// Fn or Struct type, and have the same polarity.
    pub fn check(self) -> bool {
        let mut visitor = RecursiveVisitor { ok: true };
        self.accept(&mut visitor);
        visitor.ok
    }
}

impl polar::Visitor for RecursiveVisitor {
    fn visit_recursive<P: AsPolarity>(&mut self, pol: &P, ty: Ty<P>) {
        let mut visitor = Visitor {
            guarded: false,
            ok: true,
            idx: 0,
            pol: pol.as_polarity(),
        };
        ty.accept(&mut visitor);
        self.ok &= visitor.ok;

        ty.accept(self);
    }
}

impl polar::Visitor for Visitor {
    fn visit_fn<P: AsPolarity>(&mut self, _: &P, domain: Ty<P::Neg>, range: Ty<P>) {
        let guarded = replace(&mut self.guarded, true);
        domain.accept(self);
        range.accept(self);
        self.guarded = guarded;
    }

    fn visit_struct<P: AsPolarity>(&mut self, _: &P, fields: &Fields<Ty<P>>) {
        let guarded = replace(&mut self.guarded, true);
        fields.accept(self);
        self.guarded = guarded;
    }

    fn visit_recursive<P: AsPolarity>(&mut self, _: &P, ty: Ty<P>) {
        self.idx += 1;
        ty.accept(self);
        self.idx -= 1;
    }

    fn visit_var<P: AsPolarity>(&mut self, pol: &P, var: Var) {
        if var.0 == self.idx {
            self.ok &= self.guarded && self.pol == pol.as_polarity()
        }
    }
}
