use std::fmt;

/// De Bruijn index.
#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Var(usize);

impl Var {
    pub fn new(idx: usize) -> Self {
        Var(idx)
    }

    pub fn binding(self, binding_count: usize) -> Option<usize> {
        binding_count.checked_sub(self.0 + 1)
    }
}

impl fmt::Debug for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const ALPHABET: [char; 24] = [
            'α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ',
            'ο', 'π', 'ρ', 'σ', 'τ', 'υ', 'φ', 'χ', 'ψ', 'ω',
        ];

        if let Some(chr) = ALPHABET.get(self.0) {
            write!(f, "{}", chr)
        } else {
            f.debug_tuple("Var").field(&self.0).finish()
        }
    }
}
