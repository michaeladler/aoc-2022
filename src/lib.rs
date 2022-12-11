// marker1
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;

pub fn solve(day: i32) -> Option<(String, String)> {
    match (day, aoc_lib::io::read_input(day)) {
        // marker2
        (11, Ok(input)) => Some(day11::solve(&input)),
        (10, Ok(input)) => Some(day10::solve(&input)),
        (9, Ok(input)) => Some(day09::solve(&input)),
        (8, Ok(input)) => Some(day08::solve(&input)),
        (7, Ok(input)) => Some(day07::solve(&input)),
        (6, Ok(input)) => Some(day06::solve(&input)),
        (5, Ok(input)) => Some(day05::solve(&input)),
        (4, Ok(input)) => Some(day04::solve(&input)),
        (3, Ok(input)) => Some(day03::solve(&input)),
        (2, Ok(input)) => Some(day02::solve(&input)),
        (1, Ok(input)) => Some(day01::solve(&input)),
        _ => None,
    }
}
