use std::fs::File;
use std::io::BufRead;

fn main() {
    println!("Hello, world!");
    let distance = get_distance_1("./2024/01/input.txt".to_owned());
    println!("the distance 1 is {}", distance);
    let distance2 = get_distance_2("./2024/01/input.txt".to_owned());
    print!("The distance 2 is {}", distance2);
}

fn two_column(path: String) -> (Vec<u64>, Vec<u64>) {
    let file = File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);

    let (column1, column2): (Vec<u64>, Vec<u64>) = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| {
            let parts: Vec<_> = line.split("   ").take(2).collect();
            if parts.len() == 2 {
                if let (Ok(col1), Ok(col2)) = (
                    parts[0].trim().parse::<u64>(),
                    parts[1].trim().parse::<u64>(),
                ) {
                    return Some((col1, col2)); // Retourne un tuple (col1, col2)
                }
            }
            None
        })
        .unzip();
    (column1, column2)
}

fn get_distance_1(path: String) -> u64 {
    let (mut column1, mut column2) = two_column(path);
    column1.sort();
    column2.sort();
    let distance = column1
        .iter()
        .zip(column2.iter())
        .map(|(c1, c2)| c1.abs_diff(*c2))
        .sum();
    distance
}

fn get_distance_2(path: String) -> u64 {
    let (column1, column2) = two_column(path);
    let distance = column1
        .iter()
        .map(|c1| {
            let cpt = column2.iter().filter(|c2| *c2 == c1).count();
            c1 * cpt as u64
        })
        .sum();
    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let distance = get_distance_1("./2024/01/input.txt".to_owned());
        assert_eq!(distance, 1580061);
    }

    #[test]
    fn part_2() {
        let distance = get_distance_2("./2024/01/input.txt".to_owned());
        assert_eq!(distance, 23046913);
    }
}
