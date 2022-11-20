use anyhow::Result;
use path2regex::{Key, PathRegex, PathRegexBuilder};
use regex::Regex;

pub const TEST_PATH: &'static str = "/user/:id";

#[test]
fn should_work_with_different_argument() -> Result<()> {
    assert!(PathRegex::new("/test").is_ok());
    assert!(PathRegex::new(Regex::new(r"^/test")?).is_ok());
    assert!(PathRegex::new(vec!["/a", "/b"]).is_ok());
    Ok(())
}

#[test]
fn should_get_keys() -> Result<()> {
    let re = PathRegexBuilder::new(TEST_PATH).set_end(false).build()?;
    assert_eq!(
        re.keys(),
        &vec![Key {
            name: "id".to_owned(),
            prefix: "/".to_owned(),
            suffix: "".to_owned(),
            modifier: "".to_owned(),
            pattern: "[^/\\#\\?]+?".to_owned(),
        }]
    );
    assert_eq!(
        re.captures("/user/123/show")
            .unwrap()
            .iter()
            .map(|x| {
                match x {
                    Some(x) => x.as_str(),
                    None => Default::default(),
                }
            })
            .collect::<Vec<_>>(),
        vec!["/user/123/", "123", "/"]
    );
    Ok(())
}

#[test]
#[should_panic = "Pattern cannot start with \"?\" at 6"]
fn should_throw_on_non_capturing_pattern() {
    PathRegex::new("/:foo(?:\\d+(\\.\\d+)?)").unwrap();
}

#[test]
#[should_panic = "Capturing groups are not allowed at 9"]
fn should_throw_on_nested_capturing_group() {
    PathRegex::new("/:foo(\\d+(\\.\\d+)?)").unwrap();
}

#[test]
#[should_panic = "Unbalanced pattern at 5"]
fn should_throw_on_unbalanced_pattern() {
    PathRegex::new("/:foo(abc").unwrap();
}

#[test]
#[should_panic = "Missing pattern at 5"]
fn should_throw_on_missing_pattern() {
    PathRegex::new("/:foo()").unwrap();
}

#[test]
#[should_panic = "Missing parameter name at 1"]
fn should_throw_on_missing_name() {
    PathRegex::new("/:(test)").unwrap();
}

#[test]
#[should_panic = "Unexpected OPEN at 3, expected CLOSE"]
fn should_throw_on_nested_groups() {
    PathRegex::new("/{a{b:foo}}").unwrap();
}

#[test]
#[should_panic = "Unexpected MODIFIER at 4, expected END"]
fn should_throw_on_misplaced_modifier() {
    PathRegex::new("/foo?").unwrap();
}
