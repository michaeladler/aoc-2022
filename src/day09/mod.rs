use ahash::AHashSet;
use arrayvec::ArrayVec;
use log::{debug, trace};

use aoc_lib::parse;

#[derive(Debug, Copy, Clone)]
enum Direction {
    R,
    D,
    L,
    U,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn step(&mut self, direction: Direction) {
        match direction {
            Direction::R => {
                self.x += 1;
            }
            Direction::L => {
                self.x -= 1;
            }
            Direction::D => {
                self.y -= 1;
            }
            Direction::U => {
                self.y += 1;
            }
        }
    }

    pub fn follow_head(&self, head: Self) -> Point {
        let dx = head.x - self.x;
        let dy = head.y - self.y;
        let dist_squared = dx * dx + dy * dy;
        if dist_squared > 2 {
            let (delta_x, delta_y) = ((head.x - self.x).signum(), (head.y - self.y).signum());
            return Point {
                x: self.x + delta_x,
                y: self.y + delta_y,
            };
        }
        *self
    }
}

const MAX_LEN: usize = 10;

#[derive(Debug, PartialEq, Eq)]
struct Rope {
    // head is at pos 0, tail is last pos
    points: ArrayVec<Point, MAX_LEN>,
}

impl Rope {
    pub fn new(len: usize) -> Self {
        let mut points: ArrayVec<Point, MAX_LEN> = ArrayVec::new();
        for _i in 0..len {
            points.push(Point { x: 0, y: 0 });
        }
        Self { points }
    }

    pub fn apply(&mut self, direction: Direction) {
        trace!("points before: {:?}", self.points);
        self.points[0].step(direction);

        let n = self.points.len();
        for i in 0..n - 1 {
            let (head, tail) = (self.points[i], self.points[i + 1]);
            let new_tail = tail.follow_head(head);
            self.points[i + 1] = new_tail;
        }
        trace!("points after: {:?}", self.points);
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut rope = Rope::new(2);
    let mut rope2 = Rope::new(10);

    // points visited by tail
    let mut visited: AHashSet<Point> = AHashSet::with_capacity(6000);
    let mut visited2: AHashSet<Point> = AHashSet::with_capacity(6000);

    visited.insert(*rope.points.last().unwrap());
    visited2.insert(*rope2.points.last().unwrap());

    let mut input = input;
    while !input.is_empty() {
        let (rest, steps) = parse::positive(&input[2..], false).unwrap();
        let direction = match input[0] {
            b'R' => Direction::R,
            b'D' => Direction::D,
            b'L' => Direction::L,
            b'U' => Direction::U,
            _ => {
                panic!("unexpected direction");
            }
        };
        debug!("=== applying {:?} {steps}", direction);
        for _i in 0..steps {
            rope.apply(direction);
            rope2.apply(direction);

            visited.insert(*rope.points.last().unwrap());
            visited2.insert(*rope2.points.last().unwrap());
        }
        debug!("visited2 {} rope2: {:?}", visited2.len(), rope2);
        input = parse::seek_next_line(rest);
    }

    let part1 = visited.len();
    let part2 = visited2.len();

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 09;

    #[test]
    fn part1_example() {
        let input = b"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
        let solution = solve(&input[..]);
        assert_eq!("13", solution.0);
        assert_eq!("1", solution.1);
    }

    #[test]
    fn part2_example() {
        let input = b"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
        let solution = solve(&input[..]);
        assert_eq!("36", solution.1);
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("5902", answer.0);
        assert_eq!("2445", answer.1);
    }
}
