use once_cell::sync::Lazy;

#[allow(dead_code)]
static NUMBERS: Lazy<Vec<usize>> = Lazy::new(|| {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            line.parse::<usize>()
                .expect(&format!("Failed to parse {} as number", line))
        })
        .collect()
});

#[test]
fn part1() {
    for a in &*NUMBERS {
        for b in &*NUMBERS {
            if a + b == 2020 {
                println!("Result: {} + {} = {}", a, b, a * b);
                return;
            }
        }
    }
}

#[test]
fn part2() {
    for a in &*NUMBERS {
        for b in &*NUMBERS {
            for c in &*NUMBERS {
                if a + b + c == 2020 {
                    println!("Result: {} + {} + {} = {}", a, b, c, a * b * c);
                    return;
                }
            }
        }
    }
}
