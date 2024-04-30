use std::{cmp::Ordering, fmt::Display};

use chumsky::{primitive::end, text::whitespace, Parser};

use crate::{err::ParseError, parsers::ver};

#[derive(Debug, PartialEq)]
pub enum Version {
    Common {
        major: u8,
        minor: u8,
        rev: u8,
        pre: Option<String>,
    },
    Latest,
}

impl Version {
    /// Compare to check if 2 versions are compatible according to semantic
    /// versioning standards. Returns `(bool, bool)` The first field is
    /// whether they are compatible. The second is whther it is unsure.
    /// It can be unsure if
    /// 1. Either of the fields is a [`Version::Latest`]
    /// 2. They contain a prerelease
    /// ```
    /// use semver::Version;
    ///
    /// let ver = Version::Common {major: 1, minor: 0, rev: 0, pre: None};
    /// assert_eq!(ver.compat_with(Version::Latest), (false, true));
    /// assert_eq!(ver.compat_with(Version::Common {major: 0, minor: 1, rev: 1, pre: None}), (false, false));
    /// let ver = Version::Common {major: 1, minor: 0, rev: 0, pre: Some("pre1".to_string())};
    /// assert_eq!(ver.compat_with(Version::Common {major: 0, minor: 1, rev: 1, pre: None}), (false, true));
    /// let ver = Version::Common {major: 1, minor: 0, rev: 0, pre: None};
    /// assert_eq!(ver.compat_with(Version::Common {major: 0, minor: 1, rev: 1, pre: None}), (false, false));
    /// ```
    pub fn compat_with(&self, other: Version) -> (bool, bool) {
        if let Version::Common {
            major: o_major,
            minor: o_minor,
            rev: _,
            pre: o_pre,
        } = other
        {
            if let Version::Common {
                major,
                minor,
                rev: _,
                pre,
            } = self
            {
                if *major != o_major || minor < &o_minor {
                    if pre.is_some() || o_pre.is_some() {
                        (false, true)
                    } else {
                        (false, false)
                    }
                } else {
                    (false, false)
                }
            } else {
                (false, true)
            }
        } else {
            (false, true)
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Version::Common {
            major,
            minor,
            rev,
            pre,
        } = self
        {
            if pre.is_some() {
                write!(
                    f,
                    "{}.{}.{}-{}",
                    major,
                    minor,
                    rev,
                    pre.as_ref()
                        .unwrap()
                )
            } else {
                write!(f, "{}.{}.{}", major, minor, rev)
            }
        } else {
            write!(f, "latest")
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

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Version::Common {
            major: o_major,
            minor: o_minor,
            rev: o_rev,
            pre: o_pre,
        } = other
        {
            if let Version::Common {
                major,
                minor,
                rev,
                pre,
            } = self
            {
                if major > o_major {
                    Some(Ordering::Greater)
                } else if major < o_major {
                    Some(Ordering::Less)
                } else {
                    if minor > o_minor {
                        Some(Ordering::Greater)
                    } else if minor < o_minor {
                        Some(Ordering::Less)
                    } else {
                        if rev > o_rev {
                            Some(Ordering::Greater)
                        } else if rev < o_rev {
                            Some(Ordering::Less)
                        } else {
                            if pre.is_some() && o_pre.is_some() {
                                if pre.as_ref().unwrap() == o_pre.as_ref().unwrap() {
                                    Some(Ordering::Equal)
                                } else {
                                    None
                                }
                            } else if pre.is_some() {
                                Some(Ordering::Less)
                            } else if o_pre.is_some() {
                                Some(Ordering::Greater)
                            } else {
                                Some(Ordering::Equal)
                            }
                        }
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
