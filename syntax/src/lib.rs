extern crate bytecount;
#[macro_use]
extern crate lalrpop_util;
#[cfg(any(test, feature = "proptest"))]
#[macro_use]
extern crate proptest;
extern crate memchr;
extern crate regex;
extern crate seahash;

pub mod ast;

mod error;
mod symbol;
lalrpop_mod!(parser);
#[cfg(any(test, feature = "proptest"))]
mod arbitrary;
#[cfg(test)]
mod tests;

#[cfg(any(test, feature = "proptest"))]
pub use self::arbitrary::*;
pub use self::error::{Error, Location};
pub use self::symbol::Symbol;

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
            .parse(input)
            .map_err(|err| Error::new(input, err))
    }
}
