#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut row_start = 0f64;
            let mut row_end = 127f64;
            for c in line[..7].chars() {
                match c {
                    'F' => row_end = ((row_start + row_end) / 2.).floor(),
                    'B' => row_start = ((row_start + row_end) / 2.).ceil(),
                    _ => unreachable!(),
                }
            }

            if row_start != row_end {
                panic!("ROW: {} {}", row_start, row_end);
            }
            let row = row_start.abs();

            let mut column_start = 0f64;
            let mut column_end = 7f64;
            for c in line[7..].chars() {
                match c {
                    'L' => column_end = ((column_start + column_end) / 2.).floor(),
                    'R' => column_start = ((column_start + column_end) / 2.).ceil(),
                    _ => unreachable!(),
                }
            }

            if column_start != column_end {
                panic!("COLUMN: {} {}", column_start, column_end);
            }
            let column = column_start;

            ((row * 8.) + column) as usize
        })
        .max()
        .unwrap()
}
