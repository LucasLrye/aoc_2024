//mod topologie_false;

use std::fs;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("./2024/05/src/text.txt");
    let input: String = fs::read_to_string(path).unwrap();
    let val = part_one(&input);
    println!("val is {:?}", val);
    let val2 = part_two(&input);
    println!("val2 is {:?}", val2);
}
pub fn part_one(input: &str) -> u32 {
    let rules: Vec<(i32, i32)> = input
        .lines()
        .filter(|l| l.contains("|"))
        .map(|l| l.split_once("|").unwrap())
        .map(|n| (n.0.parse().unwrap(), n.1.parse().unwrap()))
        .collect();
    let updates: Vec<Vec<i32>> = input
        .lines()
        .filter(|l| l.contains(","))
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();
    let mut result: u32 = 0;

    for update in updates.iter() {
        let mut works: bool = true;
        for rule in rules.iter() {
            if update.contains(&rule.0)
                && update.contains(&rule.1)
                && update.iter().position(|l| l == &rule.0)
                    > update.iter().position(|l| l == &rule.1)
            {
                works = false;
            }
        }
        if works {
            result += update[update.len() / 2] as u32;
        }
    }
    result
}

pub fn part_two(input: &str) -> u32 {
    let rules: Vec<(i32, i32)> = input
        .lines()
        .filter(|l| l.contains("|"))
        .map(|l| l.split_once("|").unwrap())
        .map(|n| (n.0.parse().unwrap(), n.1.parse().unwrap()))
        .collect();
    let updates: Vec<Vec<i32>> = input
        .lines()
        .filter(|l| l.contains(","))
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();
    let mut result: u32 = 0;

    for mut update in updates.into_iter() {
        let mut works: bool = true;
        for rule in rules.iter() {
            if update.contains(&rule.0) && update.contains(&rule.1) {
                let left_pos = update.iter().position(|l| l == &rule.0).unwrap();
                let right_pos = update.iter().position(|l| l == &rule.1).unwrap();
                if left_pos > right_pos {
                    works = false;
                }
            }
        }
        if works {
            continue;
        }
        while !works {
            for rule in rules.iter() {
                if update.contains(&rule.0) && update.contains(&rule.1) {
                    let left_pos = update.iter().position(|l| l == &rule.0).unwrap();
                    let right_pos = update.iter().position(|l| l == &rule.1).unwrap();
                    if left_pos > right_pos {
                        let moved_item = update.remove(right_pos);
                        update.insert(left_pos, moved_item);
                    }
                }
            }
            let mut local_works: bool = true;
            for rule in rules.iter() {
                if update.contains(&rule.0) && update.contains(&rule.1) {
                    let left_pos = update.iter().position(|l| l == &rule.0).unwrap();
                    let right_pos = update.iter().position(|l| l == &rule.1).unwrap();
                    if left_pos > right_pos {
                        local_works = false;
                    }
                }
            }
            works = local_works;
        }
        result += update[update.len() / 2] as u32;
    }
    result
}
