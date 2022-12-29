use ahash::AHashSet;
use arrayvec::ArrayVec;
use log::debug;
use rayon::prelude::*;

use aoc_lib::parse;
use aoc_lib::search::search_haystack;

#[derive(Debug)]
struct Blueprint {
    id: i32,
    /// Cost for robots: ore, clay, obsidian, geode
    costs: [Resource; 4],
}

/// The cost array has the same order: ore, clay, obsidian
type Resource = [i16; 3];

impl Blueprint {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            costs: [Default::default(); 4],
        }
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut input = input;
    let mut id: i32 = 0;
    let mut blueprints: ArrayVec<Blueprint, 30> = ArrayVec::new();
    while !input.is_empty() {
        id += 1;
        let mut bp = Blueprint::new(id);
        let (rest, cost) = parse_robot(input);
        bp.costs[0] = cost;
        let (rest, cost) = parse_robot(rest);
        bp.costs[1] = cost;
        let (rest, cost) = parse_robot(rest);
        bp.costs[2] = cost;
        let (rest, cost) = parse_robot(rest);
        bp.costs[3] = cost;
        blueprints.push(bp);

        input = parse::seek_next_line(rest);
    }

    let part1: i32 = blueprints
        .par_iter()
        .map(|bp| bp.id * maximize(bp, 24))
        .sum();

    let part2: i32 = blueprints[0..std::cmp::min(3, blueprints.len())]
        .par_iter()
        .map(|bp| maximize(bp, 32))
        .product();

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

    pub fn geode_count(&self) -> i16 {
        *self.resources.last().unwrap()
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
        Some(result)
    }
}

/// The largest number of geodes you could open in `time_left` minutes.
fn maximize(blueprint: &Blueprint, total_minutes: i16) -> i32 {
    debug!("maximize {:?}", blueprint);
    let start = State::new(total_minutes);
    let mut geode_max: i16 = 0;
    // dfs
    let mut queue = Vec::with_capacity(1024);

    queue.push(start.clone());
    let mut seen = AHashSet::with_capacity(8192);
    seen.insert(start);
    let mut candidates: ArrayVec<State, 4> = ArrayVec::new();

    // do not build more robots than needed to build another robot, e.g. if the most expensive
    // robot costs 5 ore, do not build more than 5 ore robots.
    let mut max_robots: [u16; 3] = [0; 3];
    for cost_plan in blueprint.costs {
        for (i, &cost) in cost_plan.iter().enumerate() {
            if cost as u16 > max_robots[i] {
                max_robots[i] = cost as u16;
            }
        }
    }

    while let Some(head) = queue.pop() {
        if head.minutes_left == 0 {
            let val = head.geode_count();
            if val > geode_max {
                debug!("found new max: {}", val);
                geode_max = val;
            }
            continue;
        }
        debug_assert!(head.minutes_left >= 0);
        // explore neighbors
        candidates.clear();
        for i in 0..4 {
            // do not build more robots than needed to build another robot, e.g. if the most expensive
            // robot costs 5 ore, do not build more than 5 ore robots.
            if i < max_robots.len() && head.robots[i] + 1 > max_robots[i] {
                continue;
            }

            let neighbor = head
                .build_robot(blueprint, i)
                .unwrap_or_else(|| head.run_till_end());
            // is it worth pursuing neighbor?
            // if we build neighbor and then assume we can build a geode robot every turn, can we
            // beat the current max? if not, cut off this branch.
            let g = neighbor.geode_count();
            let t = neighbor.minutes_left;
            // g + (g+1) + ... + (g+t-1) = t*g + t*(t-1)/2
            if g + t * g + ((t * (t - 1)) / 2) < geode_max {
                continue;
            }

            if !seen.contains(&neighbor) {
                candidates.push(neighbor);
            }
        }

        for neighbor in candidates.iter() {
            seen.insert(neighbor.clone());
            queue.push(neighbor.clone());
        }
    }
    geode_max as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 19;

    #[test]
    fn example_1() {
        let input = b"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
";

        let answer = solve(input);
        assert_eq!("9", answer.0, "should be 9 but was {}", answer.0);
        assert_eq!("56", answer.1, "should be 56 but was {}", answer.1);
    }

    #[test]
    fn example_2() {
        let input = b"Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

        let answer = solve(input);
        assert_eq!("12", answer.0, "should be 12 but was {}", answer.0);
        assert_eq!("62", answer.1, "should be 62 but was {}", answer.1);
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("1177", answer.0);
        assert_eq!("62744", answer.1);
    }
}
