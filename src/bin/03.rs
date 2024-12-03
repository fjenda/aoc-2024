use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use adv_code_2024::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let regex = Regex::new(r"mul\((\d+),(\d+)\)")?;
        let text = reader.lines().collect::<Result<Vec<_>, _>>()?;

        let mut pairs = Vec::new();
        for line in text {
            for cap in regex.captures_iter(&line) {
                let n: u32 = cap[1].parse()?;
                let m: u32 = cap[2].parse()?;
                pairs.push((n, m));
            }
        }

        let total: u32 = pairs.iter().map(|(n, m)| n * m).sum();
        Ok(total as usize)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let combined_regex = Regex::new(
            r"(?P<do>do\(\))|(?P<dont>don't\(\))|(?P<mul>mul\((\d+),(\d+)\))"
        ).unwrap();

        let text = reader.lines().collect::<Result<Vec<_>, _>>()?;

        let mut pairs = Vec::new();
        let mut enabled: bool = true;
        for line in text {
            for caps in combined_regex.captures_iter(&line) {
                if caps.name("do").is_some() {
                    enabled = true;
                } else if caps.name("dont").is_some() {
                    enabled = false;
                } else if let Some(_) = caps.name("mul") {
                    if enabled {
                        let n: u32 = caps.get(4).unwrap().as_str().parse()?;
                        let m: u32 = caps.get(5).unwrap().as_str().parse()?;
                        pairs.push((n, m));
                    }
                }
            }
        }

        let total: u32 = pairs.iter().map(|(n, m)| n * m).sum();
        Ok(total as usize)
    }

    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
