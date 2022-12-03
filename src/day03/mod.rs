use ahash::AHashSet;
use log::debug;

use aoc_lib::parse;

pub fn solve(input: &[u8]) -> (String, String) {
    let mut input = input;

    let mut part1: i32 = 0;

    while !input.is_empty() {
        let mut n: usize = 0;
        for &b in input.iter() {
            if b == b'\n' {
                break;
            }
            n += 1;
        }
        let mid = n / 2;

        let lhs = &input[0..mid];
        let rhs = &input[mid..n];
        debug!(
            "lhs: {}, rhs: {}",
            String::from_utf8_lossy(lhs),
            String::from_utf8_lossy(rhs)
        );

        // TODO: use bitsets to represent rucksack
        //let lhs_rucksack: u64 = 0;
        //let rhs_rucksack: u64 = 0;
        let mut lhs_rucksack = AHashSet::with_capacity(mid);
        for &b in lhs.iter() {
            lhs_rucksack.insert(b);
        }
        let mut rhs_rucksack = AHashSet::with_capacity(mid);
        for &b in rhs.iter() {
            rhs_rucksack.insert(b);
        }

        for &b in lhs_rucksack.intersection(&rhs_rucksack) {
            let prio = match b {
                b'a'..=b'z' => b - b'a' + 1,
                b'A'..=b'Z' => b - b'A' + 27,
                _ => panic!("invalid character"),
            };
            debug!("common: {}, prio: {prio}", b as char);
            part1 += prio as i32;
        }

        input = &input[n..];
        input = parse::seek_next_line(input);
    }

    let part2: i64 = 42;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 03;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part1_example() {
        init();

        let input = b"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        let solution = solve(input);
        assert_eq!("157", solution.0);
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
        assert_eq!("7917", answer.0);
        // TODO
        //assert_eq!("42", answer.1);
    }
}
