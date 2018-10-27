use std::str::FromStr;

use lazy_static::lazy_static;

use ast::*;
use parser::*;
use symbol::Interner;
use {Error, Symbol};

impl FromStr for Symbol {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PARSER: SymbolParser = SymbolParser::new();
        }

        PARSER
            .parse(&mut Interner::write(), input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for File {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PARSER: FileParser = FileParser::new();
        }

        PARSER
            .parse(&mut Interner::write(), input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for Item {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PARSER: ItemParser = ItemParser::new();
        }

        PARSER
            .parse(&mut Interner::write(), input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for Sys {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PARSER: SysParser = SysParser::new();
        }

        PARSER
            .parse(&mut Interner::write(), input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for Stmt {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PARSER: StmtParser = StmtParser::new();
        }

        PARSER
            .parse(&mut Interner::write(), input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for Read {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PARSER: ReadParser = ReadParser::new();
        }

        PARSER
            .parse(&mut Interner::write(), input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for Write {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PARSER: WriteParser = WriteParser::new();
        }

        PARSER
            .parse(&mut Interner::write(), input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for Comp {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PARSER: CompParser = CompParser::new();
        }

        PARSER
            .parse(&mut Interner::write(), input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for TyDef {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PARSER: TyDefParser = TyDefParser::new();
        }

        PARSER
            .parse(&mut Interner::write(), input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for Ty {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PARSER: TyParser = TyParser::new();
        }

        PARSER
            .parse(&mut Interner::write(), input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for Struct {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PARSER: StructParser = StructParser::new();
        }

        PARSER
            .parse(&mut Interner::write(), input)
            .map_err(|err| Error::new(input, err))
    }
}

impl FromStr for Enum {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PARSER: EnumParser = EnumParser::new();
        }

        PARSER
            .parse(&mut Interner::write(), input)
            .map_err(|err| Error::new(input, err))
    }
}
