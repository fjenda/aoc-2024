use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn reachable(start: usize, target: usize, num: &[usize]) -> bool {
    if num.is_empty() {
        return start == target;
    }

    if start > target {
        return false;
    }

    let (head, rest) = num.split_first().unwrap();
    reachable(start * head, target, rest) || reachable(start + head, target, rest)
}

fn reachable_concat(start: usize, target: usize, num: &[usize]) -> bool {
    if num.is_empty() {
        return start == target;
    }

    if start > target {
        return false;
    }

    let (head, rest) = num.split_first().unwrap();
    reachable_concat(start * head, target, rest)
        || reachable_concat(start + head, target, rest)
        || reachable_concat(concat(start, *head), target, rest)
}

fn concat(a: usize, b: usize) -> usize {
    let mut off = 1;

    while off <= b {
        off *= 10;
    }

    a * off + b
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut equations: Vec<(usize, Vec<usize>)> = Vec::new();
        let lines = reader.lines().flatten().collect::<Vec<String>>();
        for line in lines {
            let (result, numbers) = line.split_once(":").unwrap();
            let result = result.parse()?;
            let numbers = numbers
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            equations.push((result, numbers));
        }

        let mut res = 0;
        for (target, numbers) in equations {
            let (start, numbers) = numbers.split_first().unwrap();
            if reachable(*start, target, numbers) {
                res += target;
            }
        }

        Ok(res)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut equations: Vec<(usize, Vec<usize>)> = Vec::new();
        let lines = reader.lines().flatten().collect::<Vec<String>>();
        for line in lines {
            let (result, numbers) = line.split_once(":").unwrap();
            let result = result.parse()?;
            let numbers = numbers
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            equations.push((result, numbers));
        }

        let mut res = 0;
        for (target, numbers) in equations {
            let (start, numbers) = numbers.split_first().unwrap();
            if reachable_concat(*start, target, numbers) {
                res += target;
            }
        }

        Ok(res)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
