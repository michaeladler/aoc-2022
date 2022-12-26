use std::fmt::Display;

use ahash::{AHashMap, AHashSet};
use log::debug;

use aoc_lib::parse;

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
// TODO: Move to aoc-lib
pub struct Point3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3D {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Point3D { x, y, z }
    }

    /// Compute the Manhattan distance between `self` and `other`.
    pub fn manhattan(&self, other: &Point3D) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    /// Compute the Euclidean squared distance between `self` and `other`.
    pub fn euclidean_squared(&self, other: &Point3D) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

impl Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{}", self.x, self.y, self.z)
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut xs: AHashMap<(i64, i64), Vec<i64>> = AHashMap::with_capacity(3000);
    let mut ys: AHashMap<(i64, i64), Vec<i64>> = AHashMap::with_capacity(3000);
    let mut zs: AHashMap<(i64, i64), Vec<i64>> = AHashMap::with_capacity(3000);
    // TODO: do we need this neighbors map?
    let mut neighbors: AHashMap<(i64, i64, i64), AHashSet<(i64, i64, i64)>> =
        AHashMap::with_capacity(3000);
    let mut neighbor_count: AHashMap<(i64, i64, i64), usize> = AHashMap::with_capacity(3000);

    let mut input = input;
    while input.len() >= 5 {
        let (rest, x) = parse::positive(input, false).unwrap();
        let (rest, y) = parse::positive(&rest[1..], false).unwrap();
        let (rest, z) = parse::positive(&rest[1..], false).unwrap();
        let (x, y, z) = (x as i64, y as i64, z as i64);

        let entry = xs.entry((y, z)).or_insert_with(|| Vec::with_capacity(256));
        entry.push(x);

        let entry = ys.entry((x, z)).or_insert_with(|| Vec::with_capacity(256));
        entry.push(y);

        let entry = zs.entry((x, y)).or_insert_with(|| Vec::with_capacity(256));
        entry.push(z);

        neighbor_count.insert((x, y, z), 6);

        input = &rest[1..];
    }
    for (&(y, z), x_coords) in xs.iter_mut() {
        x_coords.sort_unstable();
        for (&x1, &x2) in x_coords.iter().zip(x_coords.iter().skip(1)) {
            if (x2 - x1).abs() == 1 {
                debug!("({x1}, {y}, {z}) and ({x2}, {y}, {z}) are neighbors");
                let entry = neighbors
                    .entry((x1, y, z))
                    .or_insert_with(|| AHashSet::with_capacity(256));
                entry.insert((x2, y, z));
                let entry = neighbors
                    .entry((x2, y, z))
                    .or_insert_with(|| AHashSet::with_capacity(256));
                entry.insert((x1, y, z));
            }
        }
    }
    for (&(x, z), y_coords) in ys.iter_mut() {
        y_coords.sort_unstable();
        for (&y1, &y2) in y_coords.iter().zip(y_coords.iter().skip(1)) {
            if (y2 - y1).abs() == 1 {
                debug!("({x}, {y1}, {z}) and ({x}, {y2}, {z}) are neighbors");
                let entry = neighbors
                    .entry((x, y1, z))
                    .or_insert_with(|| AHashSet::with_capacity(256));
                entry.insert((x, y2, z));
                let entry = neighbors
                    .entry((x, y2, z))
                    .or_insert_with(|| AHashSet::with_capacity(256));
                entry.insert((x, y1, z));
            }
        }
    }
    for (&(x, y), z_coords) in zs.iter_mut() {
        z_coords.sort_unstable();
        for (&z1, &z2) in z_coords.iter().zip(z_coords.iter().skip(1)) {
            if (z2 - z1).abs() == 1 {
                debug!("({x}, {y}, {z1}) and ({x}, {y}, {z2}) are neighbors");
                let entry = neighbors
                    .entry((x, y, z1))
                    .or_insert_with(|| AHashSet::with_capacity(256));
                entry.insert((x, y, z2));
                let entry = neighbors
                    .entry((x, y, z2))
                    .or_insert_with(|| AHashSet::with_capacity(256));
                entry.insert((x, y, z1));
            }
        }
    }
    for (point, others) in neighbors {
        debug!("{:?} has {} neighbors: {:?}", point, others.len(), others);
        let entry = neighbor_count.get_mut(&point).unwrap();
        *entry -= others.len();
    }

    let part1: usize = neighbor_count.values().sum();
    let part2: i64 = 42;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 18;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn example() {
        init();

        let input = b"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

        let answer = solve(input);
        assert_eq!("64", answer.0);
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("4282", answer.0);
        // TODO assert_eq!("42", answer.1);
    }
}
