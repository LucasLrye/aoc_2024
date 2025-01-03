use std::fs;
use std::path::PathBuf;

fn main() {
    println!("Hello, world 4!");
    let input = parse(PathBuf::from("./2024/04/src/text.txt"));
    let nbr_xmas = nbr_xmas(&input);
    println!("{nbr_xmas}");
    let nbr_xmas_2 = nbr_xmas_2(&input);
    println!("{nbr_xmas_2}");
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

fn nbr_xmas_2(grid: &[Vec<char>]) -> u32 {
    let mut count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            //println!("val is {:?}", grid[row][col]);
            if grid[row][col] == 'A' && check_x_mas(grid, row as i32, col as i32) {
                count += 1;
            }
        }
    }

    count
}

fn check_x_mas(grid: &[Vec<char>], row: i32, col: i32) -> bool {
    // Define patterns for diagonal checks
    let patterns = [
        (-1, -1, 1, 1), // Top-left to bottom-right
        (-1, 1, 1, -1), // Top-right to bottom-left
    ];
    let mut valid_diagonals = 0;

    // Check each pair of patterns
    for &(row_offset_1, col_offset_1, row_offset_2, col_offset_2) in &patterns {
        if check_mas(
            grid,
            row,
            col,
            row_offset_1,
            col_offset_1,
            row_offset_2,
            col_offset_2,
        ) {
            valid_diagonals += 1;
        }
    }

    valid_diagonals == 2
}
fn check_mas(
    grid: &[Vec<char>],
    row: i32,
    col: i32,
    row_offset_1: i32,
    col_offset_1: i32,
    row_offset_2: i32,
    col_offset_2: i32,
) -> bool {
    // Calculate intermediate row and column indices
    let row_1 = row + row_offset_1;
    let col_1 = col + col_offset_1;

    let row_2 = row + row_offset_2;
    let col_2 = col + col_offset_2;

    // Ensure indices are within bounds
    if row_1 < 0 || row_1 >= grid.len() as i32 || col_1 < 0 || col_1 >= grid[0].len() as i32 {
        return false;
    }
    if row_2 < 0 || row_2 >= grid.len() as i32 || col_2 < 0 || col_2 >= grid[0].len() as i32 {
        return false;
    }

    // Convert safe indices to usize
    let m_row = row_1 as usize;
    let m_col = col_1 as usize;
    let s_row = row_2 as usize;
    let s_col = col_2 as usize;

    #[cfg(test)]
    // Debugging information
    println!(
        "Checking pattern at ({}, {}) and ({}, {}): {} and {}",
        m_row, m_col, s_row, s_col, grid[m_row][m_col], grid[s_row][s_col]
    );

    // Check if the pattern matches "M.S" or "S.M"
    if (grid[m_row][m_col] == 'M' && grid[s_row][s_col] == 'S')
        || (grid[m_row][m_col] == 'S' && grid[s_row][s_col] == 'M')
    {
        #[cfg(test)]
        println!("demi-ok");
        return true;
    }
    false
}

