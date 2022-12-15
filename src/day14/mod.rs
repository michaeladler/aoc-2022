mod rock;

use ahash::AHashSet;
use aoc_lib::{parse, point::Point2D};
use log::{debug, trace};
use rock::Rock;

pub fn solve(input: &[u8]) -> (String, String) {
    let mut input = input;
    let mut rocks = RocksCollection::new();
    while !input.is_empty() {
        let mut points: Vec<Point2D> = Vec::new();
        loop {
            let (rest, x) = parse::positive(input, false).unwrap();
            let (rest, y) = parse::positive(&rest[1..], false).unwrap();
            let (x, y) = (x as i64, y as i64);
            points.push(Point2D { x, y });
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

    let part1;
    {
        let mut sand_points: AHashSet<Point2D> = AHashSet::with_capacity(1024);
        loop {
            let sand = Point2D { x: 500, y: 0 };
            match simulate(sand, &rocks, &sand_points, false) {
                Some(dest) => {
                    debug!(">> simulation finished with dest {:?}", dest);
                    sand_points.insert(dest);
                }
                None => {
                    break;
                }
            }
        }
        part1 = sand_points.len();
    }

    let part2;
    {
        let mut sand_points: AHashSet<Point2D> = AHashSet::with_capacity(1024);
        loop {
            let sand = Point2D { x: 500, y: 0 };
            match simulate(sand, &rocks, &sand_points, true) {
                Some(dest) => {
                    debug!(">> simulation finished with dest {:?}", dest);
                    if !sand_points.insert(dest) {
                        break;
                    }
                }
                None => {
                    break;
                }
            }
        }
        part2 = sand_points.len();
    }

    (part1.to_string(), part2.to_string())
}

#[derive(Debug)]
struct RocksCollection {
    rocks: Vec<Rock>,
    y_max: i64,
}

impl RocksCollection {
    pub fn new() -> Self {
        RocksCollection {
            rocks: Vec::with_capacity(200),
            y_max: i64::MIN,
        }
    }

    pub fn add_rock(&mut self, rock: Rock) {
        if rock.y_max > self.y_max {
            self.y_max = rock.y_max;
        }
        self.rocks.push(rock);
    }

    pub fn contains(&self, p: &Point2D) -> bool {
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
    mut sand: Point2D,
    rocks: &RocksCollection,
    sand_points: &AHashSet<Point2D>,
    check_floor: bool,
) -> Option<Point2D> {
    debug!("simulating {:?}, y_max: {:?}", sand, rocks.y_max);
    let floor = rocks.y_max + 2;
    loop {
        let sand_old = sand;
        let down = Point2D {
            x: sand.x,
            y: sand.y + 1,
        };
        let down_left = Point2D {
            x: down.x - 1,
            y: down.y,
        };
        let down_right = Point2D {
            x: down.x + 1,
            y: down.y,
        };

        if sand_points.contains(&down) || rocks.contains(&down) {
            // the unit of sand attempts to instead move diagonally *one step down and to the left*
            if sand_points.contains(&down_left) || rocks.contains(&down_left) {
                // If that tile is blocked, the unit of sand attempts to instead move diagonally
                // *one step down and to the right*
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
        if !check_floor && sand.y > rocks.y_max {
            debug!(
                "sand reaches out of bounds: {} > {} (y_max)",
                sand.y, rocks.y_max
            );
            return None;
        }
        if check_floor && sand.y == floor {
            debug!("sand hits floor {:?}", sand);
            return Some(sand_old);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 14;

    #[test]
    fn example() {
        let input = b"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

        let solution = solve(input);
        assert_eq!("24", solution.0, "part 1");
        assert_eq!("93", solution.1, "part 2");
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("832", answer.0);
        assert_eq!("27601", answer.1);
    }
}
