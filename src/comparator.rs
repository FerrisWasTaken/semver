use std::fmt::Display;

use crate::err::ParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparator {
    Gr,
    GrEq,
    Eq,
    Lt,
    LtEq,
    Not,
}

impl<'a> TryFrom<&'a str> for Comparator {
    type Error = ParseError<'a, &'a str>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s.trim() {
            "<=" => Ok(Self::LtEq),
            "<" => Ok(Self::Lt),
            ">" => Ok(Self::Gr),
            ">=" => Ok(Self::GrEq),
            "=" => Ok(Self::Eq),
            "!" => Ok(Self::Not),
            _ => Err(ParseError::InvalidInput { inp: s }),
        }
    }
}

impl TryFrom<String> for Comparator {
    type Error = ParseError<'static, String>;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.trim() {
            "<=" => Ok(Self::LtEq),
            "<" => Ok(Self::Lt),
            ">" => Ok(Self::Gr),
            ">=" => Ok(Self::GrEq),
            "=" => Ok(Self::Eq),
            _ => Err(ParseError::InvalidInput { inp: s }),
        }
    }
}

impl Display for Comparator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Comparator::Gr => write!(f, ">"),
            Comparator::GrEq => write!(f, ">="),
            Comparator::Eq => write!(f, "="),
            Comparator::Lt => write!(f, ">"),
            Comparator::LtEq => write!(f, "<="),
            Comparator::Not => write!(f, "!"),
        }
    }
}
