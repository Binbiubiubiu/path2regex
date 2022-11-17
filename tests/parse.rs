use path2regex::{PathRegexOptions, TryIntoWith};

fn main() -> anyhow::Result<()> {
    let tokens = "/a/:b".try_into_with(&PathRegexOptions::default())?;
    println!("{:?}", tokens);
    Ok(())
}
