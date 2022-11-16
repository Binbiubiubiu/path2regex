use eyre::Result;
use path2regex::PathRegex;
use regex::Regex;

fn main() -> Result<()> {
    // let re =  PathRegex::new(r"\w+")?;
    let re: PathRegex = Regex::new("\\w+").unwrap().into();
    println!("{}", re);
    Ok(())
}
