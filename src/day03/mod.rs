use arrayvec::ArrayVec;
use log::debug;

use aoc_lib::bitset::Bitset;
use aoc_lib::parse;

pub fn solve(input: &[u8]) -> (String, String) {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    let mut elves: ArrayVec<Bitset, 3> = ArrayVec::new();
    let mut input = input;
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

        let mut lhs_rucksack = Bitset::new();
        let mut rhs_rucksack = Bitset::new();
        for &b in lhs.iter() {
            lhs_rucksack.set((b - b'A') as usize);
        }
        for &b in rhs.iter() {
            rhs_rucksack.set((b - b'A') as usize);
        }
        let items = lhs_rucksack.union(rhs_rucksack);

        let intersection = lhs_rucksack.intersect(rhs_rucksack);
        for k in intersection.iter() {
            let b: u8 = b'A' + k as u8;
            let prio = calc_prio(b);
            debug!("common: {}, prio: {prio}", b as char);
            part1 += prio as i32;
        }
        unsafe {
            elves.push_unchecked(items);
        }
        if elves.len() == 3 {
            let intersection: Bitset = unsafe {
                elves
                    .get_unchecked(0)
                    .intersect(*elves.get_unchecked(1))
                    .intersect(*elves.get_unchecked(2))
            };
            for k in intersection.iter() {
                let b: u8 = b'A' + k as u8;
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
