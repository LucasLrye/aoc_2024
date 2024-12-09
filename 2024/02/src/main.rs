use std::fs;
use std::path::PathBuf;

fn main() {
    println!("Hello, world!");
    let _count1 = nbr_safe_1(PathBuf::from("./2024/02/src/text.txt"));
    let _count2 = nbr_safe_2(PathBuf::from("./2024/02/src/text.txt"));
    //println!("{count1}");
    //println!("{count2}");
}

fn nbr_safe_1(path: PathBuf) -> u32 {
    let input: String = fs::read_to_string(path).unwrap();
    let mut safe = 0;
    for report in input.lines() {
        let report_parsed: Vec<u32> = report
            .split_whitespace()
            .map(|val| val.parse::<u32>().unwrap())
            .collect();
        if is_safe_1(report_parsed) {
            safe += 1;
        }
    }
    println!("{safe}");
    safe
}

// check is full inc or full dec and if  1<= n-(n-1) <=3 so safe is =+ 1

fn is_safe_1(report: Vec<u32>) -> bool {
    if report.len() > 1 {
        if report[0] > report[1] {
            report
                .iter()
                .zip(report.iter().skip(1))
                .all(|(&x, &y)| x > y && x.abs_diff(y) <= 3)
        } else {
            report
                .iter()
                .zip(report.iter().skip(1))
                .all(|(&x, &y)| x < y && x.abs_diff(y) <= 3)
        }
    } else {
        false
    }
}

fn nbr_safe_2(path: PathBuf) -> u32 {
    let input: String = fs::read_to_string(path).unwrap();
    let mut safe = 0;
    for report in input.lines() {
        let report_parsed: Vec<u32> = report
            .split_whitespace()
            .map(|val| val.parse::<u32>().unwrap())
            .collect();
        if is_safe_with_dampener(report_parsed) {
            safe += 1;
        }
    }
    println!("{safe}");
    safe
}

/// Checks if a report becomes safe when one "bad" level is removed.
fn is_safe_with_dampener(report: Vec<u32>) -> bool {
    if is_safe_1(report.clone()) {
        return true;
    }

    // Try removing each level one at a time
    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);

        if is_safe_1(modified_report) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_1() {
        let count = nbr_safe_1(PathBuf::from("./text.txt"));
        assert_eq!(count, 246);
    }

    #[test]
    fn part_2_2() {
        let count = nbr_safe_2(PathBuf::from(
            "/HOME/Project/solo/aof_2024/2024/02/src/text.txt",
        ));
        assert_eq!(count, 318);
    }
}
