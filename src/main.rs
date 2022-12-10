use arrayvec::ArrayVec;
use log::debug;
use std::env;
use std::io::{self, Write};
use std::time::Instant;

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let mut days_to_solve: ArrayVec<i32, 25> = ArrayVec::new();

    for arg in env::args().skip(1) {
        let day = arg.trim_start_matches('0').parse::<i32>().unwrap();
        days_to_solve.push(day);
    }
    if days_to_solve.is_empty() {
        days_to_solve = (1..=25).collect();
    }

    let out = io::stdout();
    let mut handle = out.lock();

    let mut total_ms: f64 = 0.0;

    debug!("solving days: {:?}", days_to_solve);
    for day in days_to_solve {
        let now = Instant::now();
        if let Some(solution) = aoc::solve(day) {
            let duration = now.elapsed().as_nanos() as u64;
            let duration_ms: f64 = duration as f64 / 1_000_000.;
            total_ms += duration_ms;

            writeln!(
                handle,
                "[Day {:02}]\tpart1: {:<16}\tpart2: {:<16}\tduration: {:>10.3} ms",
                day, solution.0, solution.1, duration_ms
            )
            .unwrap();
        } else {
            debug!("skipping day {day} because no input was found");
        }
    }

    writeln!(handle, "\nTotal: {total_ms:.3} ms").unwrap();
}
