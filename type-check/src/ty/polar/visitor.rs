use ty::polar::{Ty, TyKind};
use ty::{Fields, Var};
use variance::AsPolarity;

pub trait Visitor {
    fn visit_add<P: AsPolarity>(&mut self, _: &P, lhs: Ty<P>, rhs: Ty<P>) {
        lhs.accept(self);
        rhs.accept(self);
    }

    fn visit_zero<P: AsPolarity>(&mut self, _: &P) {}

    fn visit_i32<P: AsPolarity>(&mut self, _: &P) {}

    fn visit_fn<P: AsPolarity>(&mut self, _: &P, domain: Ty<P::Neg>, range: Ty<P>) {
        domain.accept(self);
        range.accept(self);
    }

    fn visit_struct<P: AsPolarity>(&mut self, _: &P, fields: &Fields<Ty<P>>) {
        fields.accept(self)
    }

    fn visit_recursive<P: AsPolarity>(&mut self, _: &P, ty: Ty<P>) {
        ty.accept(self)
    }

    fn visit_var<P: AsPolarity>(&mut self, _: &P, _: Var) {}
}

impl<'c, P: AsPolarity + 'c> Ty<'c, P> {
    pub fn accept<V: Visitor + ?Sized>(&self, visitor: &mut V) {
        match *self.kind {
            TyKind::Add(lhs, rhs) => visitor.visit_add(&self.pol, lhs, rhs),
            TyKind::Zero => visitor.visit_zero(&self.pol),
            TyKind::I32 => visitor.visit_i32(&self.pol),
            TyKind::Fn(domain, range) => visitor.visit_fn(&self.pol, domain, range),
            TyKind::Struct(ref fields) => visitor.visit_struct(&self.pol, fields),
            TyKind::Recursive(ty) => visitor.visit_recursive(&self.pol, ty),
            TyKind::Var(var) => visitor.visit_var(&self.pol, var),
        }
    }
}
