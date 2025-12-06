use std::str::FromStr;

use crate::Aoc;

const INPUT: &str = include_str!("../inputs/06.in");

pub struct Day06;

impl Aoc for Day06 {
    fn run(&self) -> (String, String) {
        let (pt1, pt2) = run_on_input(INPUT);
        (pt1.to_string(), pt2.to_string())
    }
}

fn run_on_input(input: &str) -> (u64, u64) {
    let columns: Columns = input.parse().unwrap();

    let pt1 = columns
        .0
        .iter()
        .map(|column| {
            let mut column_iter = column.iter();
            let operation: Operation = column_iter.next().unwrap().parse().unwrap();
            column_iter.fold(0, |accum, item| {
                let n: u64 = item.trim().parse().unwrap();
                match operation {
                    Operation::Add => accum + n,
                    Operation::Multiply => match accum {
                        0 => n,
                        _ => accum * n,
                    },
                }
            })
        })
        .sum();

    let pt2 = columns
        .0
        .iter()
        .map(|column| {
            let mut column_iter = column.iter();
            let ops_str = column_iter.next().unwrap();
            let column_iter = column_iter; // immutable
            let l = ops_str.len(); // all strings are the same length
            let operation: Operation = ops_str.parse().unwrap();
            (0..l)
                .map(|i| {
                    column_iter.clone().rev().fold(None, |accum, item| {
                        let ch = item.as_bytes()[i];
                        if ch.is_ascii_digit() {
                            let digit = (ch - b'0') as u64;
                            match accum {
                                None => Some(digit),
                                Some(existing) => Some(existing * 10 + digit),
                            }
                        } else {
                            accum
                        }
                    })
                })
                .fold(0, |accum, n| {
                    let Some(n) = n else {
                        return 0;
                    };

                    match operation {
                        Operation::Add => accum + n,
                        Operation::Multiply => match accum {
                            0 => n,
                            _ => accum * n,
                        },
                    }
                })
        })
        .sum();

    (pt1, pt2)
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Columns(Vec<Vec<String>>);

impl FromStr for Columns {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().rev();

        let mut start = 0;
        let mut result = Vec::new();
        let mut found_end = false;

        for i in 0.. {
            if lines.clone().all(|line| match line.chars().nth(i) {
                Some(' ') => true,
                Some(_) => false,
                None => {
                    found_end = true;
                    true
                }
            }) {
                result.push(
                    lines
                        .clone()
                        .map(|line| line[start..i].to_string())
                        .collect(),
                );
                start = i;
            }

            if found_end {
                break;
            }
        }

        Ok(Columns(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../inputs/06.ex");

    #[test]
    fn test_example() {
        let (pt1, pt2) = run_on_input(EXAMPLE_INPUT);
        assert_eq!(4277556, pt1);
        assert_eq!(3263827, pt2);
    }
}
