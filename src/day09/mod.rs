use ahash::AHashSet;
use arrayvec::ArrayVec;
use log::{debug, trace};

use aoc_lib::{parse, point::Point2D};

#[derive(Debug, Copy, Clone)]
enum Direction {
    R,
    D,
    L,
    U,
}

fn move_point(p: &mut Point2D, direction: Direction) {
    match direction {
        Direction::R => {
            p.x += 1;
        }
        Direction::L => {
            p.x -= 1;
        }
        Direction::D => {
            p.y -= 1;
        }
        Direction::U => {
            p.y += 1;
        }
    }
}

fn follow_head(head: &Point2D, tail: &Point2D) -> Point2D {
    let dx = head.x - tail.x;
    let dy = head.y - tail.y;
    let dist_squared = dx * dx + dy * dy;
    if dist_squared > 2 {
        let (delta_x, delta_y) = ((head.x - tail.x).signum(), (head.y - tail.y).signum());
        return Point2D::new(tail.x + delta_x, tail.y + delta_y);
    }
    *tail
}

const MAX_LEN: usize = 10;

#[derive(Debug, PartialEq, Eq)]
struct Rope {
    // head is at pos 0, tail is last pos
    points: ArrayVec<Point2D, MAX_LEN>,
}

impl Rope {
    pub fn new(len: usize) -> Self {
        let mut points: ArrayVec<Point2D, MAX_LEN> = ArrayVec::new();
        for _i in 0..len {
            points.push(Point2D::new(0, 0));
        }
        Self { points }
    }

    pub fn apply(&mut self, direction: Direction) {
        trace!("points before: {:?}", self.points);
        move_point(&mut self.points[0], direction);

        let n = self.points.len();
        for i in 0..n - 1 {
            let new_tail = follow_head(&self.points[i], &self.points[i + 1]);
            self.points[i + 1] = new_tail;
        }
        trace!("points after: {:?}", self.points);
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut rope = Rope::new(2);
    let mut rope2 = Rope::new(10);

    // points visited by tail
    let mut visited: AHashSet<Point2D> = AHashSet::with_capacity(6000);
    let mut visited2: AHashSet<Point2D> = AHashSet::with_capacity(6000);

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
