use ahash::AHashMap;
use log::debug;
use petgraph::{
    algo::toposort,
    graph::{Graph, NodeIndex},
};

use aoc_lib::parse;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct BinOp {
    lhs: NodeIndex<u32>,
    rhs: NodeIndex<u32>,
    operation: Operation,
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut monkey_values: AHashMap<NodeIndex<u32>, i64> = AHashMap::with_capacity(2048);
    let mut blocked: AHashMap<NodeIndex<u32>, BinOp> = AHashMap::with_capacity(1024);

    let mut monkey_to_num: AHashMap<String, NodeIndex<u32>> = AHashMap::with_capacity(1024);
    let mut num_to_monkey: AHashMap<NodeIndex<u32>, String> = AHashMap::with_capacity(1024);

    let mut root_idx: Option<NodeIndex<u32>> = None;
    let root_str = String::from("root");

    let mut graph: Graph<String, usize> = Graph::new();

    {
        let mut input = input;
        while !input.is_empty() {
            let (rest, from_str) = parse::token(input).unwrap();
            if !monkey_to_num.contains_key(&from_str) {
                let node = graph.add_node(from_str.clone());
                monkey_to_num.insert(from_str.clone(), node);
                num_to_monkey.insert(node, from_str.clone());
            }
            let from = *monkey_to_num.get(&from_str).unwrap();
            if from_str == root_str {
                root_idx = Some(from);
            }

            debug_assert!(rest[0] == b':');
            let rest = &rest[2..];

            let mut pos_eol: usize = 0;
            let mut operation = None;
            //let mut operation :
            for b in rest.iter() {
                match &b {
                    b'\n' => break,
                    b'+' => operation = Some(Operation::Add),
                    b'-' => operation = Some(Operation::Sub),
                    b'*' => operation = Some(Operation::Mul),
                    b'/' => operation = Some(Operation::Div),
                    _ => {}
                }
                pos_eol += 1;
            }
            match operation {
                Some(operation) => {
                    let (rest, lhs) = parse::token(rest).unwrap();
                    if !monkey_to_num.contains_key(&lhs) {
                        let node = graph.add_node(lhs.clone());
                        monkey_to_num.insert(lhs.clone(), node);
                        num_to_monkey.insert(node, lhs.clone());
                    }
                    let lhs = *monkey_to_num.get(&lhs).unwrap();

                    let rhs = parse::token(&rest[3..]).unwrap().1;
                    if !monkey_to_num.contains_key(&rhs) {
                        let node = graph.add_node(rhs.clone());
                        monkey_to_num.insert(rhs.clone(), node);
                        num_to_monkey.insert(node, rhs.clone());
                    }
                    let rhs = *monkey_to_num.get(&rhs).unwrap();

                    let binop = BinOp {
                        lhs,
                        rhs,
                        operation,
                    };
                    blocked.insert(from, binop);
                }
                None => {
                    let x = parse::integer(rest, true).unwrap().1;
                    monkey_values.insert(from, x);
                }
            }

            input = parse::seek_next_line(&rest[pos_eol..]);
        }
    }
    debug!("{:?}", monkey_values);
    debug!("{:?}", blocked);
    debug!("{:?}", monkey_to_num);
    let root_idx = root_idx.unwrap();

    for (&monkey, binop) in blocked.iter() {
        // add edge A -> B if calculating B requires A
        // in the example pppw -> root, sjmn -> root
        graph.add_edge(binop.lhs, monkey, 1);
        graph.add_edge(binop.rhs, monkey, 1);
    }

    let result = toposort(&graph, None).unwrap();
    let mut part1 = None;
    for &monkey in result.iter() {
        if !monkey_values.contains_key(&monkey) {
            debug!("compute: {}", num_to_monkey.get(&monkey).unwrap());
            if let Some(binop) = blocked.remove(&monkey) {
                debug!(
                    "using: {}, {}",
                    num_to_monkey.get(&binop.lhs).unwrap(),
                    num_to_monkey.get(&binop.rhs).unwrap()
                );
                let lhs = monkey_values.get(&binop.lhs).unwrap();
                let rhs = monkey_values.get(&binop.rhs).unwrap();
                let val = match binop.operation {
                    Operation::Add => lhs + rhs,
                    Operation::Sub => lhs - rhs,
                    Operation::Mul => lhs * rhs,
                    Operation::Div => lhs / rhs,
                };
                debug!("result: {val}");
                if monkey == root_idx {
                    part1 = Some(val);
                    break;
                }
                monkey_values.insert(monkey, val);
            }
        }
    }

    let part2: i64 = 42;

    (part1.unwrap().to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 21;

    #[test]
    fn example() {
        let input = b"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

        let answer = solve(input);
        assert_eq!("152", answer.0);
        // assert_eq!("42", answer.1);
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("286698846151845", answer.0);
        //assert_eq!("42", answer.1);
    }
}
