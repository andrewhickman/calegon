// TODO make less horrible

use std::cmp::Ordering;
use std::fmt;

use variance::Polarity;
use {Fields, Var};

#[derive(Clone, Debug)]
pub(in automaton) enum Constructor {
    Fn,
    I32,
    Struct(Fields<()>),
    Var(Var),
}

#[derive(Default, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(in automaton) struct ConstructorSet {
    f: bool,
    i: bool,
    fields: Option<Fields<()>>,
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
            Constructor::Struct(fields) => self.add_fields(pol, fields),
        }
    }

    pub fn add_set(&mut self, pol: Polarity, other: &Self) {
        self.f |= other.f;
        self.i |= other.i;
        if let Some(ref fields) = other.fields {
            self.add_fields(pol, fields);
        }
        self.vars.extend(&other.vars);
        self.vars.sort();
        self.vars.dedup();
    }

    fn add_fields(&mut self, pol: Polarity, rhs: &Fields<()>) {
        if let Some(lhs) = self.fields.clone() {
            match pol {
                Polarity::Pos => self.fields = Some(lhs.intersection(rhs)),
                Polarity::Neg => self.fields = Some(lhs.union(rhs)),
            }
        } else {
            self.fields = Some(rhs.clone());
        }
    }

    pub fn vars(&self) -> &[Var] {
        &self.vars
    }

    pub fn lub_le_glb(&self, other: &Self) -> bool {
        iproduct!(self.get(), other.get()).all(|(l, r)| l <= r)
    }

    pub fn has(&self, con: &Constructor) -> bool {
        match con {
            Constructor::Fn => self.f,
            Constructor::I32 => self.i,
            Constructor::Var(var) => self.vars.binary_search(var).is_ok(),
            Constructor::Struct(fields) => self.fields.as_ref() == Some(fields),
        }
    }

    pub fn has_struct(&self) -> Option<Fields<()>> {
        self.fields.clone()
    }

    fn get(&self) -> Vec<Constructor> {
        let mut result = Vec::new();
        if self.f {
            result.push(Constructor::Fn);
        }
        if self.i {
            result.push(Constructor::I32);
        }
        if let Some(ref fields) = self.fields {
            result.push(Constructor::Struct(fields.clone()));
        }
        result.extend(self.vars.iter().cloned().map(Constructor::Var));
        result
    }
}

impl fmt::Debug for ConstructorSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_set().entries(&self.get()).finish()
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
