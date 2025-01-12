use std::collections::HashSet;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use petgraph::graphmap::DiGraphMap;
use adv_code_2024::*;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
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
97,13,75,29,47
";

fn is_valid_update(update: &[i32], rules: &[(i32, i32)]) -> bool {
    let page_positions = update
        .iter()
        .enumerate()
        .map(|(index, &page)| (page, index))
        .collect::<std::collections::HashMap<_, _>>();

    for &(x, y) in rules {
        if let (Some(&pos_x), Some(&pos_y)) = (page_positions.get(&x), page_positions.get(&y)) {
            if pos_x > pos_y {
                return false;
            }
        }
    }

    true
}

fn find_middle_page(update: &[i32]) -> i32 {
    update[update.len() / 2]
}

fn parse_input(input: String) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut sections = input.trim().split("\n\n");
    let rules_section = sections.next().unwrap();
    let updates_section = sections.next().unwrap();

    let rules = rules_section
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let updates = updates_section
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (rules, updates)
}

fn fix_update(update: &[i32], rules: &[(i32, i32)]) -> Vec<i32> {
    let update_pages: HashSet<i32> = update.iter().copied().collect();
    let mut graph = DiGraphMap::new();

    for &page in &update_pages {
        graph.add_node(page);
    }

    for &(x, y) in rules {
        if update_pages.contains(&x) && update_pages.contains(&y) {
            graph.add_edge(x, y, ());
        }
    }

    let mut sorted_pages = petgraph::algo::toposort(&graph, None)
        .expect("Graph contains a cycle!")
        .into_iter()
        .collect::<Vec<_>>();

    sorted_pages.retain(|page| update.contains(page));
    sorted_pages
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let text =
            reader
            .lines()
            .flatten()
            .collect::<Vec<String>>()
            .join("\n");

        let (rules, updates) = parse_input(text);
        let res: i32 = updates
            .iter()
            .filter(|update| is_valid_update(update, &rules))
            .map(|update| find_middle_page(update))
            .sum();

        Ok(res as usize)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let text =
            reader
                .lines()
                .flatten()
                .collect::<Vec<String>>()
                .join("\n");

        let (rules, updates) = parse_input(text);

        let res: i32 = updates
            .iter()
            .filter(|update| !is_valid_update(update, &rules))
            .map(|update| {
                let fixed_update = fix_update(update, &rules);
                find_middle_page(&fixed_update)
            })
            .sum();

        Ok(res as usize)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
