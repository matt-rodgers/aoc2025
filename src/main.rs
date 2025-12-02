use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use clap::Parser;

mod day01;

#[derive(Parser)]
struct Args {
    /// The day to run, will run all if omitted
    day: Option<usize>,
}

static DAYS: [&(dyn Aoc + Send + Sync); 1] = [&day01::Day01];

fn main() {
    let args = Args::parse();

    match args.day {
        Some(day) => {
            let aoc = DAYS.get(day - 1).expect("invalid day index");
            let timed_solution = run_with_timing(*aoc);
            println!("{timed_solution}");
        }
        None => {
            for (i, aoc) in DAYS.iter().enumerate() {
                let timed_solution = run_with_timing(*aoc);
                println!("day {:2}: {timed_solution}", i + 1);
            }
        }
    }
}

pub trait Aoc {
    /// Run the problem, returning the part1 and part2 answers as Strings
    fn run(&self) -> (String, String);
}

struct TimedSolution {
    elapsed: Duration,
    pt1: String,
    pt2: String,
}

impl Display for TimedSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pt1={:20} pt2={:20} elapsed={} us",
            self.pt1,
            self.pt2,
            self.elapsed.as_micros()
        )
    }
}

fn run_with_timing(aoc: &dyn Aoc) -> TimedSolution {
    let start = Instant::now();
    let (pt1, pt2) = aoc.run();
    let elapsed = Instant::now() - start;

    TimedSolution { elapsed, pt1, pt2 }
}
