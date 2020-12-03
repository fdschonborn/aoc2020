use std::str::FromStr;

use once_cell::sync::Lazy;
use thiserror::Error;

#[derive(Debug)]
pub struct Entry {
    pub min: usize,
    pub max: usize,
    pub letter: char,
    pub input: String,
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
                input: parts[3].to_owned(),
            })
        }
    }
}

pub static DATABASE: Lazy<Vec<Entry>> = Lazy::new(|| {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            line.parse::<Entry>()
                .expect(&format!("Failed to parse `{}` as Entry", line))
        })
        .collect::<Vec<_>>()
});

#[test]
fn part1() {
    println!(
        "Result: {}",
        DATABASE
            .iter()
            .filter(|entry| {
                (entry.min..=entry.max)
                    .contains(&entry.input.chars().filter(|c| *c == entry.letter).count())
            })
            .count()
    );
}

#[test]
fn part2() {
    println!(
        "Result: {}",
        DATABASE
            .iter()
            .filter(|entry| {
                let left = entry.input.chars().nth(entry.min - 1).unwrap() == entry.letter;
                let right = entry.input.chars().nth(entry.max - 1).unwrap() == entry.letter;

                (left || right) && !(left && right)
            })
            .count()
    )
}
