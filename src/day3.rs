#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    let mut offset = 0;

    input
        .lines()
        .filter(|text| {
            let matches = text.chars().cycle().nth(offset).unwrap() == '#';
            offset += 3;

            matches
        })
        .count()
}
