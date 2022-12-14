# Path-to-RegExp

 ![Crates.io](https://img.shields.io/crates/v/path2regex) ![Crates.io](https://img.shields.io/crates/d/path2regex) ![Crates.io](https://img.shields.io/crates/l/path2regex) [![CI](https://github.com/Binbiubiubiu/path2regex/actions/workflows/CI.yml/badge.svg)](https://github.com/Binbiubiubiu/path2regex/actions/workflows/CI.yml) [![codecov](https://codecov.io/gh/Binbiubiubiu/path2regex/branch/main/graph/badge.svg?token=YTIKDKKKBV)](https://codecov.io/gh/Binbiubiubiu/path2regex)

## Installation

``` bash
cargo add path2regex
```

## Features

- **default**: support [PathRegex](https://docs.rs/path2regex/latest/path2regex/struct.PathRegex.html) and [Parser](https://docs.rs/path2regex/latest/path2regex/struct.Parser.html)
- **compile**: support [Compiler](https://docs.rs/path2regex/latest/path2regex/struct.Compiler.html)
- **match**: support [Matcher](https://docs.rs/path2regex/latest/path2regex/struct.Matcher.html)

## Usage

Similar to [path-to-regex](https://github.com/pillarjs/path-to-regexp)

### Differences

Thought that [regex](https://docs.rs/regex/latest/regex/) was not supported `?=`,The performance of the `end` and `ends_with` property in the configuration item will vary.

notice: [regex](https://docs.rs/regex/latest/regex/) executes `captures` to get the result, providing the name `END_WITH_DELIMITER` matching group to handle the extra endings

```rust
let re = path2regex::PathRegexBuilder::new("/test")
        .set_end(false)
        .build()
        .unwrap();

// expected: Captures({0: Some("/test")})
// found: Captures({0: Some("/test"), "END_WIND_DELIMITER": Some("")})
println!("{:?}", re.captures("/test").unwrap());
// expected: Captures({0: Some("/test")})
// found: Captures({0: Some("/test/"), "END_WIND_DELIMITER": Some("")})
println!("{:?}", re.captures("/test/").unwrap());
// expected: Captures({0: Some("/test")})
// found: Captures({0: Some("/test/"), "END_WIND_DELIMITER": Some("/")})
println!("{:?}", re.captures("/test/route").unwrap());
```
