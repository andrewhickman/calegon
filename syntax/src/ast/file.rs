use std::fmt;

use ast::Stmt;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct File {
    pub stmts: Vec<Stmt>,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for stmt in &self.stmts {
            writeln!(f, "{};", stmt)?
        }
        Ok(())
    }
}
