/// De Bruijn index.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Var(pub usize);

impl Var {
    pub fn binding(self, binding_count: usize) -> Option<usize> {
        binding_count.checked_sub(self.0 + 1)
    }
}
