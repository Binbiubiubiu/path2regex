use anyhow::Result;
use path2regex::PathRegex;
use regex::Regex;

fn main() -> Result<()> {
    // let re =  PathRegex::new(r"\w+")?;
    let re: PathRegex = PathRegex::new(Regex::new("\\w+")?)?;
    println!("{}", re);
    Ok(())
}
