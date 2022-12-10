use log::debug;

use aoc_lib::parse;

#[derive(Debug)]
struct Addx(i64);

const COLS: usize = 40;
const ROWS: usize = 6;

pub fn solve(input: &[u8]) -> (String, String) {
    let mut input = input;

    let mut x: i64 = 1;
    let mut cycle: usize = 1;

    let mut part1: i64 = 0;

    const LIT: u8 = b'#';
    const BLACK: u8 = b'.';

    let mut crt: [[u8; COLS]; ROWS] = [[BLACK; COLS]; ROWS];

    while !input.is_empty() {
        debug!("Start cycle {cycle}");
        {
            // part 2
            let (row, col) = cycle_to_coords(cycle);
            if sprite_visible(col, x) {
                crt[row][col] = LIT;
            }
        }

        match input[0] {
            b'a' => {
                // addx
                let (rest, n) = parse::integer(&input[5..], false).unwrap();
                debug!("begin executing addx {n}");
                input = rest;
                cycle += 1;
                {
                    // part 2
                    let (row, col) = cycle_to_coords(cycle);
                    if sprite_visible(col, x) {
                        crt[row][col] = LIT;
                    }
                }
                if cycle % 40 == 20 {
                    part1 += x * (cycle as i64);
                }
                x += n;
            }
            b'n' => {
                debug!("noop");
            }
            _ => panic!("invalid input"),
        }
        cycle += 1;
        {
            // part 2
            let (row, col) = cycle_to_coords(cycle);
            if sprite_visible(col, x) {
                crt[row][col] = LIT;
            }
        }

        if cycle % 40 == 20 {
            part1 += x * (cycle as i64);
        }

        input = parse::seek_next_line(input);
    }

    for row in crt {
        for b in row {
            print!("{}", b as char);
        }
        println!();
    }

    let part2: i64 = 0;

    (part1.to_string(), part2.to_string())
}

/// returns (row, col)
fn cycle_to_coords(cycle: usize) -> (usize, usize) {
    let cycle = cycle - 1;
    let row = cycle / COLS;
    let col = cycle % COLS;
    (row, col)
}

fn sprite_visible(col: usize, x: i64) -> bool {
    let col = col as i64;
    x - 1 <= col && col <= x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 10;

    #[test]
    fn example() {
        let input = b"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
        let solution = solve(input);
        assert_eq!("13140", solution.0);
    }

    #[test]
    fn test_cycle_to_coords() {
        assert_eq!((0, 0), cycle_to_coords(1));
        assert_eq!((0, 39), cycle_to_coords(40));
        assert_eq!((1, 0), cycle_to_coords(41));
        assert_eq!((1, 39), cycle_to_coords(80));
        assert_eq!((5, 0), cycle_to_coords(201));
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("11960", answer.0);
    }
}
