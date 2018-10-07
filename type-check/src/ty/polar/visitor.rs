use ty::polar::{Ty, TyNeg, TyPos};
use ty::Fields;
use variance::Polarity;

pub trait Visitor {
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

impl<'c> Ty<'c> {
    pub fn accept<V: Visitor + ?Sized>(&self, visitor: &mut V) {
        match self {
            Ty::Pos(pos) => pos.accept(visitor),
            Ty::Neg(neg) => neg.accept(visitor),
        }
    }
}

impl<'c> TyPos<'c> {
    pub fn accept<V: Visitor + ?Sized>(&self, visitor: &mut V) {
        use ty::polar::TyPos::*;
        use variance::Polarity::Pos;

        match *self {
            Var(idx) => visitor.visit_var(Pos, idx),
            Join(lhs, rhs) => visitor.visit_join(lhs, rhs),
            Never => visitor.visit_never(),
            I32 => visitor.visit_i32(Pos),
            Fn(domain, range) => visitor.visit_fn_pos(domain, range),
            Struct(ref fields) => visitor.visit_struct_pos(fields),
            Recursive(ty) => visitor.visit_recursive_pos(ty),
        }
    }
}

impl<'c> TyNeg<'c> {
    pub fn accept<V: Visitor + ?Sized>(&self, visitor: &mut V) {
        use ty::polar::TyNeg::*;
        use variance::Polarity::Neg;

        match *self {
            Var(idx) => visitor.visit_var(Neg, idx),
            Meet(lhs, rhs) => visitor.visit_meet(lhs, rhs),
            Unit => visitor.visit_unit(),
            I32 => visitor.visit_i32(Neg),
            Fn(domain, range) => visitor.visit_fn_neg(domain, range),
            Struct(ref fields) => visitor.visit_struct_neg(fields),
            Recursive(ty) => visitor.visit_recursive_neg(ty),
        }
    }
}
