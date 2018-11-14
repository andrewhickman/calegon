use polar::{TyNeg, TyPos};

struct Constraint<'c>(&'c TyPos<'c>, &'c TyNeg<'c>);

impl<'c> Constraint<'c> {
    /// Immediate subconstraints
    fn immediate_subconstraints(&self) -> Option<Vec<Constraint>> {
        match *self {
            Constraint(Pos::Fn(tn1, tp1), Neg::Fn(tp2, tn2)) => {
                Some(vec![Constraint(*tp2, *tn1), Constraint(*tp1, *tn2)])
            }
            Constraint(Pos::I32, Neg::I32) => Some(vec![]),
            Constraint(Pos::Struct(left), Neg::Struct(right)) if left <= right => Some(
                left.get()
                    .iter()
                    .zip(right.get())
                    .map(|(&(_, ref l), &(_, ref r))| Constraint(l, r))
                    .collect(),
            ),
            Constraint(Pos::Recursive(ty), rhs) => {
                Some(vec![{ Constraint(ty.substitute(Pos::Recursive(ty)), rhs) }])
            }
            Constraint(lhs, Neg::Recursive(ty)) => {
                Some(vec![Constraint(lhs, ty.substitute(Neg::Recursive(ty)))])
            }
            Constraint(Pos::Join(tp1, tp2), tn) => {
                Some(vec![Constraint(tp1, tn), Constraint(tp2, tn)])
            }
            Constraint(tp, Neg::Meet(tn1, tn2)) => {
                Some(vec![Constraint(tp, tn1), Constraint(tp, tn2)])
            }
            Constraint(Pos::Never, _) => Some(vec![]),
            Constraint(_, Neg::Unit) => Some(vec![]),
            _ => None,
        }
    }
}
