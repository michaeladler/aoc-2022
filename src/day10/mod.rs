use log::debug;

use aoc_lib::parse;

#[derive(Debug)]
struct Addx(i64);

pub fn solve(input: &[u8]) -> (String, String) {
    let mut input = input;

    let mut x: i64 = 1;
    let mut cycle: i64 = 1;

    let mut part1: i64 = 0;

    while !input.is_empty() {
        debug!("cycle {cycle} begins. x={x}");
        match input[0] {
            b'a' => {
                // addx
                let (rest, n) = parse::integer(&input[5..], false).unwrap();
                debug!("addx {n}");
                input = rest;
                cycle += 1;
                if cycle % 40 == 20 {
                    part1 += x * cycle;
                }
                x += n;
            }
            b'n' => {
                debug!("noop");
            }
            _ => panic!("invalid input"),
        }
        cycle += 1;
        if cycle % 40 == 20 {
            part1 += x * cycle;
        }
        debug!("cycle {cycle} ends.");

        input = parse::seek_next_line(input);
    }
    debug!("finished program. cycle: {cycle}, x: {x}");
    let part2: i64 = 42;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 10;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn example() {
        init();

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
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("11960", answer.0);
        //assert_eq!("42", answer.1);
    }
}
