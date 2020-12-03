use std::str::FromStr;

use thiserror::Error;

#[derive(Debug)]
pub struct Entry {
    pub min: usize,
    pub max: usize,
    pub letter: char,
    pub pass: String,
}

#[derive(Debug, Error)]
pub enum ParseEntryError {
    #[error("Invalid part count, expected 4, got {}", .0)]
    InvalidPartCount(usize),
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error(transparent)]
    ParseChar(#[from] std::char::ParseCharError),
}

impl FromStr for Entry {
    type Err = ParseEntryError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts = input
            .split(['-', ' ', ':'].as_ref())
            .filter(|sub| !sub.is_empty())
            .collect::<Vec<_>>();

        let part_count = parts.len();
        if part_count != 4 {
            Err(ParseEntryError::InvalidPartCount(part_count))
        } else {
            Ok(Entry {
                min: parts[0].parse()?,
                max: parts[1].parse()?,
                letter: parts[2].parse()?,
                pass: parts[3].into(),
            })
        }
    }
}

#[aoc_generator(day2)]
pub fn generate_entries(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            line.parse()
                .expect(&format!("Failed to parse `{}` as Entry", line))
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Entry]) -> usize {
    input
        .iter()
        .filter(|entry| {
            (entry.min..=entry.max)
                .contains(&entry.pass.chars().filter(|c| *c == entry.letter).count())
        })
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Entry]) -> usize {
    input
        .iter()
        .filter(|entry| {
            let left = entry.pass.chars().nth(entry.min - 1).unwrap() == entry.letter;
            let right = entry.pass.chars().nth(entry.max - 1).unwrap() == entry.letter;

            (left || right) && !(left && right)
        })
        .count()
}
