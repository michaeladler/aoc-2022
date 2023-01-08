use log::debug;

use aoc_lib::{parse, point::Point2D};

#[derive(Debug, Clone, Copy)]
enum Orientation {
    East,
    South,
    West,
    North,
}

impl Orientation {
    pub fn score(&self) -> i32 {
        match self {
            Orientation::East => 0,
            Orientation::South => 1,
            Orientation::West => 2,
            Orientation::North => 3,
        }
    }

    pub fn turn(&self, turn: Turn) -> Self {
        let val = match turn {
            Turn::Right => self.score() + 1,
            Turn::Left => self.score() - 1,
        };
        let val = val.rem_euclid(4);
        match val {
            0 => Orientation::East,
            1 => Orientation::South,
            2 => Orientation::West,
            3 => Orientation::North,
            _ => panic!("impossible"),
        }
    }
}

const OPEN: u8 = b'.';
const EMPTY: u8 = b' ';
const WALL: u8 = b'#';

#[derive(Debug, Clone, Copy)]
struct Position {
    location: Point2D,
    orientation: Orientation,
}

impl Position {
    pub fn score(&self) -> i64 {
        let row = self.location.y + 1;
        let col = self.location.x + 1;
        1000 * row + 4 * col + (self.orientation.score() as i64)
    }
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    /// Clockwise
    Right,
    /// Counterclockwise
    Left,
}

#[derive(Debug)]
enum Instruction {
    Move(usize),
    Turn(Turn),
}

const COLS: usize = 256;
const ROWS: usize = 256;

pub fn solve(input: &[u8]) -> (String, String) {
    let mut grid: [[u8; COLS]; ROWS] = [[EMPTY; COLS]; ROWS];
    let mut instructions: Vec<Instruction> = Vec::with_capacity(512);
    {
        let mut y: usize = 0;
        let mut input = input;
        while !input.is_empty() {
            if input[0].is_ascii_digit() {
                loop {
                    match input[0] {
                        b'0'..=b'9' => {
                            let (rest, x) = parse::positive(input, false).unwrap();
                            instructions.push(Instruction::Move(x as usize));
                            input = rest;
                        }
                        b'R' => {
                            instructions.push(Instruction::Turn(Turn::Right));
                            input = &input[1..];
                        }
                        b'L' => {
                            instructions.push(Instruction::Turn(Turn::Left));
                            input = &input[1..];
                        }
                        b'\n' => {
                            break;
                        }
                        _ => panic!("unexpected char"),
                    }
                }
                break;
            }

            for (x, &b) in input.iter().enumerate() {
                if b == b'\n' {
                    input = &input[x..];
                    break;
                }
                grid[y][x] = b;
            }

            y += 1;
            input = parse::seek_next_line(input);
        }
    }
    debug!("instructions: {:?}", instructions);

    let mut pos = Position {
        location: Point2D {
            y: 0,
            x: grid[0]
                .iter()
                .enumerate()
                .find(|(_, &b)| b == OPEN)
                .unwrap()
                .0 as i64,
        },
        orientation: Orientation::East,
    };
    for inst in &instructions {
        debug!("pos: {:?}", pos);
        debug!("applying: {:?}", inst);
        match inst {
            Instruction::Move(steps) => {
                let delta = match pos.orientation {
                    Orientation::East => Point2D::new(1, 0),
                    Orientation::South => Point2D::new(0, 1),
                    Orientation::West => Point2D::new(-1, 0),
                    Orientation::North => Point2D::new(0, -1),
                };
                debug!("delta: {:?}", delta);
                let mut i: usize = 0;
                while i < *steps {
                    let mut new_loc = pos.location + delta;
                    new_loc.y = new_loc.y.rem_euclid(ROWS as i64);
                    new_loc.x = new_loc.x.rem_euclid(COLS as i64);
                    let val = grid[new_loc.y as usize][new_loc.x as usize];
                    match val {
                        OPEN => {
                            debug!("move ok");
                            pos.location = new_loc;
                        }
                        WALL => {
                            debug!("hit wall");
                            break;
                        }
                        EMPTY => {
                            let new_loc: Point2D = match pos.orientation {
                                Orientation::East => {
                                    let x = grid[pos.location.y as usize]
                                        .iter()
                                        .enumerate()
                                        .find(|(_i, &b)| b != EMPTY)
                                        .unwrap()
                                        .0;
                                    Point2D::new(x as i64, pos.location.y)
                                }
                                Orientation::West => {
                                    let x = grid[pos.location.y as usize]
                                        .iter()
                                        .enumerate()
                                        .rev()
                                        .find(|(_i, &b)| b != EMPTY)
                                        .unwrap()
                                        .0;
                                    Point2D::new(x as i64, pos.location.y)
                                }
                                Orientation::South => {
                                    let y = grid
                                        .iter()
                                        .enumerate()
                                        .find(|(_i, row)| row[pos.location.x as usize] != EMPTY)
                                        .unwrap()
                                        .0;
                                    Point2D::new(pos.location.x, y as i64)
                                }
                                Orientation::North => {
                                    let y = grid
                                        .iter()
                                        .enumerate()
                                        .rev()
                                        .find(|(_i, row)| row[pos.location.x as usize] != EMPTY)
                                        .unwrap()
                                        .0;
                                    Point2D::new(pos.location.x, y as i64)
                                }
                            };
                            let val = grid[new_loc.y as usize][new_loc.x as usize];
                            if val != WALL {
                                debug!("wrap around");
                                pos.location = new_loc;
                            } else {
                                debug!("cannot wrap around due to WALL on the other side");
                                break;
                            }
                        }
                        _ => {
                            panic!("impossible");
                        }
                    }
                    i += 1;
                }
                debug!("moved {i} steps");
            }
            Instruction::Turn(t) => {
                pos.orientation = pos.orientation.turn(*t);
            }
        }
    }

    debug!("final: {:?}", pos);
    let part1 = pos.score();
    let part2: i64 = 42;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i32 = 22;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn example() {
        init();

        let input = b"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
 
";

        let answer = solve(input);
        assert_eq!("6032", answer.0);
        // assert_eq!("42", answer.1);
    }

    #[test]
    fn part1_and_part2() {
        let answer = solve(&aoc_lib::io::read_input(DAY).unwrap());
        assert_eq!("186128", answer.0);
        //assert_eq!("42", answer.1);
    }
}
