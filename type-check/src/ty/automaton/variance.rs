use std::ops;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Polarity {
    Neg = -1,
    Pos = 1,
}

impl ops::Neg for Polarity {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Polarity::Neg => Polarity::Pos,
            Polarity::Pos => Polarity::Neg,
        }
    }
}

impl ops::Mul for Polarity {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match rhs {
            Polarity::Neg => -self,
            Polarity::Pos => self,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Variant<N, P> {
    Neg(N),
    Pos(P),
}

impl<N, P> Variant<N, P> {
    pub fn polarity(&self) -> Polarity {
        match self {
            Variant::Neg(_) => Polarity::Neg,
            Variant::Pos(_) => Polarity::Pos,
        }
    }
}
