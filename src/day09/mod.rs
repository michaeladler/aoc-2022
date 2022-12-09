use ahash::AHashSet;
use log::debug;

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
    x: i64,
    y: i64,
}

impl Point {
    /// Euclidean distance squared.
    pub fn distance_squared(&self, other: Point) -> i64 {
        let delta_x = other.x - self.x;
        let delta_y = other.y - self.y;
        return delta_x * delta_x + delta_y * delta_y;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rope {
    head: Point,
    tail: Point,
}

impl Rope {
    pub fn apply(&mut self, direction: Direction) {
        debug!("applying {:?}", direction);
        // move head, then check tail
        match direction {
            Direction::R => {
                self.head.x += 1;
            }
            Direction::L => {
                self.head.x -= 1;
            }
            Direction::D => {
                self.head.y -= 1;
            }
            Direction::U => {
                self.head.y += 1;
            }
        }
        let distance_sq = self.head.distance_squared(self.tail);
        if distance_sq > 2 {
            debug!("tail {:?} follows head {:?}", self.tail, self.head);
            match direction {
                Direction::R => {
                    self.tail.x = self.head.x - 1;
                    self.tail.y = self.head.y;
                }
                Direction::L => {
                    self.tail.x = self.head.x + 1;
                    self.tail.y = self.head.y;
                }
                Direction::D => {
                    self.tail.y = self.head.y + 1;
                    self.tail.x = self.head.x;
                }
                Direction::U => {
                    self.tail.y = self.head.y - 1;
                    self.tail.x = self.head.x;
                }
            }
            debug!("new tail at {:?}", self.tail);
        } else {
            debug!("head at {:?}, tail remains", self.head);
        }
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut rope = Rope {
        head: Point { x: 0, y: 0 },
        tail: Point { x: 0, y: 0 },
    };
    // points visited by tail
    let mut visited: AHashSet<Point> = AHashSet::with_capacity(1024);
    visited.insert(rope.tail);
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
            visited.insert(rope.tail);
        }
        input = parse::seek_next_line(rest);
    }

    let part1 = visited.len();
    let part2: i64 = 42;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 09;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part1_example() {
        init();

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
    }

    #[test]
    #[ignore]
    fn part2_example() {
        let bufs = vec![(b"", 0)];

        for (s, answer) in bufs {
            assert_eq!(answer.to_string(), solve(s).1);
        }
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("5902", answer.0);
        // TODO
        //assert_eq!("42", answer.1);
    }
}
