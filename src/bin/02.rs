use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let text = reader.lines().flatten().collect::<Vec<String>>();

        let mut num_of_safe = 0;
        for line in text {
            let res: Vec<usize> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();
            res.iter().tuple_windows().all(|(a, b)| (a < b) && (b - a) <= 3).then(|| num_of_safe += 1);
            res.iter().tuple_windows().all(|(a, b)| (a > b) && (a - b) <= 3).then(|| num_of_safe += 1);
        }

        Ok(num_of_safe)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let text = reader.lines().flatten().collect::<Vec<String>>();

        let mut num_of_safe = 0;
        for line in text {
            let backup = num_of_safe;
            let mut res: Vec<usize> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();
            res.iter().tuple_windows().all(|(a, b)| (a < b) && (b - a) <= 3).then(|| num_of_safe += 1);
            res.iter().tuple_windows().all(|(a, b)| (a > b) && (a - b) <= 3).then(|| num_of_safe += 1);
            
            if backup != num_of_safe { continue }
            
            let res_backup = res.clone();
            let mut found: bool = false;
            for i in 0..res.len() {
                res.remove(i);

                res.iter().tuple_windows().all(|(a, b)| (a < b) && (b - a) <= 3).then(|| { num_of_safe += 1; found = true; });
                res.iter().tuple_windows().all(|(a, b)| (a > b) && (a - b) <= 3).then(|| { num_of_safe += 1; found = true; });
                
                if found { break } else { res = res_backup.clone(); }
            }
        }
        
        Ok(num_of_safe)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
