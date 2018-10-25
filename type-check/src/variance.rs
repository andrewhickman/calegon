use std::{fmt, ops};

pub trait AsPolarity: Sized + fmt::Display + fmt::Debug + Copy + Default {
    type Neg: AsPolarity<Neg = Self>;

    fn as_polarity(&self) -> Polarity;
}

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

impl fmt::Display for Polarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Polarity::Neg => "-",
            Polarity::Pos => "+",
        }.fmt(f)
    }
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct Pos;

impl AsPolarity for Pos {
    type Neg = Neg;

    fn as_polarity(&self) -> Polarity {
        Polarity::Pos
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_polarity().fmt(f)
    }
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct Neg;

impl AsPolarity for Neg {
    type Neg = Pos;

    fn as_polarity(&self) -> Polarity {
        Polarity::Neg
    }
}

impl fmt::Display for Neg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_polarity().fmt(f)
    }
}
