use binary_heap_plus::{BinaryHeap, MinComparator};
use log::{debug, trace};

use aoc_lib::parse;

pub fn solve(input: &[u8]) -> (String, String) {
    let mut top3: BinaryHeap<i32, MinComparator> = BinaryHeap::with_capacity_min(3);
    for _i in 0..3 {
        top3.push(i32::MIN);
    }
    let mut calories: i32 = 0;

    let mut input = input;
    while !input.is_empty() {
        if input.first() == Some(&(b'\n')) {
            debug!("finished processing elf carrying {calories} calories");
            if let Some(mut minimum) = top3.peek_mut() {
                if calories > *minimum {
                    *minimum = calories;
                }
            }
            // reset
            calories = 0;
            input = &input[1..];
        } else if let Some((rest, n)) = parse::integer(input, true) {
            trace!("adding {n} calories to elf");
            calories += n as i32;
            input = parse::seek_next_line(rest);
        }
    }

    let lowest = top3.pop().unwrap();
    let mid = top3.pop().unwrap();
    let highest = top3.pop().unwrap();

    let part1 = highest;
    let part2 = lowest + mid + highest;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 1;

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("68923", answer.0);
        assert_eq!("200044", answer.1);
    }
}
