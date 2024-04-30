use std::fmt::Display;

use chumsky::{primitive::end, Parser};

use crate::{comparator::Comparator, err::ParseError, parsers::ver_req, version::Version};

#[derive(Debug, PartialEq)]
pub struct VersionReq {
    pub comparator: Vec<(Comparator, Version)>,
    pub name: Option<String>,
}

impl Display for VersionReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String;
        if self
            .comparator
            .len()
            == 1
        {
            s = format!("{} {}", self.comparator[0].0, self.comparator[0].1);
        } else {
            s = self
                .comparator
                .iter()
                .fold("".to_string(), |s, (c, ver)| {
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

impl VersionReq {
    pub fn match_ver(&self, ver: &Version) -> bool {
        let mut matches = false;
        for (comp, req) in self
            .comparator
            .iter()
        {
            match *comp {
                Comparator::Gr => {
                    if ver > req {
                        matches = true;
                    } else {
                        return false;
                    }
                }
                Comparator::GrEq => {
                    if ver >= req {
                        matches = true;
                    } else {
                        return false;
                    }
                }
                Comparator::Eq => {
                    if req == ver {
                        matches = true;
                    } else {
                        return false;
                    }
                }
                Comparator::Lt => {
                    if ver < req {
                        matches = true;
                    } else {
                        return false;
                    }
                }
                Comparator::LtEq => {
                    if ver <= req {
                        matches = true;
                    } else {
                        return false;
                    }
                }
                Comparator::Not => {
                    if ver != req {
                        matches = true;
                    } else {
                        return false;
                    }
                },
            }
            if !matches {
                return false;
            }
        }
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_test() {
        let ver_req = VersionReq::try_from("= 0.1.0").unwrap();
        assert!(ver_req.match_ver(&Version::try_from("0.1.0").unwrap()));
        let ver_req = VersionReq::try_from("< 0.1.0").unwrap();
        assert!(ver_req.match_ver(&Version::try_from("0.0.1").unwrap()));
        let ver_req = VersionReq::try_from("> 0.1.0").unwrap();
        assert!(ver_req.match_ver(&Version::try_from("1.0.0").unwrap()));
        let ver_req = VersionReq::try_from(">= 0.1.0").unwrap();
        assert!(ver_req.match_ver(&Version::try_from("0.1.0").unwrap()));
        let ver_req = VersionReq::try_from("<= 0.1.0").unwrap();
        assert!(ver_req.match_ver(&Version::try_from("0.1.0").unwrap()));
        let ver_req = VersionReq::try_from("> 0.1.0, < 0.1.2").unwrap();
        assert!(ver_req.match_ver(&Version::try_from("0.1.1").unwrap()));
        let ver_req = VersionReq::try_from("!0.1.1").unwrap();
        assert!(ver_req.match_ver(&Version::try_from("0.1.0").unwrap()));
        let ver_req = VersionReq::try_from("rust > 0.1.0, < 0.1.2").unwrap();
        assert!(ver_req.match_ver(&Version::try_from("0.1.1").unwrap()));
    }
}
