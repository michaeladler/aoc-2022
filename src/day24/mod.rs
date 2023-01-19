mod state;

use ahash::AHashMap;
use aoc_lib::{parse, point::Point2D};
use binary_heap_plus::{BinaryHeap, MinComparator};
use log::debug;

use state::Node;

pub fn solve(input: &[u8]) -> (u32, u32) {
    let mut initial: Node = Node::new();

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
                        initial.add_blizz(x, y, *b);
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
        initial.loc = Point2D::new(x_start as i32, 0);
        end = Point2D::new(x_end as i32, y as i32);
    }

    let part1 = a_star(initial, end);
    let part2 = 42;
    (part1, part2)
}

// Based on https://en.wikipedia.org/wiki/A*_search_algorithm#Pseudocode
fn a_star(initial: Node, end: Point2D<i32>) -> u32 {
    const INFINITY: u32 = u32::MAX - 10;

    let start = initial.loc;
    let h = |current: Point2D<i32>| end.manhattan(current) as u32;

    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    let mut g_score: AHashMap<Node, u32> = AHashMap::with_capacity(1024);
    g_score.insert(initial.clone(), 0);

    // The set of discovered nodes that may need to be (re-)expanded.
    let mut open: BinaryHeap<(u32, Node), MinComparator> = BinaryHeap::with_capacity_min(1024);
    open.push((0, initial));

    // For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from start
    // to n currently known.
    let mut came_from: AHashMap<Node, Node> = AHashMap::with_capacity(1024);

    let mut neighbors = Vec::with_capacity(5);
    while let Some((_, current)) = open.pop() {
        if current.loc == end {
            let dist = *g_score.get(&current).unwrap();
            return dist;
        }

        let tmp = current.move_blizzards(end);
        let g_current = g_score.get(&current).copied().unwrap_or(INFINITY);
        // time advances by 1
        let tentative_g_score = g_current + 1;
        debug!("tentative_g_score: {tentative_g_score}");

        neighbors.clear();
        tmp.neighbors(start, end, &mut neighbors);
        neighbors.push(current.loc); // not moving is always an option and leads to a new node in the graph
                                     // however, if a blizzard wants to take this position, we
                                     // *must* move!

        for nb in neighbors
            .iter()
            .filter(|&&candidate| !tmp.is_blizzard(candidate))
        {
            // tentative_gScore is the distance from start to the neighbor through current
            let mut neighbor_state = tmp.clone();
            neighbor_state.loc = *nb;
            if tentative_g_score < g_score.get(&neighbor_state).copied().unwrap_or(INFINITY) {
                debug!("This path to neighbor is better than any previous one. Record it!");
                // TODO: not needed here
                came_from.insert(neighbor_state.clone(), current.clone());
                g_score.insert(neighbor_state.clone(), tentative_g_score);
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
