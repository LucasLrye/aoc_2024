//! Work for test
//! But it doesn't for the whole because every page is seen 24 times...

use std::{collections::HashSet, fs, path::PathBuf};

fn main_topologie() {
    println!("Hello, world 5!");
    let (rules, updates) = parse(PathBuf::from("./2024/05/src/text.txt"));
    let (page_ord, hashet_rules) = page_ordering(rules);
    let input_updates = check_update_exist(updates, hashet_rules);
    let solution = check_valid_number(input_updates, page_ord);
    println!("Solution is {:?}", solution);
    let mid: u16 = solution.iter().map(|val| val[val.len() / 2]).sum();
    println!("Sum mid is {mid}");
}

fn parse(path: PathBuf) -> (String, String) {
    let input: String = fs::read_to_string(path).unwrap();
    let section: Vec<&str> = input.split("\n\n").collect();
    let rules = section[0];
    let updates = section[1];
    (rules.to_owned(), updates.to_owned())
}

fn page_ordering(input_rules: String) -> (Vec<u16>, HashSet<u16>) {
    let mut single_rules: HashSet<u16> = HashSet::new();
    let mut rules: Vec<(u16, u16)> = Vec::new(); //left first and right last
    let mut in_degree: Vec<u16> = vec![0; 100];
    for page in input_rules.lines() {
        // Part 1
        //println!("page is {}", page);
        let val: Vec<u16> = page.split("|").map(|val| val.parse().unwrap()).collect();
        let val2 = (val[0], val[1]);
        rules.push(val2);
        single_rules.insert(val[0]);
        single_rules.insert(val[1]);
    }

    // Build a flat in-degree table for topological sorting
    for &(_x, y) in &rules {
        in_degree[y as usize] += 1;
    }
    println!("page ordering is {:?}", in_degree);
    (in_degree, single_rules)
}
fn check_update_exist(update: String, rules: HashSet<u16>) -> Vec<Vec<u16>> {
    let mut update_mid: Vec<Vec<u16>> = Vec::new();
    for update_line in update.lines() {
        if !update_line.is_empty() {
            let update_parsed: Vec<u16> = update_line
                .split(',')
                .map(|page| page.parse::<u16>().unwrap())
                .collect();
            println!("update is {:?}", update_parsed);
            if update_parsed.iter().all(|val| rules.contains(val)) {
                update_mid.push(update_parsed);
            }
        }
    }
    update_mid
}

fn check_valid_number(input_updates: Vec<Vec<u16>>, in_degree_rules: Vec<u16>) -> Vec<Vec<u16>> {
    let mut solution: Vec<Vec<u16>> = Vec::new();
    let mut valid = true;
    for pages_updates in input_updates {
        for (indice, &page) in pages_updates.iter().enumerate() {
            if indice == pages_updates.len() - 1 {
                break;
            }

            /*println!(
                "!in_degree_rules[page as usize] {} < {}",
                in_degree_rules[page as usize],
                in_degree_rules[pages_updates[indice + 1] as usize]
            );*/
            if in_degree_rules[page as usize] >= in_degree_rules[pages_updates[indice + 1] as usize]
            {
                valid = false;
            }
        }
        if valid {
            solution.push(pages_updates);
        }
        valid = true;
    }
    solution
}

#[cfg(test)]
#[test]
fn test_05_in_degree_table() {
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
    let (in_degree, _hashet) = page_ordering(input);

    let mut in_degree_ex: Vec<u16> = vec![0; 100];
    let rules_example = [97, 75, 47, 61, 53, 29, 13];
    for (x, &y) in rules_example.iter().enumerate() {
        in_degree_ex[y as usize] += x as u16;
    }
    assert_eq!(in_degree, in_degree_ex);
}

#[cfg(test)]
#[test]
fn test_05_all() {
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
53|13


75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
        .to_owned();

    let section: Vec<&str> = input.split("\n\n").collect();
    let rules = section[0];
    let updates = section[1];
    println!("update is {:?}", updates);
    let (page_ord, hashet_rules) = page_ordering(rules.to_owned());
    let input_updates = check_update_exist(updates.to_owned(), hashet_rules);
    let solution = check_valid_number(input_updates, page_ord.clone());
    let solution_ex = vec![
        vec![75, 47, 61, 53, 29],
        vec![97, 61, 53, 29, 13],
        vec![75, 29, 13],
    ];
    assert_eq!(solution, solution_ex);

    let mid: u16 = solution.iter().map(|val| val[val.len() / 2]).sum();
    assert_eq!(mid, 143);
}
