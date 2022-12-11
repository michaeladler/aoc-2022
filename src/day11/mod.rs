use std::collections::VecDeque;

use arrayvec::ArrayVec;
use log::debug;

use aoc_lib::parse;

type N = i32;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(N),
    Mult(N),
    Square,
}

impl Operation {
    pub fn apply(&self, value: N) -> N {
        use Operation::*;
        match self {
            Add(n) => value + n,
            Mult(n) => value * n,
            Square => value * value,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<N>,
    operation: Operation,
    divisor: N,
    dest_true: usize,
    dest_false: usize,
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut monkeys: ArrayVec<Monkey, 8> = ArrayVec::new();
    let mut input = input;
    while !input.is_empty() {
        if input[0] == b'M' {
            // parse monkey

            // parse items
            input = parse::seek_next_line(input);
            let pos_eol = input
                .iter()
                .enumerate()
                .find(|(_i, &x)| x == b'\n')
                .unwrap()
                .0;
            let items: VecDeque<N> = input[18..pos_eol]
                .split(|&b| b == b',')
                .map(|s| {
                    String::from_utf8_lossy(s)
                        .trim_start()
                        .parse::<N>()
                        .unwrap()
                })
                .collect();
            debug!("items: {:?}", items);

            // parse operation
            input = &input[pos_eol + 1..];
            input = &input[23..];
            let operation: Operation = match input[0] {
                b'*' => {
                    if input[2] == b'o' {
                        Operation::Square
                    } else {
                        let (rest, val) = parse::positive(&input[2..], false).unwrap();
                        input = rest;
                        Operation::Mult(val as N)
                    }
                }
                b'+' => {
                    let (rest, val) = parse::positive(&input[2..], false).unwrap();
                    input = rest;
                    Operation::Add(val as N)
                }
                _ => panic!("unexpected operation"),
            };
            debug!("operation: {:?}", operation);

            // parse test
            input = parse::seek_next_line(input);
            let (rest, test) = parse::positive(&input[21..], false).unwrap();
            input = rest;
            debug!("test: {test}");

            // dest_true
            input = parse::seek_next_line(input);
            let (rest, dest_true) = parse::positive(&input[29..], false).unwrap();
            input = rest;
            debug!("dest_true: {dest_true}");

            // dest_false
            input = parse::seek_next_line(input);
            let (rest, dest_false) = parse::positive(&input[30..], false).unwrap();
            input = rest;
            debug!("dest_false: {dest_false}");

            let monkey = Monkey {
                items,
                operation,
                divisor: test as N,
                dest_true: dest_true as usize,
                dest_false: dest_false as usize,
            };
            monkeys.push(monkey);
        }

        input = parse::seek_next_line(input);
    }
    debug!("monkeys: {:?}", monkeys);
    let mut inspection_counters: ArrayVec<u64, 8> = ArrayVec::new();
    for _i in 0..monkeys.len() {
        inspection_counters.push(0);
    }

    let max_rounds = 20;
    let n = monkeys.len();
    for round_number in 1..=max_rounds {
        debug!("=== Round {round_number}");
        for i in 0..n {
            debug!("Monkey {i}:");
            let item_count = unsafe { monkeys.get_unchecked(i).items.len() };
            for _j in 0..item_count {
                let sender = unsafe { monkeys.get_unchecked_mut(i) };
                let level = sender.items.pop_front().unwrap();
                debug!("  Monkey inspects an item with a worry level of {level}.");
                let counter = unsafe { inspection_counters.get_unchecked_mut(i) };
                *counter += 1;

                let new_level = sender.operation.apply(level);
                debug!("    Worry level increases to {new_level}.");
                let new_level = new_level / 3;
                debug!(
                    "    Monkey gets bored with item. Worry level is divided by 3 to {new_level}."
                );
                let dest = if new_level % sender.divisor == 0 {
                    debug!(
                        "    Current worry level is divisible by {}.",
                        sender.divisor
                    );
                    sender.dest_true
                } else {
                    debug!(
                        "    Current worry level is not divisible by {}.",
                        sender.divisor
                    );
                    sender.dest_false
                };
                debug!("    Item with worry level {new_level} is thrown to monkey {dest}.");
                let receiver = unsafe { monkeys.get_unchecked_mut(dest) };
                receiver.items.push_back(new_level);
            }
        }
    }

    inspection_counters.sort_unstable();
    let n = inspection_counters.len();

    let part1 = inspection_counters[n - 1] * inspection_counters[n - 2];
    let part2: i64 = 42;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 11;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part1_example() {
        init();

        let input = b"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
        let solution = solve(input);
        assert_eq!("10605", solution.0);
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
        assert_eq!("90882", answer.0);
        // TODO
        //assert_eq!("42", answer.1);
    }
}
