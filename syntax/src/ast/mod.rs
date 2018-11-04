pub mod bind;
pub mod expr;
pub mod file;
pub mod stmt;
pub mod ty;

pub use self::bind::Bind;
pub use self::expr::Expr;
pub use self::file::File;
pub use self::stmt::Stmt;
pub use self::ty::Ty;

use std::fmt::{self, Write};

use SymbolMap;

#[derive(Debug)]
pub struct Scope<B, T> {
    pub body: Vec<B>,
    pub tail: Option<T>,
}

#[derive(Debug)]
pub struct Tuple<T>(pub Vec<T>);

#[derive(Debug)]
pub struct Map<T>(pub SymbolMap<T>);

struct Indented<W>(W, bool);

impl<W> fmt::Write for Indented<W>
where
    W: fmt::Write,
{
    fn write_str(&mut self, mut s: &str) -> fmt::Result {
        while !s.is_empty() {
            if self.1 {
                self.0.write_str("    ")?;
            }

            let split = match s.find('\n') {
                Some(pos) => {
                    self.1 = true;
                    pos + 1
                }
                None => {
                    self.1 = false;
                    s.len()
                }
            };
            self.0.write_str(&s[..split])?;
            s = &s[split..];
        }

        Ok(())
    }
}

impl<B, T> fmt::Display for Scope<B, T>
where
    B: fmt::Display,
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "(")?;
        for x in &self.body {
            writeln!(Indented(&mut *f, true), "{};", x)?;
        }
        if let Some(x) = &self.tail {
            writeln!(Indented(&mut *f, true), "{}", x)?;
        }
        write!(f, ")")
    }
}

impl<T> fmt::Display for Tuple<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[")?;
        for x in &self.0 {
            writeln!(Indented(&mut *f, true), "{},", x)?;
        }
        write!(f, "]")
    }
}

impl<T> fmt::Display for Map<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{")?;
        for (k, v) in &self.0 {
            writeln!(Indented(&mut *f, true), "{}: {},", k, v)?;
        }
        write!(f, "}}")
    }
}
