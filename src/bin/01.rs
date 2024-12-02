use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ptr::read;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let text = reader.lines().flatten().collect::<Vec<String>>();
        let mut left = Vec::with_capacity(text.len());
        let mut right = Vec::with_capacity(text.len());

        // parse the input
        for line in text {
            let mut iter = line.split_whitespace();
            left.push(iter.next().unwrap().parse::<usize>()?);
            right.push(iter.next().unwrap().parse::<usize>()?);
        }

        // sort vectors in ascending order
        left.sort();
        right.sort();

        // let mut sum = 0;
        // for i in 0..left.len() {
        //     let larger = left[i].max(right[i]);
        //     let smaller = left[i].min(right[i]);
        //     sum += larger - smaller;
        // }

        let sum = (0..left.len()).map(|i| left[i].max(right[i]) - left[i].min(right[i])).sum();

        Ok(sum)
    }


    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let text = reader.lines().flatten().collect::<Vec<String>>();
        let mut left = Vec::with_capacity(text.len());
        let mut right = Vec::with_capacity(text.len());

        // parse the input
        for line in text {
            let mut iter = line.split_whitespace();
            left.push(iter.next().unwrap().parse::<usize>()?);
            right.push(iter.next().unwrap().parse::<usize>()?);
        }

        let char_count: usize = left.iter().map(|l| l * right.iter().filter(|&r| l == r).count()).sum();

        Ok(char_count)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
