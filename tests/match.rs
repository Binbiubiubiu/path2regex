#![cfg(feature = "match")]

use regex::Regex;

fn main() {
    let a = Regex::new("foo*").unwrap();
    let str1 = "table football, foosball";
    while let Some(m) = a.find(str1) {
        println!("Found {}. Next starts at {}.", m.as_str(), m.end());
    }
}
