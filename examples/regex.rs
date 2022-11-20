

fn main() {
    let re = path2regex::PathRegexBuilder::new("/test")
        .set_end(false)
        .build()
        .unwrap();

    // expected: Captures({0: Some("/test"), "END_WIND_DELIMITER": Some("")})
    // found: Captures({0: Some("/test"), "END_WIND_DELIMITER": Some("")})
    println!("{:?}", re.captures("/test").unwrap());
    // expected: Captures({0: Some("/test")})
    // found: Captures({0: Some("/test/"), "END_WIND_DELIMITER": Some("")})
    println!("{:?}", re.captures("/test/").unwrap());
    // expected: Captures({0: Some("/test")})
    // found: Captures({0: Some("/test/"), "END_WIND_DELIMITER": Some("/")})
    println!("{:?}", re.captures("/test/route").unwrap());
}
