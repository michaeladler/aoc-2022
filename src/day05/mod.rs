use std::collections::{HashMap, VecDeque};

use log::debug;

use aoc_lib::parse;

pub fn solve(input: &[u8]) -> (String, String) {
    let mut input = input;

    let mut stacks_tmp: HashMap<usize, VecDeque<char>> = HashMap::with_capacity(10);
    while !input.is_empty() {
        for (i, &b) in input.iter().enumerate() {
            if b == b'\n' {
                input = &input[i..];
                break;
            }
            if b >= b'A' && b <= b'Z' {
                let entry = stacks_tmp.entry(i);
                entry
                    .or_insert_with(|| VecDeque::with_capacity(10))
                    .push_front(b as char);
            }
        }
        input = parse::seek_next_line(input);
        if input[0] == b'm' {
            // line starts with 'move'
            break;
        }
    }
    let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(stacks_tmp.len());
    let mut keys: Vec<&usize> = stacks_tmp.keys().collect();
    keys.sort();
    for k in keys {
        stacks.push(stacks_tmp[k].clone()); // TODO: avoid clone
    }
    debug!("stacks: {:?}", stacks);

    // parse movements
    while !input.is_empty() {
        let (rest, count) = parse::positive(input, true).unwrap();
        let (rest, from) = parse::positive(rest, true).unwrap();
        let (rest, to) = parse::positive(rest, true).unwrap();
        debug!("move {count} from {from} to {to}");
        let from = (from - 1) as usize;
        let to = (to - 1) as usize;
        for _i in 0..count {
            // TODO: use unchecked
            let item = stacks.get_mut(from).unwrap().pop_back().unwrap();
            stacks.get_mut(to).unwrap().push_back(item);
        }

        input = parse::seek_next_line(rest);
    }
    debug!("stacks: {:?}", stacks);

    let mut part1_helper: Vec<u8> = Vec::with_capacity(stacks.len());
    for deque in stacks {
        let c = deque.get(deque.len() - 1).unwrap();
        part1_helper.push(*c as u8);
    }

    let part1 = String::from_utf8_lossy(&part1_helper);
    let part2: i64 = 42;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 05;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn example() {
        init();

        let input = b"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
        let solution = solve(&input[..]);
        assert_eq!("CMZ", solution.0);
    }

    #[test]
    #[ignore]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("42", answer.0);
        assert_eq!("42", answer.1);
    }
}
