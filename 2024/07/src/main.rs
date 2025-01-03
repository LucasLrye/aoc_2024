use std::fs;
use std::path::PathBuf;

fn main() {
    println!("Hello, Day 7!");
    let input = parse(PathBuf::from("./2024/07/src/text.txt"));
    let good = part_one(&input);
    let val1 = process1(input);
    println!("val 1 is {}", val1);
    println!("part1 is {:?}", good);
}

fn parse(path: PathBuf) -> String {
    let input: String = fs::read_to_string(path).unwrap();
    input
}

fn process1(input: String) -> u64 {
    let equations: Vec<(u64, Vec<u64>)> = input
        .lines()
        .map(|l| {
            l.split_once(':')
                .map(|n| {
                    (
                        n.0.parse().unwrap(),
                        n.1.split_whitespace()
                            .map(|val| val.parse().unwrap())
                            .collect(),
                    )
                })
                .unwrap()
        })
        .collect();
    let mut total = 0;
    for eq in equations.iter() {
        total += test_operator(eq);
    }
    total
}

fn test_operator(eq: &(u64, Vec<u64>)) -> u64 {
    // Generate all possible operator combinations
    let max_combinations = 1 << (eq.1.len() - 1); // 2^(n-1) combinations
    for i in 0..max_combinations {
        let mut result = eq.1[0];
        let mut expression = eq.1[0].to_string();
        for (j, &num) in eq.1[1..].iter().enumerate() {
            if (i & (1 << j)) != 0 {
                // Add
                result += num;
                expression = format!("{} + {}", expression, num);
            } else {
                // Multiply
                result *= num;
                expression = format!("{} * {}", expression, num);
            }
            if result == eq.0 && j == eq.1.len() - 2 {
                //println!("result is {} whith expression {:?}", result, expression);
                return result;
            }
        }
    }
    0
}

fn part_one(input: &str) -> Option<i64> {
    let mut equations: Vec<(i64, Vec<i64>)> = vec![];
    for line in input.lines() {
        let target: i64 = line.split_once(":").unwrap().0.parse().unwrap();
        let nums: Vec<i64> = line
            .split_once(":")
            .unwrap()
            .1
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();
        equations.push((target, nums));
    }
    let mut result: i64 = 0;
    for equation in equations {
        let mut results: Vec<i64> = vec![equation.1[0]];

        for &n in &equation.1[1..] {
            results = results.iter().flat_map(|&r| vec![r * n, r + n]).collect();
        }
        if results.contains(&equation.0) {
            result += equation.0;
        }
    }
    Some(result)
}

#[cfg(test)]
#[test]
fn test_07_1() {
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
    let val = process1(input);
    assert_eq!(val, 3749);
}
