use std::fs;
use std::path::PathBuf;

fn main() {
    println!("Hello, Day 9!");
    let _input = parse(PathBuf::from("./2024/09/src/text.txt"));
}

fn parse(path: PathBuf) -> String {
    let input: String = fs::read_to_string(path).unwrap();
    input
}

#[cfg(test)]
mod test {

    #[test]
    fn test_09_1() {}
}
