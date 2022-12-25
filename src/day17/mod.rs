use arrayvec::ArrayVec;
use log::{debug, trace};

const MAX_ITERATIONS: usize = 4_000;
const MAX_HEIGHT: usize = 8_000;
const WIDTH: usize = 9;

const JET_RIGHT: u8 = b'>';
const JET_LEFT: u8 = b'<';

const EMPTY: char = '.';

#[derive(Debug, Clone, Copy)]
enum Item {
    Minus,
    Plus,
    ReverseL,
    Bar,
    Square,
}

impl Item {
    pub fn coords(&self, y: usize, x: usize, out: &mut ArrayVec<(usize, usize), 5>) {
        out.clear();
        match self {
            Item::Minus => {
                // y,x is the leftmost point ('#')
                for i in x..=x + 3 {
                    out.push((y, i));
                }
            }
            Item::Plus => {
                // y,x is the point at the bottom left ('.')
                out.push((y, x + 1));
                for i in x..=x + 2 {
                    out.push((y + 1, i));
                }
                out.push((y + 2, x + 1));
            }
            Item::ReverseL => {
                // y,x is the point at the bottom left ('#')
                for i in x..=x + 2 {
                    out.push((y, i));
                }
                for i in y + 1..=y + 2 {
                    out.push((i, x + 2));
                }
            }
            Item::Bar => {
                // y,x is the point at the bottom ('#')
                for i in 0..=3 {
                    out.push((y + i, x));
                }
            }
            Item::Square => {
                // y,x is the point at the bottom left ('#')
                for i in 0..=1 {
                    for j in 0..=1 {
                        out.push((y + i, x + j));
                    }
                }
            }
        }
    }
}

struct Jet<'a> {
    pattern: &'a [u8],
    idx: usize,
}

impl<'a> Jet<'a> {
    pub fn next(&mut self) -> u8 {
        let result = unsafe { *self.pattern.get_unchecked(self.idx) };
        self.idx = (self.idx + 1) % self.pattern.len();
        result
    }
}

struct Grid {
    /// Bottom is at y=0.
    grid: [[char; WIDTH]; MAX_HEIGHT],
    /// height of the tower
    height: usize,
    /// internal buffer
    coords: ArrayVec<(usize, usize), 5>,
}

impl Grid {
    pub fn new() -> Self {
        let mut grid: [[char; WIDTH]; MAX_HEIGHT] = [[EMPTY; WIDTH]; MAX_HEIGHT];
        for row in grid.iter_mut() {
            row[0] = '|';
            row[WIDTH - 1] = '|';
        }
        grid[0][0] = '+';
        for i in 0..WIDTH {
            grid[0][i] = '-';
        }
        grid[0][0] = '+';
        grid[0][WIDTH - 1] = '+';

        Self {
            grid,
            height: 0,
            coords: ArrayVec::new(),
        }
    }

    /// Simulate the drop of a single item.
    pub fn simulate_item(&mut self, item: Item, jet: &mut Jet) {
        // starting position
        let mut y: usize = self.height + 4;
        let mut x: usize = 3;

        debug!("=== dropping {:?}", item);

        loop {
            trace!(">> item at y={y}, x={x}");
            // apply jet
            let pattern = jet.next();
            let new_x = if pattern == JET_RIGHT {
                x + 1
            } else {
                debug_assert_eq!(JET_LEFT, pattern);
                x - 1
            };
            item.coords(y, new_x, &mut self.coords);
            if !self.has_collision(&self.coords) {
                debug!(
                    "Jet of gas pushes rock {}",
                    if new_x > x { "right" } else { "left" }
                );
                x = new_x;
            } else {
                debug!(
                    "Jet of gas pushes rock {}, but nothing happens",
                    if new_x > x { "right" } else { "left" }
                );
            }

            // apply gravity
            let new_y = y - 1;
            item.coords(new_y, x, &mut self.coords);
            if self.has_collision(&self.coords) {
                self.add(y, x, item, '#');
                debug!(
                    "Rock falls 1 unit, causing it to come to rest. height: {}",
                    self.height
                );
                self.render();
                break;
            }
            y = new_y;
            debug!("Rock falls 1 unit");
        }
    }

    fn has_collision(&self, coords: &[(usize, usize)]) -> bool {
        for c in coords {
            if self.grid[c.0][c.1] != EMPTY {
                return true;
            }
        }
        false
    }

