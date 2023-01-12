use log::debug;

use aoc_lib::parse;

pub fn solve(input: &[u8]) -> (String, String) {
    let mut input = input;
    while !input.is_empty() {
        input = parse::seek_next_line(input);
    }

    let part1: i64 = 42;
    let part2: i64 = 42;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 23;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn example_large() {
        init();

        let input = b"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

        let answer = solve(input);
        assert_eq!("110", answer.0);
        // assert_eq!("42", answer.1);
    }

    #[test]
    #[ignore]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("42", answer.0);
        assert_eq!("42", answer.1);
    }
}
