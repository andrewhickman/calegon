extern crate calegon_syntax as syntax;

mod fields;
mod name;

pub use self::fields::Fields;
pub use self::name::Name;

pub enum Ty {
    Pos(TyPos),
    Neg(TyNeg),
}

pub enum TyPos {
    Named(Name),
    Join(&'static TyPos, &'static TyPos),
    Never,
    I32,
    Fn(&'static TyNeg, &'static TyPos),
    Struct(Fields<TyPos>),
    Recursive(Name, &'static TyPos),
}

pub enum TyNeg {
    Named(Name),
    Meet(&'static TyNeg, &'static TyNeg),
    Unit,
    I32,
    Fn(&'static TyPos, &'static TyNeg),
    Struct(Fields<TyNeg>),
    Recursive(Name, &'static TyNeg),
}

impl TyPos {
    fn intern(self) -> &'static Self {
        unimplemented!()
    }

    fn substitute(&'static self, name: Name, ty: &'static TyPos) -> &'static TyPos {
        unimplemented!()
    }
}

impl TyNeg {
    fn intern(self) -> &'static Self {
        unimplemented!()
    }

    fn substitute(&'static self, name: Name, ty: &'static TyNeg) -> &'static TyNeg {
        unimplemented!()
    }
}

struct Constraint(&'static TyPos, &'static TyNeg);

impl Constraint {
    /// Immediate subconstraints
    fn subi(&self) -> Option<Vec<Constraint>> {
        match *self {
            Constraint(TyPos::Fn(tn1, tp1), TyNeg::Fn(tp2, tn2)) => {
                Some(vec![Constraint(*tp2, *tn1), Constraint(*tp1, *tn2)])
            }
            Constraint(TyPos::I32, TyNeg::I32) => Some(vec![]),
            Constraint(TyPos::Struct(left), TyNeg::Struct(right)) if left.contains(right) => Some(
                left.get()
                    .iter()
                    .zip(right.get())
                    .map(|(&(_, ref l), &(_, ref r))| Constraint(l, r))
                    .collect(),
            ),
            Constraint(TyPos::Recursive(name, ty), rhs) => Some(vec![{
                Constraint(
                    ty.substitute(*name, TyPos::Recursive(*name, ty).intern()),
                    rhs,
                )
            }]),
            Constraint(lhs, TyNeg::Recursive(name, ty)) => Some(vec![Constraint(
                lhs,
                ty.substitute(*name, TyNeg::Recursive(*name, ty).intern()),
            )]),
            Constraint(TyPos::Join(tp1, tp2), tn) => {
                Some(vec![Constraint(tp1, tn), Constraint(tp2, tn)])
            }
            Constraint(tp, TyNeg::Meet(tn1, tn2)) => {
                Some(vec![Constraint(tp, tn1), Constraint(tp, tn2)])
            }
            Constraint(TyPos::Never, _) => Some(vec![]),
            Constraint(_, TyNeg::Unit) => Some(vec![]),
            _ => None,
        }
    }
}
