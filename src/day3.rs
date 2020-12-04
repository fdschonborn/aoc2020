fn count_trees(input: &str, right: usize, down: usize) -> usize {
    let mut offset = 0;

    input
        .lines()
        .enumerate()
        .filter(|(line, _)| if down > 0 { line % down == 0 } else { true })
        .filter(|(_, text)| {
            let matches = text.chars().cycle().nth(offset).unwrap() == '#';
            offset += right;

            matches
        })
        .count()
}

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    count_trees(input, 3, 1)
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    let a = dbg!(count_trees(input, 1, 1));
    let b = dbg!(count_trees(input, 3, 1));
    let c = dbg!(count_trees(input, 5, 1));
    let d = dbg!(count_trees(input, 7, 1));
    let e = dbg!(count_trees(input, 1, 2));

    a * b * c * d * e
}
