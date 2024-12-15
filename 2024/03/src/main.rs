use std::fs;
use std::path::PathBuf;

use regex::Regex;

fn main() {
    println!("Hello, world!");
    let _total1 = find_mul(PathBuf::from("./2024/03/src/text.txt"));
    let _total2 = find_mul2(PathBuf::from("./2024/03/src/text.txt"));
    //println!("{count1}");
}

fn find_mul(path: PathBuf) -> u32 {
    let reg = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();
    let mut mul_vec = Vec::new();
    let input: String = fs::read_to_string(path).unwrap();
    for caps in reg.captures_iter(&input) {
        let first = caps[1].to_owned();
        let second = caps[2].to_owned();
        mul_vec.push((first, second));
    }
    //println!("mul vec {:?}", mul_vec);

    let mut total = 0;
    for (num1, num2) in mul_vec {
        total += num1.parse::<u32>().unwrap() * num2.parse::<u32>().unwrap();
    }
    println!("TOTAL 1: {:?}", total);
    total
}
fn find_mul2(path: PathBuf) -> u32 {
    let mut take = true;
    let pattern = r"(?P<mul>mul\((\d+),\s*(\d+)\))|(?P<do>do\(\))|(?P<dont>don't\(\))";
    let reg = Regex::new(pattern).unwrap();
    let mut mul_vec = Vec::new();
    let input: String = fs::read_to_string(path).unwrap();
    for caps in reg.captures_iter(&input) {
        if caps.name("mul").is_some() {
            if take {
                let first = caps[2].to_owned();
                let second = caps[3].to_owned();
                mul_vec.push((first, second));
            }
        } else if caps.name("do").is_some() {
            take = true;
        } else if caps.name("dont").is_some() {
            take = false;
        }
    }
    //println!("mul vec {:?}", mul_vec);

    let mut total = 0;
    for (num1, num2) in mul_vec {
        total += num1.parse::<u32>().unwrap() * num2.parse::<u32>().unwrap();
    }
    println!("TOTAL 2: {:?}", total);
    total
}
