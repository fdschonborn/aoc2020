const INPUT: &str = include_str!("../../input.txt");

fn main() {
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
