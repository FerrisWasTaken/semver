use std::fmt::Display;

use chumsky::{
    prelude::*,
    text::whitespace,
};
use err::ParseError;

mod err;
mod parsers;

use parsers::*;

#[derive(Debug, PartialEq)]
struct Version {
    major: u8,
    minor: u8,
    rev: u8,
    pre: Option<String>,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(pre) = self.pre.as_ref() {
            write!(f, "{}.{}.{}-{}", self.major, self.minor, self.rev, pre)
        } else {
            write!(f, "{}.{}.{}", self.major, self.minor, self.rev)
        }
    }
}

impl<'a> TryFrom<&'a str> for Version {
    type Error = ParseError<'a, &'a str>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        ver()
            .then_ignore(whitespace().ignore_then(end()))
            .parse(s)
            .into_result()
            .map_err(|x| ParseError::InternalErr { errors: x })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Comparator {
    Gr,
    GrEq,
    Eq,
    Lt,
    LtEq,
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
        }
    }
}

#[derive(Debug, PartialEq)]
struct VersionReq {
    comparator: Vec<(Comparator, Version)>,
    name: Option<String>,
}

impl Display for VersionReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String;
        if self.comparator.len() == 1 {
            s = format!("{} {}", self.comparator[0].0, self.comparator[0].1);
        } else {
            s = self.comparator.iter().fold("".to_string(), |s, (c, ver)| {
                s + format!("{c} {ver},").as_str()
            });
        };
        if let Some(pkg) = self.name.as_ref() {
            write!(f, "{pkg} {}", s.trim_end_matches(","))
        } else {
            write!(f, "{}", s.trim_end_matches(","))
        }
    }
}

impl<'a> TryFrom<&'a str> for VersionReq {
    type Error = ParseError<'a, &'a str>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        ver_req()
            .then_ignore(end().padded())
            .parse(s)
            .into_result()
            .map_err(|e| ParseError::InternalErr { errors: e })
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn norm_test() {
        assert_eq!(
            Version::try_from("1.0.1-pre1").unwrap(),
            Version {
                major: 1,
                minor: 0,
                rev: 1,
                pre: Some("pre1".to_string())
            }
        )
    }

    #[test]
    fn disp_ver_test() {
        let ver = Version {
            major: 0,
            minor: 1,
            rev: 0,
            pre: Some("pre1".to_string()),
        };
        assert_eq!(format!("{ver}"), "0.1.0-pre1")
    }

    #[test]
    fn disp_ver_req_test() {
        let ver = VersionReq {
            comparator: vec![(
                Comparator::Eq,
                Version {
                    major: 0,
                    minor: 1,
                    rev: 0,
                    pre: Some("pre1".to_string()),
                },
            )],
            name: Some("rust".to_string()),
        };
        assert_eq!(format!("{ver}"), "rust = 0.1.0-pre1");
        let ver = VersionReq {
            comparator: vec![
                (
                    Comparator::Eq,
                    Version {
                        major: 0,
                        minor: 1,
                        rev: 0,
                        pre: Some("pre1".to_string()),
                    },
                ),
                (
                    Comparator::GrEq,
                    Version {
                        major: 1,
                        minor: 0,
                        rev: 1,
                        pre: None,
                    },
                ),
            ],
            name: Some("rust".to_string()),
        };
        assert_eq!(format!("{ver}"), "rust = 0.1.0-pre1,>= 1.0.1")
    }

    #[test]
    fn norm_req_test() {
        assert_eq!(
            VersionReq::try_from("= 0.1.0"),
            Ok(VersionReq {
                comparator: vec![(
                    Comparator::Eq,
                    Version {
                        major: 0,
                        minor: 1,
                        rev: 0,
                        pre: None
                    }
                )],
                name: None
            })
        );
        assert_eq!(
            VersionReq::try_from("rust = 0.1.1, > 0.1.0"),
            Ok(VersionReq {
                comparator: vec![
                    (
                        Comparator::Eq,
                        Version {
                            major: 0,
                            minor: 1,
                            rev: 1,
                            pre: None
                        }
                    ),
                    (
                        Comparator::Gr,
                        Version {
                            major: 0,
                            minor: 1,
                            rev: 0,
                            pre: None
                        }
                    )
                ],
                name: Some("rust".to_string())
            })
        )
    }
}
