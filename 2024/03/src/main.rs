use std::fs;
use std::path::PathBuf;

use regex::Regex;

fn main() {
    println!("Hello, world!");
    let _total1 = find_mul(PathBuf::from("./2024/03/src/text.txt"));
    //println!("{count1}");

}

fn find_mul(path: PathBuf) -> f32 {
    let reg = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();
    let mut mul_vec = Vec::new();
    let input: String = fs::read_to_string(path).unwrap();
    for caps in reg.captures_iter(&input){
        let first=caps[1].to_owned();
        let second=caps[2].to_owned();
        mul_vec.push((first, second));
        
    }

    let mut total = 0.0;
    for (num1, num2) in mul_vec{
        total+= num1.parse::<f32>().unwrap() * num2.parse::<f32>().unwrap();
    }
    println!("{:?}", total);
    total
}

