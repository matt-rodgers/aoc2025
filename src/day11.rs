use std::collections::HashMap;

use crate::Aoc;

const INPUT: &str = include_str!("../inputs/11.in");

pub struct Day11;

impl Aoc for Day11 {
    fn run(&self) -> (String, String) {
        let (pt1, pt2) = run_on_input(INPUT);
        (pt1.to_string(), pt2.to_string())
    }
}

fn run_on_input(input: &str) -> (u64, usize) {
    let devices: HashMap<&str, Vec<&str>> = input
        .trim()
        .lines()
        .map(|line| {
            let (device, rest) = line.split_once(": ").unwrap();
            let outputs = rest.split_whitespace().collect();
            (device, outputs)
        })
        .collect();

    let pt1 = count_paths("you", "out", &devices);

    (pt1, 0)
}

fn count_paths(from: &str, to: &str, edges: &HashMap<&str, Vec<&str>>) -> u64 {
    match edges.get(from) {
        None => 0,
        Some(outputs) => outputs
            .iter()
            .map(|output| {
                if output == &to {
                    1
                } else {
                    count_paths(output, to, edges)
                }
            })
            .sum(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../inputs/11.ex");

    #[test]
    fn test_example() {
        let (pt1, pt2) = run_on_input(EXAMPLE_INPUT);
        assert_eq!(5, pt1);
        assert_eq!(0, pt2);
    }
}
