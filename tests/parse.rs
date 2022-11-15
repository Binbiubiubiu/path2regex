use path2regex::ParserBuilder;



fn main() {
    let p = ParserBuilder::new().delimiter("/").prefixes("23").build();
    p.parse("2323").unwrap();
   

    
}
