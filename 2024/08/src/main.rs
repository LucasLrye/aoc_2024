use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Roof {
    antennas: HashMap<char, Vec<Position>>,
    spots: Vec<Position>,
}

/// Didn't suceed -> respect to maebli.github | I was close but it wasn't that's..
fn main() {
    let input = include_str!("text.txt");
    let mut x = 0i64;
    let mut y = 0i64;

    let roof = input.chars().fold(
        Roof {
            antennas: HashMap::new(),
            spots: vec![],
        },
        |mut roof, spot| {
            match spot {
                '.' => {
                    roof.spots.push(Position { x, y });
                    x += 1;
                }
                '\n' => {
                    x = 0;
                    y += 1;
                }
                spot if spot.is_alphanumeric() => {
                    let new_spot = Position { x, y };
                    let antenna_positions = roof.antennas.entry(spot).or_default();
                    antenna_positions.push(new_spot.clone());
                    roof.spots.push(new_spot);
                    x += 1;
                }
                _ => panic!("Invalid character"),
            }
            roof
        },
    );

    let anti_nodes =
        roof.antennas
            .into_values()
            .fold(HashSet::new(), |anti_nodes, antenna_positions| {
                let pairs = antenna_positions
                    .iter()
                    .enumerate()
                    .flat_map(|(i, a)| antenna_positions.iter().skip(i + 1).map(move |b| (a, b)));

                let new_anti_nodes = pairs.fold(HashSet::new(), |mut new_anti_nodes, (a, b)| {
                    let mut loop_num = 0;
                    loop {
                        let delta = Position {
                            x: b.x - a.x,
                            y: b.y - a.y,
                        };
                        let (new_a, new_b) = (
                            Position {
                                x: a.x + delta.x * loop_num,
                                y: a.y + delta.y * loop_num,
                            },
                            Position {
                                x: b.x - delta.x * loop_num,
                                y: b.y - delta.y * loop_num,
                            },
                        );

                        let mut new_spots = 0;
                        if roof.spots.contains(&new_a) {
                            new_anti_nodes.insert(new_a);
                            new_spots += 1;
                        }
                        if roof.spots.contains(&new_b) {
                            new_anti_nodes.insert(new_b);
                            new_spots += 1;
                        }

                        if new_spots == 0 {
                            break;
                        }

                        loop_num += 1;
                    }
                    new_anti_nodes
                });

                anti_nodes.union(&new_anti_nodes).cloned().collect()
            });

    println!("{:?}", anti_nodes.len());
}
#[allow(dead_code)]
fn part1_works() {
    println!("Hello, Day 8!");
    let input = parse(PathBuf::from("./2024/08/src/text.txt"));
    let (antenna_map, input_len) = get_antenna(input.clone());
    print!("len_input is {:?}", input_len);
    let antinode = get_antinode_1(antenna_map.clone(), input_len);
    println!("Antinode : {:?} and len {}", antinode, antinode.len());
    let antinode = get_antinode_2(antenna_map, input_len);
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

fn get_antinode_1(
    antenna: HashMap<char, Vec<(usize, usize)>>,
    input_len: (usize, usize),
) -> HashSet<(usize, usize)> {
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
                    }
                }
            }
        }
    }
    antinode_hash
}
fn get_antinode_2(
    antenna: HashMap<char, Vec<(usize, usize)>>,
    input_len: (usize, usize),
) -> HashSet<(usize, usize)> {
    let mut antinode_hash: HashSet<(usize, usize)> = HashSet::new();

    for (char, mut position) in antenna.clone() {
        println!("Char: {}", char);
        for i in 0..position.len() {
            for j in (i + 1)..position.len() {
                antinode_hash.insert(position[i]);
                let mut loop_mov = 1;
                loop {
                    let mut cpt = 1;
                    let (x1, y1) = position[i];
                    let (x2, y2) = position[j];
                    let distance_x = (x2).abs_diff(x1) * loop_mov; // loop_mov ? IDK
                    let distance_y = (y2).abs_diff(y1) * loop_mov;
                    let distance = (distance_x, distance_y);

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
                        if antinode_x_1 < input_len.0
                            && antinode_y_1 < input_len.1
                            && antinode_hash.insert((antinode_x_1, antinode_y_1))
                        {
                            //position.push((antinode_x_1, antinode_y_1));
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
                        if antinode_x_2 < input_len.0
                            && antinode_y_2 < input_len.1
                            && antinode_hash.insert((antinode_x_2, antinode_y_2))
                        {
                            cpt += 1;
                            //position.push((antinode_x_2, antinode_y_2));
                        }
                    }
                    if cpt == 1 {
                        break;
                    }

                    loop_mov += 1;
                }
            }
        }
    }
    antinode_hash
}

#[cfg(test)]
mod test {
    use super::*;
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
        let antinode = get_antinode_1(antenna_map, input_len);
        println!("Antinode : {:?} and len {}", antinode, antinode.len());

        let mut grid = input; // Initialize a 12x12 grid with '.'

        for &(x, y) in &antinode {
            grid[x][y] = '#'; // Mark positions with '#' (or any other character)
        }

        // Print the grid
        for row in &grid {
            println!("{}", row.iter().collect::<String>());
        }

        assert_eq!(antinode.len(), 14);
    }

    #[test]
    fn test_08_2() {
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
        let antinode = get_antinode_2(antenna_map, input_len);
        //let antinode = get_antinode_1(antinode, input_len);

        //println!("Antinode : {:?} and len {}", antinode, antinode.len());

        let mut grid = input; // Initialize a 12x12 grid with '.'

        for &(x, y) in &antinode {
            grid[x][y] = '#'; // Mark positions with '#' (or any other character)
        }

        // Print the grid
        for row in &grid {
            println!("{}", row.iter().collect::<String>());
        }

        assert_eq!(antinode.len(), 34);
    }
}
/*
{(7, 8), (5, 8), (6, 5), (4, 9), (5, 7), (0, 4), (7, 7), (1, 3), (2, 4), (3, 8), (0, 5)}
*/
