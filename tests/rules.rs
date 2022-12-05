use anyhow::Result;
use path2regex::{
    CompilerBuilder, CompilerOptions, Key, MatchResult, MatcherBuilder, MatcherOptions, Parser,
    ParserOptions, PathRegex, PathRegexBuilder, PathRegexOptions, Token, TryIntoWith,
};
use serde_json::{json, Value};

struct CompileCase<'a> {
    params: Value,
    result: &'a str,
    options: CompilerOptions,
}

impl<'a> Default for CompileCase<'a> {
    fn default() -> Self {
        Self {
            params: json!({}),
            result: "",
            options: Default::default(),
        }
    }
}

#[derive(Default)]
struct MatchCase<'a> {
    path_name: &'a str,
    matches: Option<Vec<&'a str>>,
    params: Option<MatchResult>,
    options: MatcherOptions,
}

fn assert_re(
    path: impl TryIntoWith<PathRegex, PathRegexOptions>,
    tokens: &[Token],
    options: PathRegexOptions,
    should_parse_keys: bool,
) -> Result<PathRegex> {
    let re = PathRegexBuilder::new_with_options(path, options).build()?;
    let keys = re.keys();
    if should_parse_keys {
        let keys_in_tokens = tokens
            .iter()
            .map(|token| match token {
                Token::Key(key) => key.clone(),
                _ => Key::default(),
            })
            .filter(|x| !x.name.is_empty())
            .collect::<Vec<_>>();
        assert_eq!(keys, &keys_in_tokens, "should parse keys");
    }
    Ok(re)
}

fn assert_parse(path: impl AsRef<str>, tokens: &Vec<Token>, options: ParserOptions) -> Result<()> {
    let parser = Parser::new_with_options(options);
    assert_eq!(&parser.parse_str(path)?, tokens, "should parse");
    Ok(())
}

fn assert_compile(
    path: impl TryIntoWith<Vec<Token>, ParserOptions>,
    complie_cases: &Vec<CompileCase>,
    options: CompilerOptions,
) -> Result<()> {
    for case in complie_cases {
        #[allow(clippy::needless_update)]
        let options = CompilerOptions {
            delimiter: options.delimiter.clone(),
            prefixes: options.prefixes.clone(),
            sensitive: options.sensitive,
            encode: options.encode,
            validate: options.validate,
            ..case.options
        };
        let compiler = CompilerBuilder::new_with_options(path.clone(), options).build()?;
        if case.result.is_empty() {
            assert!(
                compiler.render(&case.params).is_err(),
                "should not compile using {}",
                case.params
            );
        } else {
            assert_eq!(
                compiler.render(&case.params)?,
                case.result,
                "should compile using {}",
                case.params
            );
        }
    }
    Ok(())
}

fn assert_match(
    path: impl TryIntoWith<PathRegex, PathRegexOptions>,
    re: &PathRegex,
    match_cases: &Vec<MatchCase>,
) -> Result<()> {
    for case in match_cases {
        let message = format!(
            "should {}match {}",
            if case.matches.is_none() { "not " } else { "" },
            case.path_name
        );
        let matches = re.captures(case.path_name).map(|cap| {
            cap.iter()
                .map(|x| match x {
                    Some(x) => x.as_str(),
                    None => Default::default(),
                })
                .collect::<Vec<_>>()
        });

        assert_eq!(matches, case.matches, "{message}");

        if case.params.is_some() {
            let matcher =
                MatcherBuilder::new_with_options(path.clone(), case.options.clone()).build()?;
            assert_eq!(
                matcher.find(case.path_name),
                case.params,
                "{message} params"
            );
        }
    }
    Ok(())
}

#[test]
fn test_rule_1() -> Result<()> {
    let path = "/";
    let ops = PathRegexOptions::default();
    let tokens = vec![Token::Static("/".to_owned())];

    let re = assert_re(path, &tokens, ops.clone(), false)?;

    assert_parse(path, &tokens, ParserOptions::from(ops))?;

    assert_compile(
        path,
        &vec![
            CompileCase {
                result: "/",
                ..Default::default()
            },
            CompileCase {
                params: json!({"id":123}),
                result: "/",
                ..Default::default()
            },
        ],
        CompilerOptions::default(),
    )?;

    assert_match(
        path,
        &re,
        &vec![
            MatchCase {
                path_name: "/",
                matches: Some(vec!["/"]),
                params: Some(MatchResult {
                    path: "/".to_owned(),
                    index: 0,
                    params: json!({}),
                }),
                ..Default::default()
            },
            MatchCase {
                path_name: "/route",
                ..Default::default()
            },
        ],
    )?;

    Ok(())
}

#[test]
fn test_rule_2() -> Result<()> {
    let path = "/test";
    let ops = PathRegexOptions::default();
    let tokens = vec![Token::Static("/test".to_owned())];

    let re = assert_re(path, &tokens, ops.clone(), false)?;

    assert_parse(path, &tokens, ParserOptions::from(ops))?;

    assert_compile(
        path,
        &vec![CompileCase {
            result: "/test",
            ..Default::default()
        }],
        CompilerOptions::default(),
    )?;

    assert_match(
        path,
        &re,
        &vec![
            MatchCase {
                path_name: "/test",
                matches: Some(vec!["/test"]),
                params: Some(MatchResult {
                    path: "/test".to_owned(),
                    index: 0,
                    params: json!({}),
                }),
                ..Default::default()
            },
            MatchCase {
                path_name: "/route",
                ..Default::default()
            },
            MatchCase {
                path_name: "/test/route",
                ..Default::default()
            },
            MatchCase {
                path_name: "/test/",
                matches: Some(vec!["/test/"]),
                params: Some(MatchResult {
                    path: "/test/".to_owned(),
                    index: 0,
                    params: json!({}),
                }),
                ..Default::default()
            },
        ],
    )?;

    Ok(())
}

