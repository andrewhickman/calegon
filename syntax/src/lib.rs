extern crate bytecount;
#[macro_use]
extern crate lalrpop_util;
extern crate int_hash;
extern crate lazy_static;
extern crate memchr;
#[cfg(any(test, feature = "arbitrary"))]
#[macro_use]
extern crate proptest;
#[cfg(any(test, feature = "arbitrary"))]
extern crate proptest_recurse;
extern crate regex;
extern crate seahash;

pub mod ast;

#[cfg(any(test, feature = "arbitrary"))]
pub mod arbitrary;

mod error;
mod symbol;
lalrpop_mod!(parser);
#[cfg(test)]
mod tests;

pub use self::error::{Error, Location};
pub use self::symbol::{Symbol, SymbolMap};

use std::str::FromStr;

impl FromStr for ast::File {
    type Err = Error;

    fn from_str(input: &str) -> Result<ast::File, Self::Err> {
        use lazy_static::lazy_static;
        use parser::FileParser;
        use symbol::Interner;

        lazy_static! {
            static ref PARSER: FileParser = FileParser::new();
        }

        PARSER
            .parse(&mut Interner::write(), input)
            .map_err(|err| Error::new(input, err))
    }
}
