use std::fs;
use std::path::PathBuf;

fn main() {
    println!("Hello, Day 8!");
    let input = parse(PathBuf::from("./2024/08/src/text.txt"));
}

fn parse(path: PathBuf) -> String {
    let input: String = fs::read_to_string(path).unwrap();
    input
}

#[cfg(test)]
#[test]
fn test_08_1() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
        .to_owned();
}
