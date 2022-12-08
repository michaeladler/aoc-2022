use ahash::AHashSet;
use log::debug;

use aoc_lib::parse;

const MAX_ROWS: usize = 99;
const MAX_COLS: usize = 99;

pub fn solve(input: &[u8]) -> (String, String) {
    let mut grid: [[i8; MAX_COLS]; MAX_ROWS] = [[0; MAX_COLS]; MAX_ROWS];
    let mut rows: u32 = 0;
    let mut cols: u32 = 0;

    let mut input = input;
    while !input.is_empty() {
        for (col, &b) in input.iter().enumerate() {
            if b == b'\n' {
                cols = col as u32;
                rows += 1;
                input = &input[col..];
                break;
            }
            grid[rows as usize][col] = (b - b'0') as i8;
        }
        input = parse::seek_next_line(input);
    }
    debug!("rows: {rows}, cols: {cols}");

    let mut visible: AHashSet<(u32, u32)> = AHashSet::with_capacity(MAX_COLS * MAX_ROWS);

    // check from WEST to EAST
    for y in 0..rows {
        let mut max: i8 = i8::MIN;
        for x in 0..cols {
            let value = unsafe { *grid.get_unchecked(y as usize).get_unchecked(x as usize) };
            // everything smaller than max is a candidate
            if value > max {
                // point can be seen from the outside
                visible.insert((y, x));
                max = value;
            }
        }
    }

    // check from EAST to WEST
    for y in 0..rows {
        let mut max: i8 = i8::MIN;
        for x in (0..cols).rev() {
            let value = unsafe { *grid.get_unchecked(y as usize).get_unchecked(x as usize) };
            // everything smaller than max is a candidate
            if value > max {
                // point can be seen from the outside
                visible.insert((y, x));
                max = value;
            }
        }
    }

    // check from NORTH to SOUTH
    for x in 0..cols {
        let mut max: i8 = i8::MIN;
        for y in 0..rows {
            let value = unsafe { *grid.get_unchecked(y as usize).get_unchecked(x as usize) };
            // everything smaller than max is a candidate
            if value > max {
                // point can be seen from the outside
                visible.insert((y, x));
                max = value;
            }
        }
    }

    // check from SOUTH to NORTH
    for x in 0..cols {
        let mut max: i8 = i8::MIN;
        for y in (0..rows).rev() {
            let value = unsafe { *grid.get_unchecked(y as usize).get_unchecked(x as usize) };
            // everything smaller than max is a candidate
            if value > max {
                // point can be seen from the outside
                visible.insert((y, x));
                max = value;
            }
        }
    }

    let part1 = visible.len();
    let part2: i64 = 42;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 08;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part1_example() {
        init();

        let input = b"30373
25512
65332
33549
35390
";
        let solution = solve(input);
        assert_eq!("21", solution.0);
    }

    #[test]
    #[ignore]
    fn part2_example() {
        let bufs = vec![(b"", 0)];

        for (s, answer) in bufs {
            assert_eq!(answer.to_string(), solve(s).1);
        }
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("1814", answer.0);
        // TODO
        //assert_eq!("42", answer.1);
    }
}
