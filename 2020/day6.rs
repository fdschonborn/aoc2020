use std::collections::HashMap;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut answers = group.chars().filter(|&c| c != '\n').collect::<Vec<_>>();
            answers.sort();
            answers.dedup();

            answers.len()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut scores = HashMap::<char, usize>::new();

            let people = group.split('\n').collect::<Vec<_>>();
            for person in &people {
                let mut questions = person.chars().collect::<Vec<_>>();
                questions.sort();
                questions.dedup();

                for question in questions {
                    scores.entry(question).and_modify(|v| *v += 1).or_insert(1);
                }
            }

            let mut all_replied = vec![];
            for (k, v) in &scores {
                if *v == people.len() {
                    all_replied.push(k);
                }
            }

            all_replied.len()
        })
        .sum()
}
