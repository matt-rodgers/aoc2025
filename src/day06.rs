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
    let mut lines_iter = input
        .trim()
        .lines()
        .map(|line| line.split_whitespace())
        .rev();

    let mut context: Vec<MathContext> = lines_iter
        .next()
        .unwrap()
        .map(|op| MathContext {
            value: None,
            op: op.parse::<Operation>().unwrap(),
        })
        .collect();

    for line in lines_iter {
        for (context, val) in context.iter_mut().zip(line) {
            let n: u64 = val.parse().unwrap();
            context.value = match context.value {
                None => Some(n),
                Some(current) => Some(context.op.apply(current, n)),
            };
        }
    }

    let pt1 = context
        .iter()
        .fold(0, |accum, ctx| accum + ctx.value.unwrap());

    (pt1, 0)
}

#[derive(Debug, Clone, Copy)]
struct MathContext {
    value: Option<u64>,
    op: Operation,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => Err(()),
        }
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
        assert_eq!(0, pt2);
    }
}
