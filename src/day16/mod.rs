use ahash::AHashSet;
use log::debug;
use std::fmt::Display;

use aoc_lib::{bitset::Bitset, parse};

const MAX_VERTICES: usize = 1 + 26 * 26;
const MAX_EDGES: usize = 8;

// our graph is one-based
const NO_EDGE: usize = 0;

const NO_FLOW_RATE: i32 = i32::MIN;

#[derive(Debug, PartialEq, Eq)]
struct Valve((char, char));

impl Valve {
    // one based
    pub fn encode(&self) -> usize {
        debug_assert!(self.0 .0.is_uppercase());
        debug_assert!(self.0 .1.is_uppercase());
        let unwrapped = self.0;
        let low = (unwrapped.1 as u8 - b'A') as usize;
        let high = (unwrapped.0 as u8 - b'A') as usize * 26;
        1 + low + high
    }

    pub fn decode(x: usize) -> Self {
        let x = x - 1;
        let high = b'A' + (x / 26) as u8;
        let low = b'A' + (x % 26) as u8;
        Valve((high as char, low as char))
    }
}

impl Display for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0 .0, self.0 .1)
    }
}

#[derive(Debug)]
struct Graph {
    edges: [[usize; MAX_EDGES]; MAX_VERTICES],
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: [[NO_EDGE; MAX_EDGES]; MAX_VERTICES],
        }
    }

    pub fn add_edge(&mut self, from: &Valve, to: &Valve) {
        let edges = self.edges.get_mut(from.encode()).unwrap();
        for edge in edges.iter_mut() {
            if *edge == NO_EDGE {
                *edge = to.encode();
                return;
            }
        }
        panic!("could not add edge");
    }

    pub fn edges(&self, vertex: usize) -> &[usize] {
        debug_assert!(vertex > 0);
        &self.edges[vertex]
    }

    pub fn degree(&self, vertex: usize) -> usize {
        debug_assert!(vertex > 0);
        let mut deg: usize = 0;
        for edge in self.edges[vertex] {
            if edge == NO_EDGE {
                return deg;
            }
            deg += 1;
        }
        deg
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Graph:")?;
        for (i, edges) in self
            .edges
            .iter()
            .enumerate()
            .skip(1)
            .filter(|(i, _)| self.degree(*i) > 0)
        {
            let from = Valve::decode(i);
            write!(f, "{}{}: ", from.0 .0, from.0 .1)?;
            for &index in edges {
                if index == NO_EDGE {
                    break;
                }
                let to = Valve::decode(index);
                write!(f, "{}{} ", to.0 .0, to.0 .1)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    /// the valve where we are right now
    valve: u16,
    /// The currently open valves
    open_valves: Bitset,
    /// Current pressure
    pressure: i32,
    /// How much time is left
    minutes_remaining: i32,
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut input = input;
    let mut flow_rates: [i32; MAX_VERTICES] = [NO_FLOW_RATE; MAX_VERTICES];
    // one-based graph; graph[1] gives the edges of AA
    let mut graph = Graph::new();

    while !input.is_empty() {
        let valve = Valve((input[6] as char, input[7] as char));
        let (rest, rate) = parse::positive(&input[23..], false).unwrap();
        flow_rates[valve.encode()] = rate as i32;

        let first_upper = rest
            .iter()
            .enumerate()
            .find(|x| (*x.1 as char).is_uppercase())
            .unwrap()
            .0;
        input = &rest[first_upper..];
        while input[0] != b'\n' {
            let other = Valve((input[0] as char, input[1] as char));
            graph.add_edge(&valve, &other);
            input = &input[2..];
            if input[0] == b',' {
                input = &input[2..];
            }
        }
        input = parse::seek_next_line(input);
    }
    debug!("{}", graph);

    // enumerate non-zero flow rates for bitset
    let mut valve_to_bitset_index: [Option<usize>; MAX_VERTICES] = [None; MAX_VERTICES];
    let mut bitset_index_to_valve: [Option<usize>; MAX_VERTICES] = [None; MAX_VERTICES];
    {
        let mut counter: usize = 0;
        for (i, &rate) in flow_rates.iter().enumerate().skip(1) {
            if rate > 0 {
                debug!("{} (offset {}): {rate}", Valve::decode(i), i);
                valve_to_bitset_index[i] = Some(counter);
                bitset_index_to_valve[counter] = Some(i);
                counter += 1;
            }
        }
    }

    // part1: dfs
    let mut stack: Vec<State> = Vec::with_capacity(1024);
    let mut seen: AHashSet<State> = AHashSet::with_capacity(1024);
    let start = State {
        valve: Valve(('A', 'A')).encode() as u16,
        open_valves: Bitset::new(),
        pressure: 0,
        minutes_remaining: 30,
    };
    stack.push(start.clone());
    seen.insert(start);
    let mut highest_pressure: i32 = i32::MIN;
    while let Some(state) = stack.pop() {
        debug!(
            "== Minute {} == (minutes left: {}, pressure: {}, queue size: {})",
            30 - state.minutes_remaining,
            state.minutes_remaining,
            state.pressure,
            stack.len()
        );
        debug!("Your position: {}", Valve::decode(state.valve as usize));
        for idx in state.open_valves.iter() {
            if let Some(valve_idx) = bitset_index_to_valve[idx as usize] {
                let valve = Valve::decode(valve_idx);
                debug!(
                    "Valve {valve} is open, releasing {} pressure",
                    flow_rates[valve_idx]
                );
            }
        }

        if state.minutes_remaining == 0 {
            debug!("No minutes remaining");
            if state.pressure > highest_pressure {
                highest_pressure = state.pressure;
                debug!("Found new highest_pressure {highest_pressure}");
            }
            continue;
        }

        // opening a valve is optional, so we need to consider both possibilities
        if let Some(bitset_idx) = valve_to_bitset_index[state.valve as usize] {
            if !state.open_valves.is_set(bitset_idx) {
                debug!("You open valve {}", Valve::decode(state.valve as usize));
                let mut new_state = state.clone();
                new_state.minutes_remaining -= 1;
                new_state.open_valves.set(bitset_idx);
                new_state.pressure +=
                    new_state.minutes_remaining * flow_rates[state.valve as usize];
                if !seen.contains(&new_state) {
                    seen.insert(new_state.clone());
                    stack.push(new_state);
                }
            }
        }

        for &neighbor_valve in graph
            .edges(state.valve as usize)
            .iter()
            .filter(|&&e| e != NO_EDGE)
        {
            let mut new_state = state.clone();
            new_state.valve = neighbor_valve as u16;
            new_state.minutes_remaining -= 1;
            if !seen.contains(&new_state) {
                debug!("You move to valve {}", Valve::decode(neighbor_valve));
                seen.insert(new_state.clone());
                stack.push(new_state.clone());
            }
        }
    }

    let part1 = highest_pressure;
    let part2: i64 = 42;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 16;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_encode_decode() {
        assert_eq!(Valve(('A', 'A')), Valve::decode(Valve(('A', 'A')).encode()));
        assert_eq!(Valve(('A', 'Z')), Valve::decode(Valve(('A', 'Z')).encode()));
        assert_eq!(Valve(('S', 'T')), Valve::decode(Valve(('S', 'T')).encode()));
        assert_eq!(Valve(('Z', 'Z')), Valve::decode(Valve(('Z', 'Z')).encode()));
    }

    #[test]
    fn part1_example() {
        init();

        let input = b"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

        let answer = solve(input);
        assert_eq!("1651", answer.0);
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("1944", answer.0);
        // TODO
        //assert_eq!("42", answer.1);
    }
}
