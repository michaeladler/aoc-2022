use ahash::AHashMap;
use binary_heap_plus::BinaryHeap;
use log::debug;
use std::fmt;

const MAX_ROWS: usize = 163;
const MAX_COLS: usize = 163;
const EMPTY: char = '.';

const START: char = 'S';
const START_ELEVATION: char = 'a';

const END: char = 'E';
const END_ELEVATION: char = 'z';

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
}

struct Grid {
    grid: [[char; MAX_COLS]; MAX_ROWS],
    rows: usize,
    cols: usize,
    start: Point,
    end: Point,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            grid: [[EMPTY; MAX_COLS]; MAX_ROWS],
            rows: 0,
            cols: 0,
            start: Point { y: 0, x: 0 },
            end: Point { y: 0, x: 0 },
        }
    }
    pub fn set(&mut self, y: usize, x: usize, c: char) {
        self.grid[y][x] = c;
        self.rows = std::cmp::max(self.rows, y + 1);
        self.cols = std::cmp::max(self.cols, x + 1);
    }

    pub fn edges(&self, y: usize, x: usize, neighbors: &mut Vec<Point>) {
        neighbors.clear();
        let directions: [(i32, i32); 4] = [
            (0, -1), // top
            (1, 0),  // right
            (0, 1),  // bottom
            (-1, 0), // left
        ];
        let elevation = unsafe { *self.grid.get_unchecked(y).get_unchecked(x) };
        let x = x as i32;
        let y = y as i32;
        for (dx, dy) in directions {
            let new_x = x + dx;
            let new_y = y + dy;
            if new_x >= 0 && new_y >= 0 && new_x < (self.cols as i32) && new_y < (self.rows as i32)
            {
                let neighbor_elevation = self.get(new_y as usize, new_x as usize);
                // the elevation of the destination square can be *at most one higher* than the elevation of your current square
                if neighbor_elevation <= elevation
                    || (elevation as u8) + 1 == (neighbor_elevation as u8)
                {
                    neighbors.push(Point { x: new_x, y: new_y });
                }
            }
        }
    }

    fn get(&self, y: usize, x: usize) -> char {
        unsafe { *self.grid.get_unchecked(y).get_unchecked(x) }
    }

    pub fn shortest_distance(&self, start: Point, end: Point) -> Option<i64> {
        const INFINITY: i64 = i64::MAX;
        let mut queue = BinaryHeap::with_capacity_min(self.rows * self.cols);
        // TODO: use an array instead, offset = y * cols + x
        let mut dist: AHashMap<Point, i64> = AHashMap::with_capacity(self.rows * self.cols);
        let mut prev: AHashMap<Point, Point> = AHashMap::with_capacity(self.rows * self.cols);

        {
            // init distances
            dist.insert(start, 0);
            for (y, row) in self.grid.iter().enumerate() {
                for (x, _) in row.iter().enumerate() {
                    let p = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                    if p != start {
                        dist.insert(p, INFINITY);
                    }
                }
            }
        }

        let edge_weight: i64 = 1;
        let mut neighbors = Vec::with_capacity(4);
        queue.push((0, start));
        // pop point p closest to start; its distance is d.
        while let Some((_d, u)) = queue.pop() {
            self.edges(u.y as usize, u.x as usize, &mut neighbors);
            for &v in neighbors.iter() {
                let alt = dist.get(&u).unwrap() + edge_weight;
                if alt < *dist.get(&v).unwrap() {
                    debug!(
                        "found new shortest path to row {}, col {} with distance {}",
                        v.y, v.x, alt
                    );
                    // found shorter path
                    dist.insert(v, alt);
                    prev.insert(v, u);
                    queue.push((alt, v));
                }
            }
        }
        dist.get(&end).cloned()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Grid (rows: {}, cols: {}, start: {:?}, dest: {:?}):",
            self.rows, self.cols, self.start, self.end
        )?;
        for row in self.grid.iter().take(self.rows) {
            for col in row.iter().take(self.cols) {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let grid = parse_input(input);
    debug!("{}", grid);
    let mut alt_starts = Vec::with_capacity(64);
    for (y, row) in grid.grid.iter().enumerate().take(grid.rows) {
        for (x, &elevation) in row.iter().enumerate().take(grid.cols) {
            if elevation == 'a' {
                let p = Point {
                    x: x as i32,
                    y: y as i32,
                };
                if p != grid.start {
                    alt_starts.push(p);
                }
            }
        }
    }

    let part1 = grid.shortest_distance(grid.start, grid.end).unwrap();
    let mut part2 = part1;
    for start in alt_starts {
        let d = grid.shortest_distance(start, grid.end).unwrap();
        if d < part2 {
            part2 = d;
        }
    }
    (part1.to_string(), part2.to_string())
}

fn parse_input(input: &[u8]) -> Grid {
    let mut grid = Grid::new();
    let mut row: usize = 0;
    let mut input = input;
    while !input.is_empty() {
        for (col, &b) in input.iter().enumerate() {
            match b as char {
                '\n' => {
                    input = &input[col..];
                    input = &input[std::cmp::min(1, input.len())..];
                    break;
                }
                START => {
                    grid.start = Point {
                        y: row as i32,
                        x: col as i32,
                    };
                    grid.set(row, col, START_ELEVATION);
                }
                END => {
                    grid.end = Point {
                        y: row as i32,
                        x: col as i32,
                    };
                    grid.set(row, col, END_ELEVATION);
                }
                _ => grid.set(row, col, b as char),
            }
        }
        row += 1;
    }
    debug_assert!(grid.start != grid.end);
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 12;

    const EXAMPLE: &[u8; 45] = b"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_edges() {
        let grid = parse_input(EXAMPLE);
        let mut neighbors = Vec::with_capacity(4);
        grid.edges(0, 0, &mut neighbors);
        assert_eq!(neighbors, vec![Point { x: 1, y: 0 }, Point { x: 0, y: 1 },])
    }

    #[test]
    fn example() {
        let solution = solve(EXAMPLE);
        assert_eq!("31", solution.0);
        assert_eq!("29", solution.1);
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("534", answer.0);
        assert_eq!("525", answer.1);
    }
}
