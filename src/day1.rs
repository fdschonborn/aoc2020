use once_cell::sync::Lazy;

pub static NUMBERS: Lazy<Vec<usize>> = Lazy::new(|| {
    include_str!("../data/day1.txt")
        .lines()
        .map(|line| line.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
});

#[test]
fn part1() {
    for a in &*NUMBERS {
        for b in &*NUMBERS {
            if a + b == 2020 {
                println!("Result: {}", a * b);
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
                    println!("Result: {}", a * b * c);
                    return;
                }
            }
        }
    }
}
