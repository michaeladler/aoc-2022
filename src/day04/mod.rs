use log::debug;

use aoc_lib::parse;

pub fn solve(input: &[u8]) -> (String, String) {
    let mut input = input;
    let mut part1: i32 = 0;
    while !input.is_empty() {
        let (rest, a1) = parse::positive(input, false).unwrap();
        let (rest, b1) = parse::positive(&rest[1..], false).unwrap();
        let (rest, a2) = parse::positive(&rest[1..], false).unwrap();
        let (rest, b2) = parse::positive(&rest[1..], false).unwrap();
        debug!("{a1}-{b1},{a2}-{b2}");
        debug_assert!(a1 <= b1);
        debug_assert!(a2 <= b2);

        // a1 <= a2 <= b2 <= b1
        if a1 <= a2 && b2 <= b1 {
            part1 += 1;
        } else if a2 <= a1 && b1 <= b2 {
            part1 += 1;
        }

        input = parse::seek_next_line(rest);
    }

    let part2: i64 = 42;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 04;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part1_example() {
        init();

        let input = b"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
        assert_eq!("2", solve(input).0);
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
        assert_eq!("538", answer.0);
        //assert_eq!("42", answer.1);
    }
}
