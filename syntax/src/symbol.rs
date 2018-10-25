use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::hash::BuildHasherDefault;

use seahash::SeaHasher;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Symbol(u32);

impl Symbol {
    // Symbols should only be created on the main thread to ensure they are comparable.
    pub fn intern(string: &str) -> Self {
        Interner::with(|interner| interner.intern(string))
    }

    pub fn as_str(self) -> &'static str {
        Interner::with(|interner| interner.as_str(self))
    }
}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Interner::with(|interner| write!(f, "{}", interner.as_str(*self)))
    }
}

#[derive(Debug, Default)]
struct Interner {
    symbols: HashMap<&'static str, Symbol, BuildHasherDefault<SeaHasher>>,
    strings: Vec<&'static str>,
}

impl Interner {
    fn with<T, F: FnOnce(&mut Self) -> T>(f: F) -> T {
        thread_local! {
            static INTERNER: RefCell<Interner> = RefCell::default();
        }

        INTERNER.with(|interner| f(&mut *interner.borrow_mut()))
    }

    fn intern(&mut self, string: &str) -> Symbol {
        if let Some(&symbol) = self.symbols.get(string) {
            return symbol;
        }

        let symbol = Symbol(self.strings.len() as u32);
        let string = Box::leak(string.into());
        self.symbols.insert(string, symbol);
        self.strings.push(string);
        symbol
    }

    fn as_str(&self, symbol: Symbol) -> &'static str {
        &self.strings[symbol.0 as usize]
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
        assert!(Symbol::from_str("a_a").is_err());
    }
}
