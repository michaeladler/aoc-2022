use ahash::AHashSet;
use log::{debug, info};

use aoc_lib::parse;
use aoc_lib::search::search_haystack;

#[derive(Debug)]
struct Blueprint {
    id: u16,
    /// Cost for robots: ore, clay, obsidian, geode
    costs: [Resource; 4],
}

/// The cost array has the same order: ore, clay, obsidian
type Resource = [i16; 3];

impl Blueprint {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            costs: [Default::default(); 4],
        }
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut input = input;
    let mut id: u16 = 0;
    let mut part1: i32 = 0;
    while !input.is_empty() {
        id += 1;
        let mut blueprint = Blueprint::new(id);
        let (rest, cost) = parse_robot(input);
        blueprint.costs[0] = cost;
        let (rest, cost) = parse_robot(rest);
        blueprint.costs[1] = cost;
        let (rest, cost) = parse_robot(rest);
        blueprint.costs[2] = cost;
        let (rest, cost) = parse_robot(rest);
        blueprint.costs[3] = cost;
        debug!("{:?}", blueprint);
        input = parse::seek_next_line(rest);

        let opened = maximize(&blueprint, 24);
        info!("{id}: {opened}");
        part1 += (blueprint.id as i32) * (opened as i32);
    }

    let part2: i64 = 42;

    (part1.to_string(), part2.to_string())
}

fn parse_robot(input: &[u8]) -> (&[u8], Resource) {
    let idx = search_haystack(&b"costs"[..], input).unwrap();
    let mut start = &input[idx..];
    let mut cost: Resource = Default::default();

    loop {
        let (rest, amount) = parse::positive(start, true).unwrap();
        let amount = amount as i16;
        let rest = parse::skip_ws(rest);
        let (rest, token) = parse::token(rest).unwrap();
        match token.as_str() {
            "ore" => {
                cost[0] = amount;
            }
            "clay" => {
                cost[1] = amount;
            }
            "obsidian" => {
                cost[2] = amount;
            }
            _ => panic!("invalid token"),
        }
        if rest[0] == b'.' {
            return (&rest[1..], cost);
        }
        start = rest;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    minutes_left: i16,
    /// ore, clay, obsidian, geode
    robots: [u16; 4],
    /// ore, clay, obsidian, geode
    resources: [i16; 4],
}

impl State {
    pub fn new(minutes_left: i16) -> Self {
        Self {
            minutes_left,
            // we start with 1 ore-robot
            robots: [1, 0, 0, 0],
            resources: [0; 4],
        }
    }

    /// Advance without building anything
    fn tick(&self, count: u16) -> State {
        let count = count as i16;
        debug!("advancing {count} ticks");
        let mut result = self.clone();
        result.minutes_left -= count;
        // Each robot can collect 1 of its resource type per minute.
        for (res, robot_count) in result.resources.iter_mut().zip(self.robots) {
            *res += count * (robot_count as i16);
        }
        result
    }

    pub fn run_till_end(&self) -> State {
        self.tick(self.minutes_left as u16)
    }

    pub fn build_robot(&self, blueprint: &Blueprint, idx: usize) -> Option<State> {
        let cost = &blueprint.costs[idx];

        // same trick as in day 16, i.e. instead of calling `tick()` n times call `tick(n)` once.
        let mut ticks_needed: u16 = 0;

        for ((have_resources, needed_resources), robot_count) in
            self.resources.iter().zip(cost.iter()).zip(self.robots)
        {
            let delta = have_resources - needed_resources;
            if delta < 0 {
                if robot_count == 0 {
                    return None;
                }
                let delta_abs = -delta as u16;
                let mut quot = delta_abs / robot_count;
                if delta_abs % robot_count != 0 {
                    quot += 1;
                }
                ticks_needed = std::cmp::max(quot, ticks_needed);
            }
        }
        ticks_needed += 1;
        if ticks_needed as i16 > self.minutes_left {
            return None;
        }
        let mut result = self.tick(ticks_needed);
        for (res, actual_cost) in result.resources.iter_mut().zip(cost.iter()) {
            *res -= actual_cost;
            debug_assert!(*res >= 0);
        }
        result.robots[idx] += 1;
        match idx {
                0 => debug!(
                    "The new ore-collecting robot is ready; you now have {} of them. (minutes_left: {})",
                    result.robots[idx], result.minutes_left
                ),
                1 => debug!(
                    "The new clay-collecting robot is ready; you now have {} of them. (minutes_left: {})",
                    result.robots[idx], result.minutes_left
                ),
                2 => debug!(
                    "The new obsidian-collecting robot is ready; you now have {} of them. (minutes_left: {})",
                    result.robots[idx], result.minutes_left
                ),
                3 => debug!(
                    "The new geode-cracking robot is ready; you now have {} of them. (state: {:?})",
                    result.robots[idx], result
                ),
                _ => panic!("invalid idx"),
            };
        Some(result)
    }
}

/// The largest number of geodes you could open in `time_left` minutes.
fn maximize(blueprint: &Blueprint, total_minutes: i16) -> i16 {
    debug!("maximize {:?}", blueprint);
    let start = State::new(total_minutes);
    let mut geode_max: State = start.clone();
    // dfs
    let mut queue = Vec::with_capacity(1024);

    queue.push(start.clone());
    let mut seen = AHashSet::with_capacity(1024);
    seen.insert(start);
    while let Some(head) = queue.pop() {
        if head.minutes_left == 0 {
            let val = *head.resources.last().unwrap();
            if val > *geode_max.resources.last().unwrap() {
                debug!("found new max: {} (state: {:?})", val, head);
                geode_max = head;
            }
            continue;
        }
        debug_assert!(head.minutes_left >= 0);
        debug!(
            "== Minute {} ==, geode_max: {}, queue: {}, seen: {}",
            geode_max.resources.last().unwrap(),
            total_minutes - head.minutes_left,
            queue.len(),
            seen.len(),
        );
        debug!("head: {:?}", head);
        // explore neighbors
        for i in 0..4 {
            let neighbor = head
                .build_robot(blueprint, i)
                .unwrap_or_else(|| head.run_till_end());
            if !seen.contains(&neighbor) {
                seen.insert(neighbor.clone());
                queue.push(neighbor);
            }
        }
    }
    *geode_max.resources.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 19;

    #[test]
    fn example() {
        let input = b"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

        let answer = solve(input);
        assert_eq!("33", answer.0, "should be 33 but was {}", answer.0);
    }

    #[test]
    fn example_2() {
        let input = b"Blueprint 18: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 17 obsidian.";
        let answer = solve(input);
        assert_eq!("1", answer.0, "should be 0 but was {}", answer.0);
    }

    #[test]
    fn example_3() {
        let input = b"Blueprint 28: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 2 ore and 8 obsidian.";
        let answer = solve(input);
        assert_eq!("8", answer.0, "should be 8 but was {}", answer.0);
    }

    #[test]
    #[ignore]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("1177", answer.0, "too low");
        //assert_eq!("42", answer.1);
    }
}
