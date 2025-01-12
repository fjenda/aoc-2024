use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: i64,
    col: i64,
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self {
            row: row.try_into().unwrap(),
            col: col.try_into().unwrap(),
        }
    }

    fn boundary_check(&self, w: i64, h: i64) -> bool {
        self.row >= 0 && self.row < h && self.col >= 0 && self.col < w
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader.lines().flatten().collect::<Vec<String>>().join("\n");
        let height = input.lines().count().try_into().unwrap();
        let width = input.lines().next().unwrap().len().try_into().unwrap();
        let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.char_indices() {
                if c == '.' {
                    continue;
                }

                antennas.entry(c).or_default().push(Position::new(row, col));
            }
        }

        let mut anti: HashSet<Position> = HashSet::new();
        for positions in antennas.values() {
            for pair in positions.iter().combinations(2) {
                let (a, b) = (*pair[0], *pair[1]);
                let delta = b - a;

                let anti_1 = b + delta;
                let anti_2 = a - delta;

                if anti_1.boundary_check(width, height) {
                    anti.insert(anti_1);
                }

                if anti_2.boundary_check(width, height) {
                    anti.insert(anti_2);
                }
            }
        }

        Ok(anti.len())
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader.lines().flatten().collect::<Vec<String>>().join("\n");
        let height = input.lines().count().try_into().unwrap();
        let width = input.lines().next().unwrap().len().try_into().unwrap();

        let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.char_indices() {
                if c == '.' {
                    continue;
                }
                antennas.entry(c).or_default().push(Position::new(col, row));
            }
        }

        let mut anti: HashSet<Position> = HashSet::new();
        for positions in antennas.values() {
            for pair in positions.iter().combinations(2) {
                let (a, b) = (*pair[0], *pair[1]);
                let delta = b - a;

                let mut antinode = b;
                while antinode.boundary_check(width, height) {
                    anti.insert(antinode);
                    antinode = antinode + delta;
                }

                let mut antinode = a;
                while antinode.boundary_check(width, height) {
                    anti.insert(antinode);
                    antinode = antinode - delta;
                }
            }
        }

        Ok(anti.len())
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
