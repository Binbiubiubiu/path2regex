use path2regex::ParserBuilder;

fn main() {
    let _p = ParserBuilder::new().delimiter("/").prefixes("23").build();
    // p.parse("2323").unwrap();
    let a = 1;
    format!("{}", a);
}
