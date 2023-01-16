use ahash::AHashSet;
use arrayvec::ArrayVec;
use log::{debug, trace};

use aoc_lib::{
    interval::{merge_intervals, ClosedInterval},
    parse,
    point::Point2D,
};

#[derive(Debug)]
struct Sensor(Point2D);

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Beacon(Point2D);

pub fn solve(input: &[u8]) -> (String, String) {
    let pairings = parse_input(input);
    let part1 = count_illegal_beacon_locs(&pairings, 2000000).0;
    let part2 = tuning_frequency(&pairings, 4000000);
    (part1.to_string(), part2.to_string())
}

fn parse_input(input: &[u8]) -> ArrayVec<(Sensor, Beacon), 64> {
    let mut pairings: ArrayVec<(Sensor, Beacon), 64> = ArrayVec::new();
    let mut input = input;
    while !input.is_empty() {
        let (rest, x) = parse::integer(input, true).unwrap();
        let (rest, y) = parse::integer(rest, true).unwrap();
        let sensor = Sensor(Point2D::new(x, y));

        let (rest, x) = parse::integer(rest, true).unwrap();
        let (rest, y) = parse::integer(rest, true).unwrap();
        let beacon = Beacon(Point2D::new(x, y));
        pairings.push((sensor, beacon));

        input = parse::seek_next_line(rest);
    }
    pairings
}

fn count_illegal_beacon_locs(
    pairings: &[(Sensor, Beacon)],
    y_dest: i64,
) -> (usize, Vec<ClosedInterval>) {
    let mut intervals: Vec<ClosedInterval> = Vec::with_capacity(256);
    let mut beacons_in_row: AHashSet<Beacon> = AHashSet::new();
    for (sensor, beacon) in pairings {
        if beacon.0.y == y_dest {
            beacons_in_row.insert(*beacon);
        }
        let dy = (sensor.0.y - y_dest).abs();
        let dist = sensor.0.manhattan(beacon.0);
        let dx = dist - dy;
        if dx < 0 {
            continue;
        }

        debug_assert!(dx >= 0);
        let interval = ClosedInterval::new(sensor.0.x - dx, sensor.0.x + dx);
        intervals.push(interval);
    }
    trace!(
        "beacons_in_row: {}, intervals: {:?}",
        beacons_in_row.len(),
        intervals
    );

    let merged = merge_intervals(&intervals);
    trace!("merged: {:?}", merged);

    let sum: usize = merged.iter().map(|x| x.len()).sum();
    (sum - beacons_in_row.len(), merged)
}

fn tuning_frequency(pairings: &[(Sensor, Beacon)], max: usize) -> i64 {
    let horizontal = ClosedInterval::new(0, max as i64);
    for y in 0..=max {
        let non_beacons = count_illegal_beacon_locs(pairings, y as i64).1;
        // find gap
        for (lhs, rhs) in non_beacons.iter().zip(non_beacons.iter().skip(1)) {
            if let (Some(lhs), Some(rhs)) = (lhs.intersect(&horizontal), rhs.intersect(&horizontal))
            {
                debug!("y={y}, lhs {:?}, rhs: {:?}", lhs, rhs);
                if lhs.disjoint(&rhs) {
                    let gap_width = rhs.a - lhs.b;
                    debug!("found gap of width {gap_width}");
                    if gap_width == 2 {
                        return (y as i64) + 4000000 * (lhs.b + 1);
                    }
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 15;

    const EXAMPLE: &[u8] = b"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn part1_example() {
        let pairings = parse_input(EXAMPLE);
        assert_eq!(26, count_illegal_beacon_locs(&pairings, 10).0);
    }

    #[test]
    fn part2_example() {
        let pairings = parse_input(EXAMPLE);
        assert_eq!(56000011, tuning_frequency(&pairings, 20));
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("5166077", answer.0);
        assert_eq!("13071206703981", answer.1);
    }
}
