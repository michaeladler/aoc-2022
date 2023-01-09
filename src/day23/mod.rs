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
    #[ignore]
    fn example() {
        init();

        let input = b"TODO
";

        let answer = solve(input);
        assert_eq!("42", answer.0);
        // assert_eq!("42", answer.1);
    }

    #[test]
    #[ignore]
    fn part1_example() {
        init();

        let bufs = vec![(b"", 0)];

        for (s, answer) in bufs {
            assert_eq!(answer.to_string(), solve(s).0);
        }
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
    #[ignore]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("42", answer.0);
        assert_eq!("42", answer.1);
    }
}
