use aoc_lib::point::Point2D;

const MAX_WIDTH: usize = 122;
const MAX_HEIGHT: usize = 27;

#[derive(Clone)]
pub struct Blizzard {
    // indexed by row
    right: [u128; MAX_HEIGHT],
    // indexed by row
    left: [u128; MAX_HEIGHT],
    // indexed by column
    up: [u32; MAX_WIDTH],
    // indexed by column
    down: [u32; MAX_WIDTH],
}

impl Blizzard {
    pub fn new() -> Self {
        Self {
            right: [0; MAX_HEIGHT],
            left: [0; MAX_HEIGHT],
            up: [0; MAX_WIDTH],
            down: [0; MAX_WIDTH],
        }
    }

    pub fn add_blizz(&mut self, x: usize, y: usize, b: u8) {
        match b {
            b'>' => {
                self.right[y] |= 1 << x;
            }
            b'<' => {
                self.left[y] |= 1 << x;
            }
            b'v' => {
                self.down[x] |= 1 << y;
            }
            b'^' => {
                self.up[x] |= 1 << y;
            }
            _ => {}
        }
    }

    pub fn move_blizzards(&self, end: Point2D<i32>) -> Self {
        let mut result = self.clone();
        let width = end.x;
        for row in result.left.iter_mut() {
            let x = *row;
            *row = x >> 1 | x << (width - 1);
        }
        for row in result.right.iter_mut() {
            let x = *row;
            *row = x << 1 | x >> (width - 1);
        }
        let height = end.y - 1;
        for col in result.up.iter_mut() {
            let x = *col;
            *col = x >> 1 | x << (height - 1);
        }
        for col in result.down.iter_mut() {
            let x = *col;
            *col = x << 1 | x >> (height - 1);
        }
        result
    }

    pub fn is_blizzard(&self, p: Point2D<i32>) -> bool {
        self.left_blizz(p) || self.right_blizz(p) || self.up_blizz(p) || self.down_blizz(p)
    }

    pub fn left_blizz(&self, p: Point2D<i32>) -> bool {
        (self.left[p.y as usize] & (1 << p.x as usize)) != 0
    }

    pub fn right_blizz(&self, p: Point2D<i32>) -> bool {
        (self.right[p.y as usize] & (1 << p.x as usize)) != 0
    }

    pub fn up_blizz(&self, p: Point2D<i32>) -> bool {
        (self.up[p.x as usize] & (1 << p.y as usize)) != 0
    }

    pub fn down_blizz(&self, p: Point2D<i32>) -> bool {
        (self.down[p.x as usize] & (1 << p.y as usize)) != 0
    }

    /*
    pub fn print(&self, position: Point2D<i32>, end: Point2D<i32>) {
        println!("State:");
        let width = end.x as usize + 1;
        for i in 0..=width {
            if i as i32 == 1 {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();

        for y in 1..end.y as usize {
            print!("#");
            for x in 1..width {
                let p = Point2D::new(x as i32, y as i32);
                let count = (self.left_blizz(p) as u32)
                    + (self.right_blizz(p) as u32)
                    + (self.up_blizz(p) as u32)
                    + (self.down_blizz(p) as u32);
                if count > 1 {
                    print!("{count}");
                } else if self.left_blizz(p) {
                    print!("<");
                } else if self.right_blizz(p) {
                    print!(">");
                } else if self.up_blizz(p) {
                    print!("^");
                } else if self.down_blizz(p) {
                    print!("v");
                } else if position == Point2D::new(x as i32, y as i32) {
                    print!("E");
                } else {
                    print!(".");
                }
            }
            println!("#");
        }

        for i in 0..=width {
            if i as i32 == end.x {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
    */
}
