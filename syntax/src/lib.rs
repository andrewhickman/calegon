extern crate bytecount;
extern crate lalrpop_util;
#[cfg(any(test, feature = "arbitrary"))]
#[macro_use]
extern crate proptest;
extern crate lazy_static;
extern crate memchr;
#[cfg(any(test, feature = "arbitrary"))]
extern crate proptest_recurse;
extern crate regex;
extern crate seahash;

pub mod ast;

mod error;
mod symbol;
lalrpop_mod!(
    #[allow(dead_code, unused_imports)]
    parser
);
#[cfg(any(test, feature = "arbitrary"))]
mod arbitrary;
#[cfg(test)]
mod tests;

#[cfg(any(test, feature = "arbitrary"))]
pub use self::arbitrary::*;
pub use self::error::{Error, Location};
pub use self::symbol::Symbol;

use lalrpop_util::lalrpop_mod;
use symbol::Interner;

pub struct Parser {
    inner: parser::FileParser,
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            inner: parser::FileParser::new(),
        }
    }
}

impl Parser {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn parse(&self, input: &str) -> Result<ast::File, Error> {
        self.inner
            .parse(&mut Interner::write(), input)
            .map_err(|err| Error::new(input, err))
    }
}
