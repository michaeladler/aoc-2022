mod blizzard;
mod state;

use ahash::AHashMap;
use aoc_lib::{parse, point::Point2D};
use binary_heap_plus::{BinaryHeap, MinComparator};
use log::{debug, trace};
use num_integer::Integer;

use state::Node;

use crate::day24::blizzard::Blizzard;

pub fn solve(input: &[u8]) -> (u32, u32) {
    let mut blizz = Blizzard::new();

    let start: Point2D<i32>;
    let end: Point2D<i32>;
    {
        let mut input = input;
        let x_start = input.iter().enumerate().find(|x| *x.1 == b'.').unwrap().0;
        let mut x_end: usize = 0;
        input = parse::seek_next_line(input);
        let mut y: usize = 0;
        while !input.is_empty() {
            y += 1;
            if input[1] == b'#' {
                x_end = input.iter().enumerate().find(|x| *x.1 == b'.').unwrap().0;
                break;
            }
            for (x, b) in input.iter().enumerate() {
                match b {
                    b'>' | b'<' | b'v' | b'^' => {
                        blizz.add_blizz(x, y, *b);
                    }
                    b'\n' => {
                        input = &input[x..];
                        break;
                    }
                    _ => {}
                }
            }
            input = parse::seek_next_line(input);
        }
        start = Point2D::new(x_start as i32, 0);
        end = Point2D::new(x_end as i32, y as i32);
    }

    debug!("start: {:?}, end: {:?}", start, end);
    let height = end.y - 1;
    let width = end.x - 1;
    let cycle_len = height.lcm(&width) as usize;
    debug!("cycle_len: lcm({width}, {height})={cycle_len}");

    let mut blizz_configs = Vec::with_capacity(cycle_len);
    for _ in 0..cycle_len {
        blizz_configs.push(blizz.clone());
        blizz = blizz.move_blizzards(end);
    }

    let part1 = a_star(start, end, &blizz_configs);
    let part2 = 42;
    (part1, part2)
}

// Based on https://en.wikipedia.org/wiki/A*_search_algorithm#Pseudocode
fn a_star(start: Point2D<i32>, end: Point2D<i32>, blizz_configs: &[Blizzard]) -> u32 {
    const INFINITY: u32 = u32::MAX - 10;

    let h = |current: Point2D<i32>| end.manhattan(current) as u32;

    let initial = Node::new(start);

    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    let mut g_score: AHashMap<Node, u32> = AHashMap::with_capacity(1024);
    g_score.insert(initial, 0);

    // The set of discovered nodes that may need to be (re-)expanded.
    let mut open: BinaryHeap<(u32, Node), MinComparator> = BinaryHeap::with_capacity_min(1024);
    open.push((0, initial));

    // For node n, cameFrom[n] is the node immediately preceding it on the
    // cheapest path from start to n currently known.
    //let mut came_from: AHashMap<Node, Node> = AHashMap::with_capacity(512);

    let mut neighbors = Vec::with_capacity(5);
    while let Some((_, current)) = open.pop() {
        if current.position == end {
            return *g_score.get(&current).unwrap();
        }

        let g_current = g_score.get(&current).copied().unwrap_or(INFINITY);
        // time advances by 1
        let minute = (current.blizz_idx + 1) % blizz_configs.len() as u32;
        let tentative_g_score = g_current + 1;
        trace!("tentative_g_score: {tentative_g_score}");

        neighbors.clear();
        current.neighbors(start, end, &mut neighbors);
        // not moving is an option and leads to a new node in the graph;
        // however, if a blizzard wants to take this position, we must move!
        neighbors.push(current.position);

        let blizz = blizz_configs.get(minute as usize).unwrap();

        for nb in neighbors
            .iter()
            .filter(|&&candidate| !blizz.is_blizzard(candidate))
        {
            // tentative_gScore is the distance from start to the neighbor through current
            let mut neighbor_state = current;
            neighbor_state.blizz_idx = minute;
            neighbor_state.position = *nb;
            if tentative_g_score < g_score.get(&neighbor_state).copied().unwrap_or(INFINITY) {
                trace!("This path to neighbor is better than any previous one. Record it!");
                //came_from.insert(neighbor_state, current);
                g_score.insert(neighbor_state, tentative_g_score);
                let f_score_nb = tentative_g_score + h(*nb);
                if !open.iter().any(|other| other.1 == neighbor_state) {
                    open.push((f_score_nb, neighbor_state));
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 24;

    #[test]
    fn example() {
        let input = b"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

        let answer = solve(input);
        assert_eq!(18, answer.0);
        // assert_eq!("42", answer.1);
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!(257, answer.0);
    }
}
