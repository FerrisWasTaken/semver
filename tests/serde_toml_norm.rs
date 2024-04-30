use semver::{Comparator, Deserialize, Version, VersionReq};

#[derive(Debug, Deserialize, PartialEq)]
struct TestFile {
    test_str: Version,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Req {
    comp: VersionReq
}

#[test]
fn norm_test() {
    assert_eq!(
        toml::from_str(r#"test_str = "1.0.1""#),
        Ok(TestFile {
            test_str: Version::Common {
                major: 1,
                minor: 0,
                rev: 1,
                pre: None
            }
        })
    );
    assert_eq!(
        toml::from_str(r#"test_str = "latest""#),
        Ok(TestFile {
            test_str: Version::Latest
        })
    );
    // assert_eq!(
    //     toml::from_str(r#"comp = "rust = 1.0.1""#),
    //     Ok(Req {comp: VersionReq {comparator: vec![(Comparator)]}})
    // )
}
