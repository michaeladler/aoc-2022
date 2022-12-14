use std::cmp::{max, min};

use log::trace;

#[derive(Debug)]
pub struct Rock {
    points: Vec<Point>,
    pub y_min: i32,
    pub y_max: i32,
    pub x_min: i32,
    pub x_max: i32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Rock {
    pub fn new(points: Vec<Point>) -> Self {
        let mut x_min = i32::MAX;
        let mut x_max = i32::MIN;
        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;
        for p in &points {
            if p.x < x_min {
                x_min = p.x;
            }
            if p.x > x_max {
                x_max = p.x;
            }
            if p.y < y_min {
                y_min = p.y;
            }
            if p.y > y_max {
                y_max = p.y;
            }
        }
        Self {
            points,
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    /// Test whether `candidate` is part of the rock structure.
    pub fn contains(&self, candidate: &Point) -> bool {
        trace!("checking points: {:?}", self.points);
        trace!(
            "top_left=({}, {}), bottom_right({}, {})",
            self.x_min,
            self.y_min,
            self.x_max,
            self.y_max
        );
        if candidate.y < self.y_min
            || candidate.y > self.y_max
            || candidate.x < self.x_min
            || candidate.x > self.x_max
        {
            trace!(
                "rejecting point {:?} because not inside rectangle",
                candidate
            );
            return false;
        }
        trace!("point {:?} is inside rectangle", candidate);
        for (p, q) in self.points.iter().zip(&self.points[1..]) {
            trace!("checking line {:?} -> {:?}", p, q);
            if p.x == q.x {
                let result = candidate.x == p.x
                    && min(p.y, q.y) <= candidate.y
                    && candidate.y <= max(p.y, q.y);
                if result {
                    trace!("point is on horizontal line: {result}");
                    return result;
                }
            } else if p.y == q.y {
                let result = candidate.y == p.y
                    && min(p.x, q.x) <= candidate.x
                    && candidate.x <= max(p.x, q.x);
                if result {
                    trace!("point is on vertical line: {result}");
                    return result;
                }
            } else {
                panic!("only horizontal and vertical lines are allowed");
            }
        }
        trace!("point {:?} does not hit this rock", candidate);
        false
    }
}
