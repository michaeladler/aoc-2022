use ahash::{AHashMap, AHashSet};
use log::debug;
use std::fmt::Display;

use aoc_lib::graph::AdjacencyMatrix;
use aoc_lib::{bitset::Bitset, parse};

const MAX_VALVE: usize = 26 * 26;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Valve((char, char));

impl Valve {
    pub fn encode(&self) -> usize {
        let unwrapped = self.0;
        let low = (unwrapped.1 as u8 - b'A') as usize;
        let high = (unwrapped.0 as u8 - b'A') as usize * 26;
        low + high
    }
}

impl Display for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0 .0, self.0 .1)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    /// the valve where we are right now
    valve: Valve,
    /// The currently open valves
    open_valves: Bitset,
    /// Current pressure
    pressure: i32,
    /// How much time is left
    minutes_remaining: i32,
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut input = input;
    let mut flow_rates: AHashMap<Valve, i32> = AHashMap::with_capacity(16);
    // one-based graph; graph[1] gives the edges of AA
    let mut graph: AdjacencyMatrix<MAX_VALVE> = AdjacencyMatrix::new();

    while !input.is_empty() {
        let valve = Valve((input[6] as char, input[7] as char));
        let (rest, rate) = parse::positive(&input[23..], false).unwrap();
        if rate > 0 {
            flow_rates.insert(valve, rate as i32);
        }

        let first_upper = rest
            .iter()
            .enumerate()
            .find(|x| (*x.1 as char).is_uppercase())
            .unwrap()
            .0;
        input = &rest[first_upper..];
        while input[0] != b'\n' {
            let other = Valve((input[0] as char, input[1] as char));
            graph.add_edge_undirected(valve.encode(), other.encode(), 1);
            input = &input[2..];
            if input[0] == b',' {
                input = &input[2..];
            }
        }
        input = parse::seek_next_line(input);
    }

    println!("calculating floyd_warshall");
    let dist = graph.floyd_warshall();
    println!("finished floyd_warshall");

    // enumerate non-zero flow rates for bitset
    let mut valve_to_bitset_index: AHashMap<Valve, usize> = AHashMap::with_capacity(16);
    for (i, &valve) in flow_rates.keys().enumerate() {
        valve_to_bitset_index.insert(valve, i);
    }

    let part1 = solve_part1(&dist, &valve_to_bitset_index, &flow_rates);
    let part2: i64 = 42;

    (part1.to_string(), part2.to_string())
}

fn solve_part1(
    dist: &[[i32; MAX_VALVE]],
    valve_to_bitset_index: &AHashMap<Valve, usize>,
    flow_rates: &AHashMap<Valve, i32>,
) -> i32 {
    let mut stack: Vec<State> = Vec::with_capacity(1024);
    let mut seen: AHashSet<State> = AHashSet::with_capacity(1024);
    let start = State {
        valve: Valve(('A', 'A')),
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
        debug!(
            "Your position: {}, pressure: {}",
            state.valve, state.pressure
        );
        // see if we have a new best solution
        if state.pressure > highest_pressure {
            highest_pressure = state.pressure;
            debug!("Found new highest_pressure {highest_pressure}");
        }

        // instead of visiting direct neighbors, we just go directly to valves with flow rate
        // > 0 and which haven't been opened yet; this is much, much faster.
        for (&dest, &idx) in valve_to_bitset_index {
            if !state.open_valves.is_set(idx) {
                let minutes = dist[state.valve.encode()][dest.encode()] + 1;
                let rest = state.minutes_remaining - minutes;
                if rest >= 0 {
                    let mut new_state = state.clone();
                    new_state.valve = dest;
                    new_state.minutes_remaining = rest;
                    new_state.open_valves.set(idx);
                    let rate = flow_rates.get(&dest).unwrap();
                    let pressure = new_state.minutes_remaining * rate;
                    new_state.pressure += pressure;
                    if !seen.contains(&new_state) {
                        debug!("You move to valve {} and open it, which takes {minutes} minutes and adds {}*{rate} = {pressure} pressure", new_state.valve, new_state.minutes_remaining);
                        seen.insert(new_state.clone());
                        stack.push(new_state);
                    }
                }
            }
        }
    }
    highest_pressure
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 16;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
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
    #[ignore]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("1944", answer.0);
        // TODO
        //assert_eq!("42", answer.1);
    }
}
