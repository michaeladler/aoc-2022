use ahash::{AHashMap, AHashSet};
use log::{debug, trace};

use aoc_lib::parse;

type Point = (i32, i32, i32);

pub fn solve(input: &[u8]) -> (String, String) {
    let mut xs: AHashMap<(i32, i32), Vec<i32>> = AHashMap::with_capacity(3000);
    let mut ys: AHashMap<(i32, i32), Vec<i32>> = AHashMap::with_capacity(3000);
    let mut zs: AHashMap<(i32, i32), Vec<i32>> = AHashMap::with_capacity(3000);
    let mut neighbors: AHashMap<Point, AHashSet<Point>> = AHashMap::with_capacity(3000);
    let mut neighbor_count: AHashMap<Point, usize> = AHashMap::with_capacity(3000);

    let (mut x_min, mut y_min, mut z_min) = (i32::MAX, i32::MAX, i32::MAX);
    let (mut x_max, mut y_max, mut z_max) = (i32::MIN, i32::MIN, i32::MIN);

    let mut input = input;
    while input.len() >= 5 {
        let (rest, x) = parse::positive(input, false).unwrap();
        let (rest, y) = parse::positive(&rest[1..], false).unwrap();
        let (rest, z) = parse::positive(&rest[1..], false).unwrap();
        let (x, y, z) = (x as i32, y as i32, z as i32);
        if x < x_min {
            x_min = x;
        }
        if y < y_min {
            y_min = y;
        }
        if z < z_min {
            z_min = z;
        }
        if x > x_max {
            x_max = x;
        }
        if y > y_max {
            y_max = y;
        }
        if z > z_max {
            z_max = z;
        }

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
                trace!("({x1}, {y}, {z}) and ({x2}, {y}, {z}) are neighbors");
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
                trace!("({x}, {y1}, {z}) and ({x}, {y2}, {z}) are neighbors");
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
                trace!("({x}, {y}, {z1}) and ({x}, {y}, {z2}) are neighbors");
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
    let mut part2: usize = 0;

    x_min -= 1;
    y_min -= 1;
    z_min -= 1;
    x_max += 1;
    y_max += 1;
    z_max += 1;
    // bfs: flood fill algorithm
    let n = ((x_max - x_min + 1) * (y_max - y_min + 1) * (z_max - z_min + 1)) as usize;
    let mut seen: AHashSet<Point> = AHashSet::with_capacity(n);
    let mut queue: Vec<Point> = Vec::with_capacity(1024);
    let deltas = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    queue.push((0, 0, 0));
    while let Some((x, y, z)) = queue.pop() {
        for delta in &deltas {
            let candidate = (x + delta.0, y + delta.1, z + delta.2);
            if (candidate.0 >= x_min && candidate.0 <= x_max)
                && (candidate.1 >= y_min && candidate.1 <= y_max)
                && (candidate.2 >= z_min && candidate.2 <= z_max)
                && !seen.contains(&candidate)
            {
                if neighbor_count.contains_key(&candidate) {
                    // point is hit by water, thus exposed
                    part2 += 1;
                } else {
                    seen.insert(candidate);
                    queue.push(candidate);
                }
            }
        }
    }

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 18;

    #[test]
    fn example() {
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
        assert_eq!("64", answer.0, "part 1");
        assert_eq!("58", answer.1, "part 2");
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("4282", answer.0, "part 1");
        assert_eq!("2452", answer.1);
    }
}
