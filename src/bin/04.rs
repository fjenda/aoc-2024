use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::enumerate;
use adv_code_2024::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn get_char(grid: &[String], x: i32, y: i32) -> Option<char> {
    if y >= 0 && y < grid.len() as i32 {
        if let Some(row) = grid.get(y as usize) {
            if x >= 0 && x < row.len() as i32 {
                return row.chars().nth(x as usize);
            }
        }
    }
    None
}

fn is_xmas(grid: &[String], y: i32, x: i32) -> bool {
    let diagonals = [
        [(-1, -1), (1, 1)], // tl tr
        [(-1, 1), (1, -1)], // bl br
    ];

    let mut matches = [false; 2];

    for diag in &diagonals {
        // one way
        if !matches_pattern(grid, x, y, diag[0], diag[1]) {
            return false;
        }
    }

    true
}

fn matches_pattern(grid: &[String], x: i32, y: i32, dir1: (i32, i32), dir2: (i32, i32)) -> bool {
    if let Some('M') = get_char(grid, x + dir1.0, y + dir1.1) {
        if let Some('S') = get_char(grid, x + dir2.0, y + dir2.1) {
            return true;
        }
    }

    // reverse
    if let Some('S') = get_char(grid, x + dir1.0, y + dir1.1) {
        if let Some('M') = get_char(grid, x + dir2.0, y + dir2.1) {
            return true;
        }
    }

    false
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let text = reader.lines().flatten().collect::<Vec<String>>();
        let mut total: usize = 0;

        let directions = [
            (-1, -1), (-1, 0), (-1, 1), // tl t tr
            (0, -1),          (0, 1),   // l  x  r
            (1, -1), (1, 0), (1, 1),    // bl b br
        ];

        // each line
        for (line_idx, line) in enumerate(&text) {
            // each char
            for (char_idx, c) in enumerate(line.chars()) {
               if c == 'X' {
                   // check around
                   for &(dx, dy) in &directions {
                       let mut x = char_idx as i32;
                       let mut y = line_idx as i32;

                       // check for M A S
                       if let Some('M') = get_char(&text, x + dx, y + dy) {
                           if let Some('A') = get_char(&text, x + 2 * dx, y + 2 * dy) {
                               if let Some('S') = get_char(&text, x + 3 * dx, y + 3 * dy) {
                                   total += 1;
                               }
                           }
                       }
                   }
               }
            }
        }

        Ok(total)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let text = reader.lines().flatten().collect::<Vec<String>>();
        let mut total: usize = 0;

        let diagonals = [
            (-1, -1), (-1, 1), // tl tr
            ( 1, -1), ( 1, 1), // bl br
        ];

        // each line
        for (line_idx, line) in enumerate(&text) {
            // each char
            for (char_idx, c) in enumerate(line.chars()) {
                if c == 'A' {
                    if is_xmas(&text, line_idx as i32, char_idx as i32) {
                        total += 1;
                    }
                }
            }
        }

        Ok(total)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
