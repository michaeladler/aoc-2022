use arrayvec::ArrayVec;
use log::debug;
use serde_json::Value;
use std::cmp::Ordering;

use aoc_lib::parse;

pub fn solve(input: &[u8]) -> (String, String) {
    let mut part1 = 0;
    let mut index: u64 = 0;
    let mut all_packets: ArrayVec<Value, 512> = ArrayVec::new();
    let mut input = input;
    while !input.is_empty() {
        index += 1;
        debug!("=== {index} ===");
        let pos_eol = input
            .iter()
            .enumerate()
            .find(|(_i, &x)| x == b'\n')
            .unwrap()
            .0;
        let lhs: Value = serde_json::from_slice(&input[0..pos_eol]).unwrap();
        input = &input[pos_eol + 1..];

        let pos_eol = input
            .iter()
            .enumerate()
            .find(|(_i, &x)| x == b'\n')
            .unwrap()
            .0;
        let rhs: Value = serde_json::from_slice(&input[0..pos_eol]).unwrap();

        let result = compare_values(&lhs, &rhs);
        debug!("comparison result: {:?}", result);
        if result == Ordering::Less {
            part1 += index;
        }

        all_packets.push(lhs);
        all_packets.push(rhs);

        input = &input[pos_eol..];
        input = parse::seek_next_line(input);
        input = parse::seek_next_line(input);
    }

    let divider1: Value = serde_json::from_slice(b"[[2]]").unwrap();
    let divider2: Value = serde_json::from_slice(b"[[6]]").unwrap();

    all_packets.push(divider1.clone());
    all_packets.push(divider2.clone());
    all_packets.sort_unstable_by(|a, b| compare_values(a, b));

    let mut part2: usize = 1;
    for (i, packet) in all_packets.iter().enumerate() {
        if *packet == divider1 || *packet == divider2 {
            part2 *= i + 1;
        }
    }

    (part1.to_string(), part2.to_string())
}

fn compare_values(lhs: &Value, rhs: &Value) -> Ordering {
    compare_values_helper(lhs, rhs, 0)
}

fn compare_values_helper(lhs: &Value, rhs: &Value, level: usize) -> Ordering {
    let indent = level * 2;
    let mut ws = String::with_capacity(indent);
    for _i in 0..indent {
        ws.push(' ');
    }
    debug!("{ws}- Compare {:?} vs {:?}", lhs, rhs);

    match (lhs, rhs) {
        (Value::Number(a), Value::Number(b)) => {
            // If *both values are integers*, the *lower integer* should come first
            a.as_i64().unwrap().cmp(&b.as_i64().unwrap())
        }
        (Value::Array(xs), Value::Array(ys)) => {
            // If *both values are lists*, compare the first value of each list, then the second
            // value, and so on
            for (x, y) in xs.iter().zip(ys.iter()) {
                match compare_values_helper(x, y, level + 1) {
                    Ordering::Less => {
                        return Ordering::Less;
                    }
                    Ordering::Greater => {
                        return Ordering::Greater;
                    }
                    Ordering::Equal => {
                        // continue with rest of list
                    }
                }
            }
            // check who ran out of items first
            return xs.len().cmp(&ys.len());
        }
        (Value::Number(a), Value::Array(_)) => {
            let alist = Value::Array(vec![Value::Number(a.clone())]);
            debug!(
                "Mixed types; convert left to {:?} and retry comparison",
                alist
            );
            return compare_values_helper(&alist, rhs, level + 1);
        }
        (Value::Array(_), Value::Number(b)) => {
            let blist = Value::Array(vec![Value::Number(b.clone())]);
            debug!(
                "Mixed types; convert right to {:?} and retry comparison",
                blist
            );
            return compare_values_helper(lhs, &blist, level + 1);
        }
        _ => {
            panic!("unsupported JSON value");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 13;

    #[test]
    fn example() {
        let input = b"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

        let solution = solve(input);
        assert_eq!("13", solution.0);
        assert_eq!("140", solution.1);
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("6484", answer.0);
        assert_eq!("19305", answer.1);
    }
}
