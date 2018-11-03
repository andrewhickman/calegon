use std::fmt::{self, Write};

use ast;

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

impl fmt::Display for ast::File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for item in &self.items {
            writeln!(f, "{};", item);
        }
        Ok(())
    }
}

impl fmt::Display for ast::Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ast::Item::Sys(sys) => sys.fmt(f),
            ast::Item::Comp(comp) => comp.fmt(f),
            ast::Item::TyDef(ty_def) => ty_def.fmt(f),
        }
    }
}

impl fmt::Display for ast::Sys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "sys {} {{", self.name)?;
        for stmt in &self.stmts {
            writeln!(Indented(&mut *f, true), "{};", stmt)?;
        }
        write!(f, "}}")
    }
}

impl fmt::Display for ast::Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ast::Stmt::Item(item) => item.fmt(f),
            ast::Stmt::Read(read) => read.fmt(f),
            ast::Stmt::Write(write) => write.fmt(f),
            ast::Stmt::Expr(expr) => expr.fmt(f),
            ast::Stmt::Binding(binding) => binding.fmt(f),
        }
    }
}

impl fmt::Display for ast::Read {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "read")?;
        let mut comps = self.comps.iter();
        if let Some(first) = comps.next() {
            write!(f, " {}", first)?;
            for comp in comps {
                write!(f, ", {}", comp)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for ast::Write {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "write")?;
        let mut comps = self.comps.iter();
        if let Some(first) = comps.next() {
            write!(f, " {}", first)?;
            for comp in comps {
                write!(f, ", {}", comp)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for ast::Comp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "comp {} {}", self.name, self.ty)
    }
}

impl fmt::Display for ast::TyDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type {} {}", self.name, self.ty)
    }
}

impl fmt::Display for ast::Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ast::Ty::Never => "never".fmt(f),
            ast::Ty::Unit => "unit".fmt(f),
            ast::Ty::I32 => "i32".fmt(f),
            ast::Ty::TyDef(symbol) => symbol.fmt(f),
            ast::Ty::Struct(s) => s.fmt(f),
            ast::Ty::Enum(e) => e.fmt(f),
        }
    }
}

impl fmt::Display for ast::Struct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "struct {{")?;
        for &(ref name, ref ty) in &self.fields {
            writeln!(Indented(&mut *f, true), "{}: {},", name, ty)?;
        }
        write!(f, "}}")
    }
}

impl fmt::Display for ast::Enum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "enum {{")?;
        for &(ref name, ref ty) in &self.fields {
            writeln!(Indented(&mut *f, true), "{}: {},", name, ty)?;
        }
        write!(f, "}}")
    }
}

impl fmt::Display for ast::Binding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "let {}", self.name)?;
        if let Some(ref ty) = self.ty {
            write!(f, ": {}", ty)?
        }
        if let Some(ref val) = self.val {
            write!(f, " = {}", val)?
        }
        Ok(())
    }
}

impl fmt::Display for ast::Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ast::Expr::FnCall(func, arg) => write!(f, "{} {}", func, arg),
            ast::Expr::Scope(stmts, tail) => {
                writeln!(f, "{{")?;
                for stmt in stmts {
                    writeln!(Indented(&mut *f, true), "{};", stmt)?;
                }
                if let Some(tail) = tail {
                    writeln!(Indented(&mut *f, true), "{}", tail)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl fmt::Display for ast::Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ast::Term::Literal(lit) => lit.fmt(f),
            ast::Term::Var(name) => name.fmt(f),
            ast::Term::Dot(expr, name) => write!(f, "{}.{}", expr, name),
            ast::Term::Struct(fields) => {
                writeln!(f, "{{")?;
                for &(ref name, ref ty) in fields {
                    writeln!(Indented(&mut *f, true), "{}: {},", name, ty)?;
                }
                write!(f, "}}")
            }
            ast::Term::Expr(expr) => expr.fmt(f),
        }
    }
}
