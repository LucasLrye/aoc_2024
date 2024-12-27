use std::fs;
use std::path::PathBuf;

fn main() {
    println!("Hello, world 5!");
    let input = parse(PathBuf::from("./2024/05/src/text.txt"));
    let page_ord = page_ordering(input);
    //println!("{page_ord}");
    //let nbr_xmas_2 = nbr_xmas_2(&input);
    //println!("{nbr_xmas_2}");
}

fn parse(path: PathBuf) -> String {
    let input: String = fs::read_to_string(path).unwrap();
    input
}

fn page_ordering(input: String) -> Vec<u16> {
    let mut rules: Vec<(u16, u16)> = Vec::new(); //left first and right last
    let mut in_degree: Vec<u16> = vec![0; 100]; // Assuming page numbers are <= 999
    for page in input.lines() {
        // Part 1
        //        println!("page is {}", page);
        if !page.contains("|") {
            break;
        }
        let val: Vec<u16> = page.split("|").map(|val| val.parse().unwrap()).collect();
        let val2 = (val[0], val[1]);
        rules.push(val2);
    }

    // Perform Kahn's Algorithm
    /*
           let mut queue: VecDeque<u16> = update
               .iter()
               .filter(|&&page| in_degree[page as usize] == 0)
               .cloned()
               .collect();

    let mut sorted_order = Vec::new();
    while let Some(current) = queue.pop_front() {
        sorted_order.push(current);
        for &(x, y) in &rules {
            if x == current {
                in_degree[y as usize] -= 1;
                if in_degree[y as usize] == 0 {
                    queue.push_back(y);
                }
            }
        }
    } */

    // Build a flat in-degree table for topological sorting
    for &(x, y) in &rules {
        in_degree[y as usize] += 1;
    }
    println!("page ordering is {:?}", in_degree);
    in_degree
}

#[cfg(test)]
#[test]
fn test_05() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13"
        .to_owned();
    let in_degree = page_ordering(input);

    let mut in_degree_ex: Vec<u16> = vec![0; 100];
    let rules_example = [97, 75, 47, 61, 53, 29, 13];
    for (x, &y) in rules_example.iter().enumerate() {
        in_degree_ex[y as usize] += x as u16;
    }
    assert_eq!(in_degree, in_degree_ex);
}
