pub fn count_trees(input: &str, right: usize, down: usize) -> usize {
    let mut offset = 0;

    input
        .lines()
        .enumerate()
        .filter(|(n, _)| if down > 0 { n % down == 0 } else { true })
        .filter(|(_, line)| {
            let matches = line.chars().cycle().nth(offset).unwrap() == '#';
            offset += right;

            matches
        })
        .count()
}

#[aoc(day3, part1)]
pub fn day3_part1(input: &str) -> usize {
    count_trees(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn day3_part2(input: &str) -> usize {
    count_trees(input, 1, 1)
        * count_trees(input, 3, 1)
        * count_trees(input, 5, 1)
        * count_trees(input, 7, 1)
        * count_trees(input, 1, 2)
}
