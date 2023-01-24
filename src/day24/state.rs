use aoc_lib::point::Point2D;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node {
    pub position: Point2D<i32>,
    pub blizz_idx: u32,
}

impl Node {
    pub fn new(start: Point2D<i32>) -> Self {
        Self {
            position: start,
            blizz_idx: 0,
        }
    }

    pub fn neighbors(&self, end: Point2D<i32>, out: &mut Vec<Point2D<i32>>) {
        let deltas = [
            Point2D::new(0, -1),
            Point2D::new(0, 1),
            Point2D::new(1, 0),
            Point2D::new(-1, 0),
        ];
        for d in deltas {
            let candidate = self.position + d;
            if (candidate == Point2D::new(1, 0) || candidate == end)
                || (candidate.x > 0
                    && candidate.x <= end.x
                    && candidate.y > 0
                    && candidate.y < end.y)
            {
                out.push(candidate);
            }
        }
    }
}
