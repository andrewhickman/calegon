mod display;
#[cfg(test)]
mod from_str;

use Symbol;

#[derive(Debug)]
pub struct File {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    Sys(Sys),
    Comp(Comp),
    TyDef(TyDef),
}

#[derive(Debug)]
pub struct Sys {
    pub name: Symbol,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Item(Item),
    Read(Read),
    Write(Write),
}

#[derive(Debug)]
pub struct Read {
    pub comps: Vec<Symbol>,
}

#[derive(Debug)]
pub struct Write {
    pub comps: Vec<Symbol>,
}

#[derive(Debug)]
pub struct Comp {
    pub name: Symbol,
    pub ty: Ty,
}

#[derive(Debug)]
pub struct TyDef {
    pub name: Symbol,
    pub ty: Ty,
}

#[derive(Debug)]
pub enum Ty {
    Never,
    Unit,
    I32,
    TyDef(Symbol),
    Struct(Struct),
    Enum(Enum),
}

#[derive(Debug)]
pub struct Struct {
    pub fields: Vec<(Symbol, Ty)>,
}

#[derive(Debug)]
pub struct Enum {
    pub fields: Vec<(Symbol, Ty)>,
}