fn nbr_xmas(grid: &[Vec<char>]) -> u32 {
    let directions = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // diagonal right down
        (-1, -1), // diagonal left up
        (1, -1),  // diagonal left down
        (-1, 1),  // diagonal right up
    ];

    let mut count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'X' {
                for &(dr, dc) in &directions {
                    if is_xmas(grid, row as i32, col as i32, dr, dc) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn is_xmas(grid: &[Vec<char>], start_row: i32, start_col: i32, dr: i32, dc: i32) -> bool {
    let target = ['X', 'M', 'A', 'S'];
    for i in 0..4 {
        let r = start_row + dr * i;
        let c = start_col + dc * i;
        if r < 0 || c < 0 || r >= grid.len() as i32 || c >= grid[0].len() as i32 {
            return false; // Out of bounds
        }
        if grid[r as usize][c as usize] != target[i as usize] {
            return false; // Mismatch
        }
    }
    true
}

#[cfg(test)]
#[test]
fn test_nbr_xmas() {
    let input = vec![
        vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
        vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
        vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
        vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
        vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
        vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
        vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
        vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
        vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
        vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
    ];

    assert_eq!(nbr_xmas(&input), 18);
}

#[cfg(test)]
#[test]
fn test_nbr_xmas_2() {
    let input = vec![
        vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
        vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
        vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
        vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
        vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
        vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
        vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
        vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
        vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
        vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
    ];

    assert_eq!(nbr_xmas_2(&input), 9);
}

#[cfg(test)]
#[test]
fn test_2_simple() {
    let input = vec![
        vec!['M', 'M', 'S'],
        vec!['M', 'A', 'M'],
        vec!['M', 'M', 'S'],
    ];
    println!("test 2");

    assert_eq!(nbr_xmas_2(&input), 1);
}

#[test]
fn test_x_max_2() {
    let grid = vec![
        vec!['A', 'M', 'S'],
        vec!['M', 'A', 'M'],
        vec!['S', 'M', 'A'],
    ];
    assert!(check_x_mas(&grid, 1, 1)); // Example with a valid X pattern
    assert!(!check_x_mas(&grid, 0, 0)); // Example with no valid pattern
}

/*
// Bad

fn nbr_xmas(input: String) -> u32 {
    let mut cpt = 0;
    let input = input.trim();
    let line_lenght = input.lines().next().unwrap_or("0").len();
    println!("len of lines is {}", line_lenght);
    for (i, char) in input.chars().enumerate() {
        //println!("i {}", i);
        if i < 3 {
            if char == 'X' {
                if input.chars().nth(i + 1) == Some('M') {
                    // right
                    if input.chars().nth(i + 2) == Some('A') {
                        // right
                        if input.chars().nth(i + 3) == Some('S') {
                            // right

                            cpt += 1;
                        }
                    }
                } else if input.chars().nth(i + line_lenght) == Some('M') {
                    println!("test D");
                    // down
                    if input.chars().nth(i + line_lenght * 2) == Some('A') {
                        println!("test D");
                        // down
                        if input.chars().nth(i + line_lenght * 3) == Some('S') {
                            println!("test D");
                            // down
                            cpt += 1;
                        }
                    }
                } else if input.chars().nth(i + line_lenght + 1) == Some('M') {
                    // diag right down
                    if input.chars().nth(i + line_lenght * 2 + 2) == Some('A') {
                        // diag right down
                        if input.chars().nth(i + line_lenght * 3 + 3) == Some('S') {
                            // diag right down
                            cpt += 1;
                        }
                    }
                }
            }
        } else if i < line_lenght * 3 {
            if char == 'X' {
                if input.chars().nth(i + 1) == Some('M') {
                    // right
                    if input.chars().nth(i + 2) == Some('A') {
                        // right
                        if input.chars().nth(i + 3) == Some('S') {
                            // right

                            cpt += 1;
                        }
                    }
                } else if input.chars().nth(i - 1) == Some('M') {
                    // left
                    if input.chars().nth(i - 2) == Some('A') {
                        // left
                        if input.chars().nth(i - 3) == Some('S') {
                            // left
                            cpt += 1;
                        }
                    }
                } else if input.chars().nth(i + line_lenght) == Some('M') {
                    println!("test D");
                    // down
                    if input.chars().nth(i + line_lenght * 2) == Some('A') {
                        println!("test D");
                        // down
                        if input.chars().nth(i + line_lenght * 3) == Some('S') {
                            println!("test D");
                            // down
                            cpt += 1;
                        }
                    }
                } else if input.chars().nth(i + line_lenght - 1) == Some('M') {
                    // diag left down
                    if input.chars().nth(i + line_lenght * 2 - 2) == Some('A') {
                        // diag left down
                        if input.chars().nth(i + line_lenght * 3 - 3) == Some('S') {
                            // diag left down
                            cpt += 1;
                        }
                    }
                } else if input.chars().nth(i + line_lenght + 1) == Some('M') {
                    // diag right down
                    if input.chars().nth(i + line_lenght * 2 + 2) == Some('A') {
                        // diag right down
                        if input.chars().nth(i + line_lenght * 3 + 3) == Some('S') {
                            // diag right down
                            cpt += 1;
                        }
                    }
                }
            }
        } else if char == 'X' {
            if input.chars().nth(i + 1) == Some('M') {
                // right
                if input.chars().nth(i + 2) == Some('A') {
                    // right
                    if input.chars().nth(i + 3) == Some('S') {
                        // right

                        cpt += 1;
                    }
                }
            } else if input.chars().nth(i - 1) == Some('M') {
                // left
                if input.chars().nth(i - 2) == Some('A') {
                    // left
                    if input.chars().nth(i - 3) == Some('S') {
                        // left
                        cpt += 1;
                    }
                }
            } else if input.chars().nth(i - line_lenght) == Some('M') {
                println!("test U");
                println!("next : {:?}", input.chars().nth(i - (line_lenght * 3 + 2)));
                // up
                if input.chars().nth(i - (line_lenght * 2 + 1)) == Some('A') {
                    println!("test U");
                    // up
                    if input.chars().nth(i - (line_lenght * 3 + 2)) == Some('S') {
                        println!("test U");
                        // up
                        cpt += 1;
                    }
                }
            } else if input.chars().nth(i + line_lenght) == Some('M') {
                println!("test D");
                // down
                if input.chars().nth(i + line_lenght * 2) == Some('A') {
                    println!("test D");
                    // down
                    if input.chars().nth(i + line_lenght * 3) == Some('S') {
                        println!("test D");
                        // down
                        cpt += 1;
                    }
                }
            } else if input.chars().nth(i - line_lenght - 1) == Some('M') {
                // diag left up
                if input.chars().nth(i - line_lenght * 2 - 2) == Some('A') {
                    // diag left up
                    if input.chars().nth(i - line_lenght * 3 - 3) == Some('S') {
                        // diag left up
                        cpt += 1;
                    }
                }
            } else if input.chars().nth(i - line_lenght + 1) == Some('M') {
                // diag right up
                if input.chars().nth(i - line_lenght * 2 + 2) == Some('A') {
                    // diag right up
                    if input.chars().nth(i - line_lenght * 3 + 3) == Some('S') {
                        // diag right up
                        cpt += 1;
                    }
                }
            } else if input.chars().nth(i + line_lenght - 1) == Some('M') {
                // diag left down
                if input.chars().nth(i + line_lenght * 2 - 2) == Some('A') {
                    // diag left down
                    if input.chars().nth(i + line_lenght * 3 - 3) == Some('S') {
                        // diag left down
                        cpt += 1;
                    }
                }
            } else if input.chars().nth(i + line_lenght + 1) == Some('M') {
                // diag right down
                if input.chars().nth(i + line_lenght * 2 + 2) == Some('A') {
                    // diag right down
                    if input.chars().nth(i + line_lenght * 3 + 3) == Some('S') {
                        // diag right down
                        cpt += 1;
                    }
                }
            }
        }
    }
    cpt
}

*/
