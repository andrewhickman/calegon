mod display;
#[cfg(any(test, feature = "arbitrary"))]
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
    Expr(Expr),
    Binding(Binding),
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

#[derive(Debug)]
pub struct Binding {
    pub name: Symbol,
    pub ty: Option<Ty>,
    pub val: Option<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    Literal(i32),
    Var(Symbol),
    Struct(Vec<(Symbol, Expr)>),
    Dot(Box<Expr>, Symbol),

    FnCall(Box<Expr>, Box<Expr>),
    Scope(Vec<Stmt>, Option<Box<Expr>>),
}
