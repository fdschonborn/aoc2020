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
pub struct ParseEntryError;

impl FromStr for Entry {
    type Err = ParseEntryError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split = input
            .split(['-', ' ', ':'].as_ref())
            .filter(|sub| !sub.is_empty())
            .collect::<Vec<_>>();

        Ok(Entry {
            min: split[0].parse().unwrap(),
            max: split[1].parse().unwrap(),
            letter: split[2].parse().unwrap(),
            input: split[3].parse().unwrap(),
        })
    }
}

pub static DATABASE: Lazy<Vec<Entry>> = Lazy::new(|| {
    include_str!("../data/day2.txt")
        .lines()
        .map(|line| line.parse::<Entry>().expect("Failed to parse entry"))
        .collect::<Vec<_>>()
});

#[test]
fn test() {
    println!(
        "Count of Valid Passwords: {}",
        DATABASE
            .iter()
            .filter(|entry| {
                (entry.min..=entry.max)
                    .contains(&entry.input.chars().filter(|c| *c == entry.letter).count())
            })
            .count()
    );
}
