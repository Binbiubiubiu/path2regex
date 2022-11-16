use path2regex::{json, Compiler};

fn main() -> eyre::Result<()> {
    let s = Compiler::new("/:a/:b")?.render(&json!({
        "a":1,
        "b":2
    }))?;
    dbg!(s);
    Ok(())
}
