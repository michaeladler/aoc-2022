use std::fmt;

use ahash::{AHashMap, AHashSet};
use log::debug;

use aoc_lib::{parse, point::Point2D};

const ELF: u8 = b'#';
const EMPTY: u8 = b'.';

#[derive(Debug)]
struct Grid {
    points: AHashSet<Point2D>,
    y_min: i64,
    y_max: i64,
    x_min: i64,
    x_max: i64,
}

impl Grid {
    pub fn with_capacity(cap: usize) -> Grid {
        Grid {
            points: AHashSet::with_capacity(cap),
            y_min: i64::MAX,
            y_max: i64::MIN,
            x_min: i64::MAX,
            x_max: i64::MIN,
        }
    }

    pub fn clear(&mut self) {
        self.points.clear();
        self.y_min = i64::MAX;
        self.y_max = i64::MIN;
        self.x_min = i64::MAX;
        self.x_max = i64::MIN;
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
        self.points.insert(point);
    }

    pub fn empty_area(&self) -> usize {
        let total_area = (self.y_max - self.y_min + 1) * (self.x_max - self.x_min + 1);
        let total_area = total_area as usize;
        let k = self.points.len();
        debug!("total_area: {total_area}, points count: {k}");
        total_area - k
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "points: (x_min: {}, y_min: {}, x_max: {}, y_max: {})",
            self.x_min, self.y_min, self.x_max, self.y_max
        )?;
        for y in self.y_min..=self.y_max {
            for x in self.x_min..=self.x_max {
                if self.points.contains(&Point2D::new(x, y)) {
                    write!(f, "{}", ELF as char)?;
                } else {
                    write!(f, "{}", EMPTY as char)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn next(&self) -> Self {
        use Direction::*;
        match self {
            North => South,
            South => West,
            West => East,
            East => North,
        }
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut grid = Grid::with_capacity(32);
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
    debug!("initial: {}", grid);
    let mut grid2 = Grid::with_capacity(grid.points.len());
    let mut idx: u32 = 0;
    let rounds = 10;
    let mut start_orientation = Direction::North;

    let mut propositions: AHashMap<Point2D, Vec<Point2D>> =
        AHashMap::with_capacity(grid.points.len());

    let mut part1: usize = 0;
    for i in 1..=rounds {
        propositions.clear();
        debug!("== start round {} ==", i);
        let (old, new) = if idx == 0 {
            (&grid, &mut grid2)
        } else {
            (&grid2, &mut grid)
        };
        new.clear();

        // first half: each Elf considers the eight positions adjacent to themself
        for &elf in old.points.iter() {
            let (x, y) = (elf.x, elf.y);
            let nw = Point2D::new(x - 1, y - 1);
            let n = Point2D::new(x, y - 1);
            let ne = Point2D::new(x + 1, y - 1);
            let sw = Point2D::new(x - 1, y + 1);
            let s = Point2D::new(x, y + 1);
            let se = Point2D::new(x + 1, y + 1);
            let e = Point2D::new(x + 1, y);
            let w = Point2D::new(x - 1, y);
            let all_neighbors = [nw, n, ne, sw, s, se, e, w];
            let mut has_elf_nearby = false;
            for nb in all_neighbors {
                if old.points.contains(&nb) {
                    has_elf_nearby = true;
                    break;
                }
            }
            if !has_elf_nearby {
                // If no other Elves are in one of those eight positions, the Elf *does not do
                // anything* during this round
                debug!("{:?}: no other elf nearby, not moving", elf);
                new.insert(elf);
                continue;
            }

            // try all orientations to find next position
            let mut current_orient = start_orientation;
            let mut found_proposition = false;
            for _ in 0..4 {
                let neighbors = match current_orient {
                    Direction::North => [n, ne, nw],
                    Direction::South => [s, se, sw],
                    Direction::West => [w, nw, sw],
                    Direction::East => [e, ne, se],
                };

                let mut is_clear = true;
                for nb in &neighbors {
                    if old.points.contains(nb) {
                        is_clear = false;
                        break;
                    }
                }
                if is_clear {
                    let dest = neighbors[0];
                    debug!("{:?} proposes to move {:?} to {dest}", elf, current_orient);
                    found_proposition = true;
                    propositions
                        .entry(dest)
                        .and_modify(|xs| xs.push(elf))
                        .or_insert_with(|| vec![elf]);
                    break;
                }
                current_orient = current_orient.next();
            }
            if !found_proposition {
                debug!("found no proposition for {:?}, it must stay put", elf);
                new.insert(elf);
            }
        }

        // second half: each Elf moves to their proposed destination tile if they were the *only*
        // Elf to propose moving to that position
        for (dest, elves) in propositions.iter() {
            debug!("{:?} wants to be visited by: {:?}", dest, elves);
            match elves.len() {
                0 => unreachable!(),
                1 => {
                    debug!("{:?} is now inhabitated", dest);
                    new.insert(*dest);
                }
                _ => {
                    debug!("it's not possible due to overbooking, every elf must stay");
                    for &elf in elves {
                        new.insert(elf);
                    }
                }
            }
        }

        debug!("== end of round {} ==", i);
        debug!(
            "old_len={}, new_len={}, {}",
            old.points.len(),
            new.points.len(),
            new
        );

        // sanity check: no elf got lost
        debug_assert!(old.points.len() == new.points.len());
        if i == 10 {
            part1 = new.empty_area();
        }

        // update
        start_orientation = start_orientation.next();
        idx = 1 - idx;
    }

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
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("4070", answer.0);
        //assert_eq!("42", answer.1);
    }
}
