// marker1
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

pub fn solve(day: i32) -> Option<(String, String)> {
    match (day, aoc_lib::io::read_input(day)) {
        // marker2
        (5, Ok(input)) => Some(day05::solve(&input)),
        (4, Ok(input)) => Some(day04::solve(&input)),
        (3, Ok(input)) => Some(day03::solve(&input)),
        (2, Ok(input)) => Some(day02::solve(&input)),
        (1, Ok(input)) => Some(day01::solve(&input)),
        _ => None,
    }
}
