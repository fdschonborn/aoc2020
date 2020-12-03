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
#[error("failed to parse entry")]
pub enum ParseEntryError {
    ParseInt(#[from] std::num::ParseIntError),
    ParseChar(#[from] std::char::ParseCharError),
}

impl FromStr for Entry {
    type Err = ParseEntryError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split = input
            .split(['-', ' ', ':'].as_ref())
            .filter(|sub| !sub.is_empty())
            .collect::<Vec<_>>();

        Ok(Entry {
            min: split[0].parse()?,
            max: split[1].parse()?,
            letter: split[2].parse()?,
            input: split[3].to_owned(),
        })
    }
}

#[allow(dead_code)]
static DATABASE: Lazy<Vec<Entry>> = Lazy::new(|| {
    include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<Entry>().expect("Failed to parse entry"))
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