#[test]
fn test_rule_3() -> Result<()> {
    let path = "/test/";
    let ops = PathRegexOptions::default();
    let tokens = vec![Token::Static("/test/".to_owned())];

    let re = assert_re(path, &tokens, ops.clone(), false)?;

    assert_parse(path, &tokens, ParserOptions::from(ops))?;

    assert_compile(
        path,
        &vec![CompileCase {
            result: "/test/",
            ..Default::default()
        }],
        CompilerOptions::default(),
    )?;

    assert_match(
        path,
        &re,
        &vec![
            MatchCase {
                path_name: "/test",
                matches: None,
                ..Default::default()
            },
            MatchCase {
                path_name: "/test/",
                matches: Some(vec!["/test/"]),
                ..Default::default()
            },
            MatchCase {
                path_name: "/test//",
                matches: Some(vec!["/test//"]),
                ..Default::default()
            },
        ],
    )?;

    Ok(())
}

#[test]
fn test_rule_4() -> Result<()> {
    let path = "/test";
    let ops = PathRegexOptions {
        sensitive: true,
        ..PathRegexOptions::default()
    };
    let tokens = vec![Token::Static("/test".to_owned())];

    let re = assert_re(path, &tokens, ops.clone(), false)?;

    assert_parse(path, &tokens, ParserOptions::from(ops))?;

    assert_match(
        path,
        &re,
        &vec![
            MatchCase {
                path_name: "/test",
                matches: Some(vec!["/test"]),
                ..Default::default()
            },
            MatchCase {
                path_name: "/TEST",
                ..Default::default()
            },
        ],
    )?;

    assert_compile(
        path,
        &vec![CompileCase {
            result: "/test",
            ..Default::default()
        }],
        CompilerOptions::default(),
    )?;

    Ok(())
}

#[test]
fn test_rule_5() -> Result<()> {
    let path = "/test";
    let ops = PathRegexOptions {
        strict: true,
        ..PathRegexOptions::default()
    };
    let tokens = vec![Token::Static("/test".to_owned())];

    let re = assert_re(path, &tokens, ops.clone(), false)?;

    assert_parse(path, &tokens, ParserOptions::from(ops))?;

    assert_match(
        path,
        &re,
        &vec![
            MatchCase {
                path_name: "/test",
                matches: Some(vec!["/test"]),
                ..Default::default()
            },
            MatchCase {
                path_name: "/test/",
                ..Default::default()
            },
            MatchCase {
                path_name: "/TEST",
                matches: Some(vec!["/TEST"]),
                ..Default::default()
            },
        ],
    )?;

    assert_compile(
        path,
        &vec![CompileCase {
            result: "/test",
            ..Default::default()
        }],
        CompilerOptions::default(),
    )?;

    Ok(())
}

#[test]
fn test_rule_6() -> Result<()> {
    let path = "/test/";
    let ops = PathRegexOptions {
        strict: true,
        ..PathRegexOptions::default()
    };
    let tokens = vec![Token::Static("/test/".to_owned())];

    let re = assert_re(path, &tokens, ops.clone(), false)?;

    assert_parse(path, &tokens, ParserOptions::from(ops))?;

    assert_match(
        path,
        &re,
        &vec![
            MatchCase {
                path_name: "/test",
                ..Default::default()
            },
            MatchCase {
                path_name: "/test/",
                matches: Some(vec!["/test/"]),
                ..Default::default()
            },
            MatchCase {
                path_name: "/test//",
                ..Default::default()
            },
        ],
    )?;

    assert_compile(
        path,
        &vec![CompileCase {
            result: "/test/",
            ..Default::default()
        }],
        CompilerOptions::default(),
    )?;

    Ok(())
}

#[test]
fn test_rule_7() -> Result<()> {
    let path = "/test";
    let ops = PathRegexOptions {
        end: false,
        ..PathRegexOptions::default()
    };
    let tokens = vec![Token::Static("/test".to_owned())];

    let re = assert_re(path, &tokens, ops.clone(), false)?;

    assert_parse(path, &tokens, ParserOptions::from(ops))?;

    assert_match(
        path,
        &re,
        &vec![
            MatchCase {
                path_name: "/test",
                matches: Some(vec!["/test", ""]),
                ..Default::default()
            },
            MatchCase {
                path_name: "/test/",
                matches: Some(vec!["/test/", ""]),
                ..Default::default()
            },
            MatchCase {
                path_name: "/test/route",
                matches: Some(vec!["/test/", "/"]),
                ..Default::default()
            },
            MatchCase {
                path_name: "/route",
                ..Default::default()
            },
        ],
    )?;

    assert_compile(
        path,
        &vec![CompileCase {
            result: "/test",
            ..Default::default()
        }],
        CompilerOptions::default(),
    )?;

    Ok(())
}
