use std::usize;

/// De Bruijn index. Unbound variables grow downwards from usize::MAX.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Var(pub usize);

pub struct Context {
    unbound: usize,
    bound: usize,
}

impl Var {
    pub fn binding(self, binding_depth: usize) -> Option<usize> {
        binding_depth.checked_sub(self.0.saturating_add(1))
    }
}

impl Context {
    pub fn new() -> Self {
        Context {
            unbound: usize::MAX,
            bound: 0,
        }
    }

    pub fn push_bound(&mut self) -> Var {
        let var = Var(self.bound);
        self.bound += 1;
        var
    }

    pub fn pop_bound(&mut self) {
        self.bound -= 1;
    }

    pub fn unbound(&mut self) -> Var {
        let var = Var(self.unbound);
        self.unbound -= 1;
        var
    }
}

#[cfg(debug_assertions)]
impl Drop for Context {
    fn drop(&mut self) {
        debug_assert_eq!(self.bound, 0);
    }
}
