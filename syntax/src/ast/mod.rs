mod fmt;

use std::str::FromStr;

use {parser, Error, Symbol};

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

impl FromStr for File {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parser::FileParser::new()
            .parse(input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for Item {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parser::ItemParser::new()
            .parse(input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for Sys {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parser::SysParser::new()
            .parse(input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for Stmt {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parser::StmtParser::new()
            .parse(input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for Read {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parser::ReadParser::new()
            .parse(input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for Write {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parser::WriteParser::new()
            .parse(input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for Comp {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parser::CompParser::new()
            .parse(input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for TyDef {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parser::TyDefParser::new()
            .parse(input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for Ty {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parser::TyParser::new()
            .parse(input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for Struct {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parser::StructParser::new()
            .parse(input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for Enum {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parser::EnumParser::new()
            .parse(input)
            .map_err(|err| Error::new(input, err))
    }
}
