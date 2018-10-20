use std::cmp::Ordering;
use std::fmt;

use iter_set;

use ty::{Fields, Var};
use variance::Polarity;

#[derive(Clone, Debug)]
pub(in ty::automaton) enum Constructor {
    Fn,
    I32,
    Struct(Fields<()>),
    Var(Var),
}

#[derive(Default)]
pub(in ty::automaton::state) struct ConstructorSet {
    f: bool,
    i: bool,
    fields: Fields<()>,
    vars: Vec<Var>,
}

impl ConstructorSet {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, pol: Polarity, con: &Constructor) {
        match con {
            Constructor::Fn => self.f = true,
            Constructor::I32 => self.i = true,
            Constructor::Var(var) => if let Err(idx) = self.vars.binary_search(var) {
                self.vars.insert(idx, *var);
            },
            Constructor::Struct(fields) => match pol {
                Polarity::Pos => self.fields = self.fields.intersection(fields),
                Polarity::Neg => self.fields = self.fields.union(fields),
            },
        }
    }

    #[cfg(test)]
    pub fn has(&self, con: &Constructor) -> bool {
        match con {
            Constructor::Fn => self.f,
            Constructor::I32 => self.i,
            Constructor::Var(var) => self.vars.binary_search(var).is_ok(),
            Constructor::Struct(fields) => self.fields == *fields,
        }
    }
}

impl fmt::Debug for ConstructorSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut set = f.debug_set();
        if self.f {
            set.entry(&"⟨→⟩");
        }
        if self.i {
            set.entry(&"⟨i⟩");
        }
        set.entries(self.fields.get().iter().map(|&(l, _)| l));
        set.entries(&self.vars);
        set.finish()
    }
}

impl PartialEq for Constructor {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Constructor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use self::Constructor::*;

        match (self, other) {
            (Var(a), Var(b)) if a == b => Some(Ordering::Equal),
            (I32, I32) => Some(Ordering::Equal),
            (Fn, Fn) => Some(Ordering::Equal),
            (Struct(l), Struct(r)) => l.cmp_labels(r),
            _ => None,
        }
    }
}