    fn add(&mut self, y: usize, x: usize, item: Item, rep: char) {
        item.coords(y, x, &mut self.coords);
        let mut max_y: usize = 0;
        for c in &self.coords {
            let y = c.0;
            self.grid[y][c.1] = rep;
            if y > max_y {
                max_y = y;
            }
        }
        if max_y > self.height {
            self.height = max_y;
        }
    }

    pub fn render(&self) {
        let rows = &self.grid[0..std::cmp::min(self.height + 7, MAX_HEIGHT)];
        for row in rows.iter().rev() {
            debug!("{}", row.iter().collect::<String>());
        }
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut pattern = input;
    if pattern.last().copied() == Some(b'\n') {
        pattern = &pattern[0..pattern.len() - 1];
    }
    let mut jet = Jet { pattern, idx: 0 };
    let all_items: [Item; 5] = [
        Item::Minus,
        Item::Plus,
        Item::ReverseL,
        Item::Bar,
        Item::Square,
    ];

    let mut grid = Grid::new();
    let mut counter: usize = 0;
    // deltas needed for part 2
    let mut deltas = Vec::with_capacity(8192);
    let mut old_height = 0;
    let mut part1 = 0;
    'outer: loop {
        for &item in &all_items {
            grid.simulate_item(item, &mut jet);
            counter += 1;
            if counter == 2022 {
                part1 = grid.height;
            }
            let delta = grid.height - old_height;
            deltas.push(delta);
            old_height = grid.height;
            if counter == MAX_ITERATIONS {
                break 'outer;
            }
        }
    }

    // part 2
    let (start, cycle_len) = find_cycle(&deltas).unwrap();
    debug!("detected cycle of length {cycle_len} starting at {start}",);
    let h_before: usize = deltas[0..start].iter().sum();
    let h_cycle: usize = deltas[start..start + cycle_len].iter().sum();
    debug!("h_before: {h_before}, h_cycle: {h_cycle}");

    let total: usize = 1_000_000_000_000 - start;
    // how many cycles do fit in
    let cycle_count = total / cycle_len;
    let rem = total % cycle_len;
    debug!("cycle_count: {cycle_count}, rem: {rem}");

    let h_rem: usize = deltas[start..start + rem].iter().sum();

    let part2 = h_before + h_cycle * cycle_count + h_rem;

    (part1.to_string(), part2.to_string())
}

fn find_cycle(x: &[usize]) -> Option<(usize, usize)> {
    for start in 0..x.len() / 2 {
        if let Some(cycle_len) = find_cycle_helper(x, start) {
            return Some((start, cycle_len));
        }
    }
    None
}

/// Find cycle beginning beginning at `start`.
fn find_cycle_helper(x: &[usize], start: usize) -> Option<usize> {
    let x = &x[start..];
    let n = x.len();
    'outer: for cycle_len in 2..n / 2 {
        let mut a1 = 0;
        let mut b1 = cycle_len;
        let mut a2 = cycle_len;
        let mut b2 = 2 * cycle_len;
        loop {
            if b2 > n {
                break;
            }
            if x[a1..b1] != x[a2..b2] {
                debug!("cycle_len {cycle_len} invalidated: {a1}..{b1} vs {a2}..{b2}");
                continue 'outer;
            }
            a1 += cycle_len;
            b1 += cycle_len;
            a2 += cycle_len;
            b2 += cycle_len;
        }
        return Some(cycle_len);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 17;

    #[test]
    fn example() {
        let example = b">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>\n";
        let answer = solve(example);
        assert_eq!("3068", answer.0, "expected 3068 but got {}", answer.0);
        let expected = "1514285714288";
        let actual = answer.1;
        assert_eq!(expected, actual, "expected {expected} but got {actual}");
    }

    #[test]
    fn test_find_cycle() {
        let x = vec![1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        let result = find_cycle_helper(&x, 0);
        assert_eq!(Some(4), result);
    }

    #[test]
    fn test_find_cycle_with_offset() {
        let x = vec![42, 43, 44, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 45];
        let result = find_cycle_helper(&x, 0);
        assert_eq!(None, result);
        let result = find_cycle_helper(&x, 3);
        assert_eq!(Some(4), result);
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("3202", answer.0);
        assert_eq!("1591977077352", answer.1);
    }
}
