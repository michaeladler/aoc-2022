mod rock;

use ahash::AHashSet;
use aoc_lib::parse;
use log::{debug, trace};
use rock::{Point, Rock};

pub fn solve(input: &[u8]) -> (String, String) {
    let mut part1 = 0;

    let mut input = input;
    let mut rocks = RocksCollection::new();
    while !input.is_empty() {
        let mut points: Vec<Point> = Vec::new();
        loop {
            let (rest, x) = parse::positive(input, false).unwrap();
            let (rest, y) = parse::positive(&rest[1..], false).unwrap();
            let (x, y) = (x as i32, y as i32);
            points.push(Point { x, y });
            if rest[0] == b'\n' {
                trace!("{:?}", points);
                input = rest;
                break;
            }
            input = &rest[4..];
        }
        rocks.add_rock(Rock::new(points));

        input = parse::seek_next_line(input);
    }

    let mut sand_points: AHashSet<Point> = AHashSet::with_capacity(1024);
    for i in 0..u64::MAX {
        let sand = Point { x: 500, y: 0 };
        match simulate(sand, &rocks, &sand_points) {
            Some(dest) => {
                debug!(">> simulation finished with dest {:?}", dest);
                sand_points.insert(dest);
                debug!("sand_points: {:?}", sand_points);
            }
            None => {
                part1 = i;
                break;
            }
        }
    }

    let part2: i64 = 42;

    (part1.to_string(), part2.to_string())
}

#[derive(Debug)]
struct RocksCollection {
    rocks: Vec<Rock>,
    y_max: i32,
}

impl RocksCollection {
    pub fn new() -> Self {
        RocksCollection {
            rocks: Vec::with_capacity(200),
            y_max: i32::MIN,
        }
    }

    pub fn add_rock(&mut self, rock: Rock) {
        if rock.y_max > self.y_max {
            self.y_max = rock.y_max;
        }
        self.rocks.push(rock);
    }

    pub fn contains(&self, p: &Point) -> bool {
        for r in &self.rocks {
            if r.contains(p) {
                return true;
            }
        }
        false
    }
}

// Let it snow... Returns destination of sand.
fn simulate(
    mut sand: Point,
    rocks: &RocksCollection,
    sand_points: &AHashSet<Point>,
) -> Option<Point> {
    debug!("simulating {:?}, y_max: {:?}", sand, rocks.y_max);
    loop {
        let down = Point {
            x: sand.x,
            y: sand.y + 1,
        };
        if sand_points.contains(&down) || rocks.contains(&down) {
            // the unit of sand attempts to instead move diagonally *one step down and to the left*
            let down_left = Point {
                x: down.x - 1,
                y: down.y,
            };
            if sand_points.contains(&down_left) || rocks.contains(&down_left) {
                // If that tile is blocked, the unit of sand attempts to instead move diagonally
                // *one step down and to the right*
                let down_right = Point {
                    x: down.x + 1,
                    y: down.y,
                };
                if sand_points.contains(&down_right) || rocks.contains(&down_right) {
                    //If all three possible destinations are blocked, the unit of sand *comes to
                    //rest* and no longer moves
                    return Some(sand);
                } else {
                    sand = down_right;
                }
            } else {
                sand = down_left;
            }
        } else {
            sand = down;
        }
        if sand.y > rocks.y_max {
            debug!(
                "sand reaches out of bounds: {} > {} (y_max)",
                sand.y, rocks.y_max
            );
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 14;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part1_example() {
        init();

        let input = b"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

        let solution = solve(input);
        assert_eq!("24", solution.0);
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("832", answer.0);
        // TODO
        //assert_eq!("42", answer.1);
    }
}
