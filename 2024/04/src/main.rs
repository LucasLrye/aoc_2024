use std::fs;
use std::path::PathBuf;

fn main() {
    println!("Hello, world!");
    let _total1 = parse(PathBuf::from("./2024/04/src/text.txt"));
    //println!("{count1}");
}

fn parse(path: PathBuf) -> u32 {
    let input: String = fs::read_to_string(path).unwrap();
    0
}
