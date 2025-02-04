use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("Hello, Day 8!");
    let input = parse(PathBuf::from("./2024/08/src/text.txt"));
    let (antenna_map, input_len) = get_antenna(input.clone());
    print!("len_input is {:?}", input_len);
    let (antinode, _result_nbr_tot) = get_antinode(antenna_map, input_len);
    println!("Antinode : {:?} and len {}", antinode, antinode.len());
}

fn parse(path: PathBuf) -> Vec<Vec<char>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_antenna(input: Vec<Vec<char>>) -> (HashMap<char, Vec<(usize, usize)>>, (usize, usize)) {
    let mut data: Vec<(char, (usize, usize))> = Vec::new();
    let mut antenna = HashMap::new(); // Hashmap: (char, (usize, usize))
    let len_x = input.len();
    let mut len_y = 0;
    for (x, x_value) in input.iter().enumerate() {
        len_y = x_value.len();
        for (y, char) in x_value.iter().enumerate() {
            if *char != '.' {
                data.push((*char, (x, y)));
            }
        }
    }
    for (ch, (x, y)) in data {
        antenna.entry(ch).or_insert_with(Vec::new).push((x, y));
    }
    (antenna, (len_x, len_y))
}

fn get_antinode(
    antenna: HashMap<char, Vec<(usize, usize)>>,
    input_len: (usize, usize),
) -> (HashSet<(usize, usize)>, usize) {
    let mut cpt = 0;
    let mut antinode_hash: HashSet<(usize, usize)> = HashSet::new();
    for (char, position) in &antenna {
        println!("Char: {}", char);
        for i in 0..position.len() {
            for j in (i + 1)..position.len() {
                let (x1, y1) = position[i];
                let (x2, y2) = position[j];
                let distance_x = (x2).abs_diff(x1);
                let distance_y = (y2).abs_diff(y1);
                let distance = (distance_x, distance_y);
                println!(
                    "Distance between {:?} and {:?}: {:?}",
                    (x1, y1),
                    (x2, y2),
                    distance
                );
                let antinode_x_1 = if x1 > x2 {
                    Some(x1 + distance.0)
                } else {
                    x1.checked_sub(distance.0)
                };
                let antinode_y_1 = if y1 > y2 {
                    Some(y1 + distance.1)
                } else {
                    y1.checked_sub(distance.1)
                };

                if let (Some(antinode_x_1), Some(antinode_y_1)) = (antinode_x_1, antinode_y_1) {
                    if antinode_x_1 < input_len.0 && antinode_y_1 < input_len.1 {
                        antinode_hash.insert((antinode_x_1, antinode_y_1));
                        cpt += 1;
                    }
                }

                let antinode_x_2 = if x2 > x1 {
                    Some(x2 + distance.0)
                } else {
                    x2.checked_sub(distance.0)
                };
                let antinode_y_2 = if y2 > y1 {
                    Some(y2 + distance.1)
                } else {
                    y2.checked_sub(distance.1)
                };

                if let (Some(antinode_x_2), Some(antinode_y_2)) = (antinode_x_2, antinode_y_2) {
                    if antinode_x_2 < input_len.0 && antinode_y_2 < input_len.1 {
                        antinode_hash.insert((antinode_x_2, antinode_y_2));
                        cpt += 1;
                    }
                }
            }
        }
    }
    (antinode_hash, cpt)
}

#[cfg(test)]
#[test]
fn test_08_1() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
        .to_owned();
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (antenna_map, input_len) = get_antenna(input.clone());
    print!("len_input is {:?}", input_len);
    let (antinode, _result_nbr_tot) = get_antinode(antenna_map, input_len);
    println!("Antinode : {:?} and len {}", antinode, antinode.len());

    let mut grid = vec![vec!['.'; 12]; 12]; // Initialize a 12x12 grid with '.'

    for &(x, y) in &antinode {
        grid[x][y] = '#'; // Mark positions with '#' (or any other character)
    }

    // Print the grid
    for row in &grid {
        println!("{}", row.iter().collect::<String>());
    }

    assert_eq!(antinode.len(), 14);
}

/*
{(7, 8), (5, 8), (6, 5), (4, 9), (5, 7), (0, 4), (7, 7), (1, 3), (2, 4), (3, 8), (0, 5)}
*/
