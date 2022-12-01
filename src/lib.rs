// marker1
pub mod day01;

use std::io::Result;

pub fn solve(day: i32) -> Result<(String, String)> {
    match day {
        // marker2
        1 => Ok(day01::solve(&aoc_lib::io::read_input(day)?)),
        _ => panic!("invalid day: {}", day),
    }
}
