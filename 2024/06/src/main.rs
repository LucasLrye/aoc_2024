use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("Hello, Day 6!");
    let input = parse(PathBuf::from("./2024/06/src/text.txt"));
    let val1 = process(input);
    println!("val 1 is {}", val1);
}

/*
fn parse(path: PathBuf) -> String {
    let input: String = fs::read_to_string(path).unwrap();
    input
}
*/
fn parse(path: PathBuf) -> Vec<Vec<char>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn process(grid: Vec<Vec<char>>) -> u32 {
    let directions = [
        (-1, 0), // up
        (0, 1),  // right
        (1, 0),  // down
        (0, -1), // left
    ];
    let mut cpt_direction = 0;
    let mut cpt_step = 0;
    let mut case_visited: HashSet<(i32, i32)> = HashSet::new();

    let mut next_row;
    let mut next_col;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '^' {
                println!("guard found at row {} and col {}", row, col);
                next_col = col as i32;
                next_row = row as i32;
                loop {
                    let &(dr, dc) = &directions[cpt_direction];
                    //println!("We are at row {} and col {}", next_row + dr, next_col + dc);

                    if (next_row + dr).abs() >= (grid.len() as i32).abs()
                        || (next_col + dc).abs() >= (grid[row].len() as i32).abs()
                    {
                        break;
                    }
                    if grid[(next_row + dr) as usize][(next_col + dc) as usize] == '#' {
                        if cpt_direction == 3 {
                            cpt_direction = 0;
                        } else {
                            cpt_direction += 1;
                        }
                    } else {
                        next_row += dr;
                        next_col += dc;
                        let not_visited = case_visited.insert((next_row, next_col));
                        if not_visited {
                            cpt_step += 1;
                        }
                    }
                }
            }
        }
    }
    cpt_step
}

#[cfg(test)]
#[test]
fn test_06() {
    let input = vec![
        vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
        vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
    ];

    let val = process(input);
    println!(" val is {}", val);
    assert_eq!(val, 41);
}

/*
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
*/
