use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use seahash::SeaHasher;

use ty::automaton::head;
use ty::automaton::FieldAlphabet;
use variance::Polarity;

pub type StateId = u32;

pub struct State {
    pol: Polarity,
    cons: head::ConstructorSet,
    trans: HashMap<FieldAlphabet, StateId, BuildHasherDefault<SeaHasher>>,
}
