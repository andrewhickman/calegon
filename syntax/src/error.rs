use std::{error, fmt};

use bytecount::count;
use memchr::memrchr;

use parser::Token;

pub(crate) type ParseError<'a> = ::lalrpop_util::ParseError<usize, Token<'a>, (usize, String)>;

#[derive(Clone, Debug)]
pub struct Error {
    location: Location,
    message: String,
}

pub(crate) fn error<'a>(location: usize, message: impl ToString) -> ParseError<'a> {
    ::lalrpop_util::ParseError::User {
        error: (location, message.to_string()),
    }
}

impl Error {
    pub(crate) fn new(input: &str, err: ParseError) -> Self {
        use lalrpop_util::ParseError::*;

        let (location, message) = match err {
            InvalidToken { location } => {
                (Location::new(input, location), "invalid token".to_owned())
            }
            UnrecognizedToken {
                token: Some((location, token, _)),
                ..
            } => (
                Location::new(input, location),
                format!("unexpected token '{}'", token),
            ),
            UnrecognizedToken { token: None, .. } => (
                Location::new(input, input.len()),
                "unexpected EOF".to_owned(),
            ),
            ExtraToken {
                token: (location, token, _),
            } => (
                Location::new(input, location),
                format!("extra token '{}' found", token),
            ),
            User {
                error: (location, message),
            } => (Location::new(input, location), message),
        };

        Error { location, message }
    }

    pub fn location(&self) -> Location {
        self.location
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.message.fmt(f)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Location {
    pub row: usize,
    pub col: usize,
}

impl Location {
    fn new(input: &str, idx: usize) -> Self {
        let input = input[..idx].as_bytes();
        let row = count(input, b'\n');
        let col = match memrchr(b'\n', input) {
            Some(start) => idx - 1 - start,
            None => idx,
        };
        Location { row, col }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.row + 1, self.col + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn location() {
        assert_eq!(Location::new("\n", 0), Location { row: 0, col: 0 });
        assert_eq!(Location::new("\n\n", 1), Location { row: 1, col: 0 });

        fn test(input: &str, row: usize, col: usize) {
            let idx = input.find('#').unwrap();
            assert_eq!(Location::new(input, idx), Location { row, col });
        }

        test("\n#\n", 1, 0);
        test("he#lo\n", 0, 2);
        test("hello\nwo#ld\n", 1, 2);
        test("hell\n\nwo#ld\n", 2, 2);
        test("hello\nw\n#ld\n", 2, 0);
    }
}
