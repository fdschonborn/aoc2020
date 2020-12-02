#[cfg(test)]
const INPUT: &str = include_str!("../data/day1.txt");

#[test]
fn part1() {
    let values = INPUT
        .lines()
        .map(|line| line.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    for a in &values {
        for b in &values {
            if a + b == 2020 {
                println!("Result: {}", a * b);
                return;
            }
        }
    }
}

#[test]
fn part2() {
    let values = INPUT
        .lines()
        .map(|line| line.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    for a in &values {
        for b in &values {
            for c in &values {
                if a + b + c == 2020 {
                    println!("Result: {}", a * b * c);
                    return;
                }
            }
        }
    }
}
