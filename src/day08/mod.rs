use ahash::AHashSet;

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

    // part 2
    let mut part2: u32 = 0;
    for y in 0..rows {
        for x in 0..cols {
            let value = unsafe { *grid.get_unchecked(y as usize).get_unchecked(x as usize) };
            let scenic_up = (0..y)
                .rev()
                .find(|&y_above| unsafe {
                    *grid
                        .get_unchecked(y_above as usize)
                        .get_unchecked(x as usize)
                        >= value
                })
                .map(|y_first_bad| y - y_first_bad)
                .unwrap_or(y);
            let scenic_down = (y + 1..rows)
                .find(|&y_down| unsafe {
                    *grid
                        .get_unchecked(y_down as usize)
                        .get_unchecked(x as usize)
                        >= value
                })
                .map(|y_first_bad| y_first_bad - y)
                .unwrap_or((rows - 1) - y);
            let scenic_left = (0..x)
                .rev()
                .find(|&x_left| unsafe {
                    *grid
                        .get_unchecked(y as usize)
                        .get_unchecked(x_left as usize)
                        >= value
                })
                .map(|x_first_bad| x - x_first_bad)
                .unwrap_or(x);
            let scenic_right = (x + 1..cols)
                .find(|&x_right| unsafe {
                    *grid
                        .get_unchecked(y as usize)
                        .get_unchecked(x_right as usize)
                        >= value
                })
                .map(|x_first_bad| x_first_bad - x)
                .unwrap_or((cols - 1) - x);
            let scenic_score = scenic_left * scenic_right * scenic_up * scenic_down;
            if scenic_score > part2 {
                part2 = scenic_score;
            }
        }
    }
    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 08;

    #[test]
    fn example() {
        let input = b"30373
25512
65332
33549
35390
";
        let solution = solve(input);
        assert_eq!("21", solution.0);
        assert_eq!("8", solution.1);
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("1814", answer.0);
        assert_eq!("330786", answer.1);
    }
}
