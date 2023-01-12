use std::fmt;

use ahash::AHashMap;
use log::debug;

use aoc_lib::{parse, point::Point2D};

const ELF: u8 = b'#';
const EMPTY: u8 = b'.';

#[derive(Debug)]
struct Grid {
    points: AHashMap<Point2D, u32>,
    y_min: i64,
    y_max: i64,
    x_min: i64,
    x_max: i64,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            points: AHashMap::with_capacity(4096),
            y_min: 0,
            y_max: 0,
            x_min: 0,
            x_max: 0,
        }
    }

    pub fn insert(&mut self, point: Point2D) {
        if point.y < self.y_min {
            self.y_min = point.y;
        } else if point.y > self.y_max {
            self.y_max = point.y;
        }
        if point.x < self.x_min {
            self.x_min = point.x;
        } else if point.x > self.x_max {
            self.x_max = point.x;
        }
        self.points
            .entry(point)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "points:\n")?;
        for y in self.y_min..=self.y_max {
            for x in self.x_min..=self.x_max {
                match self.points.get(&Point2D::new(x, y)) {
                    Some(_count) => write!(f, "{}", ELF as char)?,
                    None => write!(f, "{}", EMPTY as char)?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut grid = Grid::new();
    {
        // origin is top-left of our input
        let mut y: i64 = 0;
        let mut input = input;
        while !input.is_empty() {
            for (x, b) in input.iter().enumerate() {
                if *b == b'\n' {
                    y += 1;
                    input = &input[x..];
                    break;
                }
                if *b == ELF {
                    grid.insert(Point2D::new(x as i64, y));
                }
            }
            input = parse::seek_next_line(input);
        }
    }
    debug!("initial: {:?}", grid);
    debug!("initial: {}", grid);

    let part1: i64 = 42;
    let part2: i64 = 42;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 23;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn example_large() {
        init();

        let input = b"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

        let answer = solve(input);
        assert_eq!("110", answer.0);
        // assert_eq!("42", answer.1);
    }

    #[test]
    #[ignore]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("42", answer.0);
        assert_eq!("42", answer.1);
    }
}
