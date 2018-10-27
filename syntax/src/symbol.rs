use std::collections::HashMap;
use std::fmt;
use std::hash::BuildHasherDefault;
use std::sync::{RwLock, RwLockWriteGuard};

use lazy_static::lazy_static;
use seahash::SeaHasher;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Symbol(u32);

impl Symbol {
    #[cfg(test)]
    pub(crate) fn intern(string: &str) -> Self {
        INTERNER.write().unwrap().intern(string)
    }

    pub fn as_str(self) -> &'static str {
        // Should never need to block outside of tests
        INTERNER.read().unwrap().strings[self.0 as usize]
    }
}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Debug, Default)]
pub(crate) struct Interner {
    symbols: HashMap<&'static str, Symbol, BuildHasherDefault<SeaHasher>>,
    strings: Vec<&'static str>,
}

lazy_static! {
    static ref INTERNER: RwLock<Interner> = RwLock::default();
}

impl Interner {
    pub fn write<'a>() -> RwLockWriteGuard<'a, Self> {
        INTERNER.write().unwrap()
    }

    pub fn intern(&mut self, string: &str) -> Symbol {
        if let Some(&symbol) = self.symbols.get(string) {
            return symbol;
        }

        let symbol = Symbol(self.strings.len() as u32);
        let string = Box::leak(string.into());
        self.symbols.insert(string, symbol);
        self.strings.push(string);
        symbol
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    proptest! {
        #[test]
        fn roundtrip(string in "[[:alnum:]]+") {
            let symbol = Symbol::intern(&string);
            prop_assert_eq!(symbol.as_str(), &string);
            let symbol2 = Symbol::intern(&string);
            prop_assert_eq!(symbol, symbol2);
        }
    }

    #[test]
    fn parse_symbol() {
        assert!(Symbol::from_str("_").is_err());
        assert!(Symbol::from_str("_a").is_ok());
        assert!(Symbol::from_str("a").is_ok());
        assert!(Symbol::from_str("a_").is_err());
        assert!(Symbol::from_str("a__").is_err());
        assert!(Symbol::from_str("a__a").is_err());
        assert!(Symbol::from_str("a_a").is_ok());
    }
}
