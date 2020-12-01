const INPUT: &str = include_str!("../../input.txt");

fn main() {
    let values = INPUT
        .lines()
        .map(|line| line.parse::<usize>())
        .collect::<Result<Vec<_>, std::num::ParseIntError>>()
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
