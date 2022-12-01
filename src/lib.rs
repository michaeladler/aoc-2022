// marker1
pub mod day01;
pub mod day02;

use std::io::Result;

pub fn solve(day: i32) -> Result<(String, String)> {
    match day {
        // marker2
        2 => Ok(day02::solve(&aoc_lib::io::read_input(day)?)),
        1 => Ok(day01::solve(&aoc_lib::io::read_input(day)?)),
        _ => panic!("invalid day: {}", day),
    }
}
