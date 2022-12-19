use ahash::AHashMap;
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

    let dist = graph.floyd_warshall();

    // enumerate non-zero flow rates for bitset
    let mut valve_to_bitset_index: AHashMap<Valve, usize> = AHashMap::with_capacity(16);
    for (i, &valve) in flow_rates.keys().enumerate() {
        valve_to_bitset_index.insert(valve, i);
    }

    let facts = Facts {
        dist,
        valve_to_bitset_index,
        flow_rates,
    };

    let start = Valve(('A', 'A'));

    let mut cache = AHashMap::with_capacity(1024);
    let part1 = calc_pressure(&facts, &mut cache, start, 30, Bitset::new(), 0);

    let mut cache = AHashMap::with_capacity(1024);
    let part2 = calc_pressure(&facts, &mut cache, start, 26, Bitset::new(), 1);

    (part1.to_string(), part2.to_string())
}

struct Facts {
    dist: [[i32; MAX_VALVE]; MAX_VALVE],
    // TODO: get rid of this, substracting a constant should be enough
    valve_to_bitset_index: AHashMap<Valve, usize>,
    // TODO: use array
    flow_rates: AHashMap<Valve, i32>,
}

// Dynamic programming approach
fn calc_pressure(
    facts: &Facts,
    // cached solutions
    cache: &mut AHashMap<(Valve, i32, Bitset, u16), i32>,
    // cache these
    start: Valve,
    minutes_left: i32,
    open_valves: Bitset,
    players_remaining: u16, // 1 then 0
) -> i32 {
    debug!(
        "solving: {start}, {minutes_left}, open_valves: {:?}",
        open_valves
    );

    let mut best = 0;

    // instead of visiting direct neighbors, we just go directly to valves with flow rate
    // > 0 and which haven't been opened yet; this is much, much faster.
    for (&dest, &idx) in facts
        .valve_to_bitset_index
        .iter()
        .filter(|x| !open_valves.is_set(*x.1))
    {
        // it takes 1 minute to open the valve after getting there
        let duration = facts.dist[start.encode()][dest.encode()] + 1;
        let new_minutes_left = minutes_left - duration;
        if new_minutes_left > 0 {
            let rate = facts.flow_rates.get(&dest).unwrap();
            let pressure = new_minutes_left * rate;
            let mut new_open_valves = open_valves;
            new_open_valves.set(idx);

            let cache_key = (dest, new_minutes_left, new_open_valves, players_remaining);
            let val = if let Some(cached_value) = cache.get(&cache_key) {
                pressure + cached_value
            } else {
                let pressure_subproblem = calc_pressure(
                    facts,
                    cache,
                    dest,
                    new_minutes_left,
                    new_open_valves,
                    players_remaining,
                );
                cache.insert(cache_key, pressure_subproblem);
                pressure + pressure_subproblem
            };
            if val > best {
                best = val;
            }
        }
    }

    if players_remaining > 0 {
        // now it is the next player's turn
        let best_other_player = calc_pressure(
            facts,
            cache,
            Valve(('A', 'A')),
            26,
            open_valves,
            players_remaining - 1,
        );
        if best_other_player > best {
            best = best_other_player;
        }
    }

    debug!(
        "{start}, {minutes_left}, {:?} has max pressure {best}",
        open_valves
    );
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 16;

    #[test]
    fn example() {
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
        assert_eq!("1651", answer.0, "part 1");
        assert_eq!(
            "1707", answer.1,
            "part 2 should be 1707 but was {}",
            answer.1
        );
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!(
            "1944", answer.0,
            "part 1: should be 1944 but was {}",
            answer.0
        );
        assert_eq!("2679", answer.1);
    }
}
