use std::collections::HashSet;

use arrayvec::ArrayVec;
use log::debug;

use aoc_lib::parse;

pub fn solve(input: &[u8]) -> (String, String) {
    let mut input = input;

    let mut part1: i32 = 0;
    let mut part2: i32 = 0;

    let mut elves: ArrayVec<HashSet<u8>, 3> = ArrayVec::new();

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

        // TODO: use bitsets to represent rucksack
        //let lhs_rucksack: u64 = 0;
        //let rhs_rucksack: u64 = 0;
        let mut items = HashSet::with_capacity(n);
        let mut lhs_rucksack = HashSet::with_capacity(mid);
        for &b in lhs.iter() {
            lhs_rucksack.insert(b);
            items.insert(b);
        }
        let mut rhs_rucksack = HashSet::with_capacity(mid);
        for &b in rhs.iter() {
            rhs_rucksack.insert(b);
            items.insert(b);
        }

        for &b in lhs_rucksack.intersection(&rhs_rucksack) {
            let prio = calc_prio(b);
            debug!("common: {}, prio: {prio}", b as char);
            part1 += prio as i32;
        }
        // TODO: unchecked
        elves.push(items);
        if elves.len() == 3 {
            // from https://github.com/rust-lang/rfcs/issues/2023
            let intersection = elves.iter().skip(1).fold(elves[0].clone(), |acc, hs| {
                acc.intersection(hs).cloned().collect()
            });
            for &b in intersection.iter() {
                let prio = calc_prio(b);
                part2 += prio as i32;
            }

            elves.clear();
        }

        input = &input[n..];
        input = parse::seek_next_line(input);
    }

    (part1.to_string(), part2.to_string())
}

fn calc_prio(b: u8) -> u8 {
    match b {
        b'a'..=b'z' => b - b'a' + 1,
        b'A'..=b'Z' => b - b'A' + 27,
        _ => panic!("invalid character"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 03;

    #[test]
    fn part1_example() {
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
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("7917", answer.0);
        assert_eq!("2585", answer.1);
    }
}
