use chumsky::{
    error::Simple, extra, primitive::{any, choice, just}, text::{int, whitespace}, IterParser, Parser
};

use crate::{Version, VersionReq, Comparator};

pub(crate) fn ver<'a>() -> impl Parser<'a, &'a str, Version, extra::Err<Simple<'a, char>>> {
    let number = whitespace::<_, _, extra::Err<Simple<char>>>().ignore_then(
        int::<&str, _, chumsky::extra::Err<Simple<char>>>(10).map(|i| i.parse::<u8>().unwrap()),
    );
    let suffix = just("-")
        .ignore_then(
            any::<&str, extra::Err<Simple<char>>>()
                .filter(|x: &char| x.is_alphanumeric())
                .repeated()
                .collect::<String>(),
        )
        .or_not();

    number
        .then_ignore(just("."))
        .then(number)
        .then_ignore(just("."))
        .then(number)
        .then(suffix)
        .map(|(((major, minor), rev), pre)| Version {
            major,
            minor,
            rev,
            pre,
        })
}

pub(crate) fn ver_req<'a>() -> impl Parser<'a, &'a str, VersionReq, extra::Err<Simple<'a, char>>> {
    let pkg = any::<&'a str, extra::Err<Simple<char>>>()
        .filter(|c: &char| c.is_alphanumeric())
        .padded()
        .repeated()
        .at_least(1)
        .collect::<String>();
    let comp = choice((just("="), just(">="), just("<="), just("<"), just(">")))
        .padded()
        .map(|c| Comparator::try_from(c).expect("invalid input"));
    let compare = comp
        .padded()
        .then(ver().padded())
        .separated_by(just(','))
        .collect::<Vec<(Comparator, Version)>>();
    pkg.padded()
        .or_not()
        .then(compare.padded())
        .map(|(pkg, comparator)| VersionReq {
            comparator,
            name: pkg,
        })
}
