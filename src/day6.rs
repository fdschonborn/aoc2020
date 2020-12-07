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
