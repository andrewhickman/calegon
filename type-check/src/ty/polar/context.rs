use std::collections::HashSet;
use std::hash::BuildHasherDefault;

use seahash::SeaHasher;
use typed_arena::Arena;

use ty::polar::Ty;

pub struct Context<'c> {
    // TODO refwrapper
    cache: HashSet<&'c Ty<'c>, BuildHasherDefault<SeaHasher>>,
    arena: Arena<Ty<'c>>,
}

impl<'c> Context<'c> {
    pub fn intern(&'c mut self, ty: Ty<'c>) -> &'c Ty {
        unimplemented!()
    }
}
