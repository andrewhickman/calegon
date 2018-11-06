#[cfg(test)]
mod tests;

use syntax::ast::File;

use ty::automaton::nfa::Scheme;

pub fn resolve(file: &File) -> Result<Scheme, ()> {
    unimplemented!()
}
