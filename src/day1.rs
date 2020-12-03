#[aoc_generator(day1)]
pub fn generate_numbers(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| {
            line.parse()
                .expect(&format!("Failed to parse `{}` as usize", line))
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[usize]) -> usize {
    for a in input {
        for b in input {
            if a + b == 2020 {
                return a * b;
            }
        }
    }

    unreachable!()
}

#[aoc(day1, part2)]
pub fn part2(input: &[usize]) -> usize {
    for a in input {
        for b in input {
            for c in input {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }

    unreachable!()
}
