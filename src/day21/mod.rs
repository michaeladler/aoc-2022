use std::collections::VecDeque;

use ahash::{AHashMap, AHashSet};
use log::{debug, trace};
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
    let mut num_to_binop: AHashMap<NodeIndex<u32>, BinOp> = AHashMap::with_capacity(1024);

    let mut monkey_to_num: AHashMap<String, NodeIndex<u32>> = AHashMap::with_capacity(1024);

    let mut root_idx: Option<NodeIndex<u32>> = None;
    let root_str = String::from("root");

    let mut humn_idx: Option<NodeIndex<u32>> = None;
    let humn_str = String::from("humn");

    let mut graph: Graph<String, ()> = Graph::new();

    {
        let mut input = input;
        while !input.is_empty() {
            let (rest, from_str) = parse::token(input).unwrap();
            if !monkey_to_num.contains_key(&from_str) {
                let node = graph.add_node(from_str.clone());
                monkey_to_num.insert(from_str.clone(), node);
            }
            let from = *monkey_to_num.get(&from_str).unwrap();
            if from_str == root_str {
                root_idx = Some(from);
            } else if from_str == humn_str {
                humn_idx = Some(from);
            }

            debug_assert!(rest[0] == b':');
            let rest = &rest[2..];

            let mut pos_eol: usize = 0;
            let mut operation = None;
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
                    }
                    let lhs = *monkey_to_num.get(&lhs).unwrap();

                    let rhs = parse::token(&rest[3..]).unwrap().1;
                    if !monkey_to_num.contains_key(&rhs) {
                        let node = graph.add_node(rhs.clone());
                        monkey_to_num.insert(rhs.clone(), node);
                    }
                    let rhs = *monkey_to_num.get(&rhs).unwrap();

                    let binop = BinOp {
                        lhs,
                        rhs,
                        operation,
                    };
                    num_to_binop.insert(from, binop);
                }
                None => {
                    let x = parse::integer(rest, true).unwrap().1;
                    monkey_values.insert(from, x);
                }
            }

            input = parse::seek_next_line(&rest[pos_eol..]);
        }
    }
    let num_to_binop = num_to_binop;
    let root_idx = root_idx.unwrap();
    let humn_idx = humn_idx.unwrap();
    debug!(
        "root_binop: {:?}, humn_idx: {:?}",
        num_to_binop.get(&root_idx).unwrap(),
        humn_idx
    );

    for (&monkey, binop) in num_to_binop.iter() {
        // add edge A -> B if calculating B requires A
        // in the example pppw -> root, sjmn -> root
        graph.add_edge(binop.lhs, monkey, ());
        graph.add_edge(binop.rhs, monkey, ());
    }

    let result = toposort(&graph, None).unwrap();
    let mut part1 = None;
    for &monkey in result.iter() {
        if !monkey_values.contains_key(&monkey) {
            trace!("compute: {}", graph.node_weight(monkey).unwrap());
            if let Some(binop) = num_to_binop.get(&monkey) {
                trace!(
                    "using: {}, {}",
                    graph.node_weight(binop.lhs).unwrap(),
                    graph.node_weight(binop.rhs).unwrap()
                );
                let lhs = monkey_values.get(&binop.lhs).unwrap();
                let rhs = monkey_values.get(&binop.rhs).unwrap();
                let val = match binop.operation {
                    Operation::Add => lhs + rhs,
                    Operation::Sub => lhs - rhs,
                    Operation::Mul => lhs * rhs,
                    Operation::Div => lhs / rhs,
                };
                trace!("result: {val}");
                if monkey == root_idx {
                    part1 = Some(val);
                    break;
                }
                monkey_values.insert(monkey, val);
            }
        }
    }

    // start part2

    // compute path from humn_idx to root_idx
    let mut path = VecDeque::with_capacity(1024);
    // seen contains all nodes which are reachable from humn_idx
    let mut seen: AHashSet<NodeIndex> = AHashSet::with_capacity(1024);
    {
        // bfs
        let mut queue: Vec<NodeIndex> = Vec::with_capacity(1024);
        let mut parent = AHashMap::with_capacity(1024);
        queue.push(humn_idx);
        seen.insert(humn_idx);
        while let Some(current) = queue.pop() {
            if current == root_idx {
                break;
            }
            for nb in graph.neighbors(current) {
                if !seen.contains(&nb) {
                    seen.insert(nb);
                    parent.insert(nb, current);
                    queue.push(nb);
                }
            }
        }
        let mut current = Some(root_idx);
        while let Some(node) = current {
            path.push_front(node);
            current = parent.get(&node).copied();
        }
    }

    // pop root
    path.pop_back();
    let last = *path.back().unwrap();
    debug!("{}", graph.node_weight(last).unwrap());

    let root_binop = num_to_binop.get(&root_idx).unwrap();
    let mut target_value: i64 = if root_binop.lhs == last {
        *monkey_values.get(&root_binop.rhs).unwrap()
    } else {
        *monkey_values.get(&root_binop.lhs).unwrap()
    };

    while let Some(node) = path.pop_back() {
        debug!("target_value: {target_value}");
        if let Some(binop) = num_to_binop.get(&node) {
            // binop must yield target value
            // only one of the two values of binop is comes from humn
            let lhs_human = seen.contains(&binop.lhs);
            let rhs_human = seen.contains(&binop.rhs);
            debug_assert!(!(lhs_human && rhs_human));
            let new_target_value = match binop.operation {
                Operation::Add => {
                    if lhs_human {
                        target_value - *monkey_values.get(&binop.rhs).unwrap()
                    } else {
                        target_value - *monkey_values.get(&binop.lhs).unwrap()
                    }
                }
                Operation::Sub => {
                    if lhs_human {
                        target_value + *monkey_values.get(&binop.rhs).unwrap()
                    } else {
                        *monkey_values.get(&binop.lhs).unwrap() - target_value
                    }
                }
                Operation::Mul => {
                    if lhs_human {
                        target_value / *monkey_values.get(&binop.rhs).unwrap()
                    } else {
                        target_value / *monkey_values.get(&binop.lhs).unwrap()
                    }
                }
                Operation::Div => {
                    if lhs_human {
                        target_value * *monkey_values.get(&binop.rhs).unwrap()
                    } else {
                        *monkey_values.get(&binop.lhs).unwrap() / target_value
                    }
                }
            };
            target_value = new_target_value;
        }
    }

    let part2 = target_value;

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
        assert_eq!("152", answer.0, "part 1");
        assert_eq!("301", answer.1, "part 2");
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("286698846151845", answer.0);
        assert_eq!("3759566892641", answer.1);
    }
}
