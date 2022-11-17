#![cfg(feature = "compile")]

use path2regex::Compiler;
use serde_json::json;

fn main() -> anyhow::Result<()> {
    let s = Compiler::new("/:a/:b")?.render(&json!({
        "a":1,
        "b":2
    }))?;
    dbg!(s);
    Ok(())
}
