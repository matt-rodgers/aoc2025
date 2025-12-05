use std::str::FromStr;

use crate::Aoc;

const INPUT: &str = include_str!("../inputs/05.in");

pub struct Day05;

impl Aoc for Day05 {
    fn run(&self) -> (String, String) {
        let (pt1, pt2) = run_on_input(INPUT);
        (pt1.to_string(), pt2.to_string())
    }
}

fn run_on_input(input: &str) -> (usize, usize) {
    let Inventory {
        fresh_ranges,
        available_ingredients,
    } = input.parse().unwrap();

    let pt1 = available_ingredients
        .iter()
        .filter(|id| fresh_ranges.iter().any(|range| range.contains(id)))
        .count();

    (pt1, 0)
}

#[derive(Debug, Clone)]
struct Inventory {
    fresh_ranges: Vec<std::ops::RangeInclusive<u64>>,
    available_ingredients: Vec<u64>,
}

impl FromStr for Inventory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ranges, ids) = s.trim().split_once("\n\n").unwrap();

        let fresh_ranges = ranges
            .trim()
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('-').unwrap();
                let start = start.parse().unwrap();
                let end = end.parse().unwrap();
                start..=end
            })
            .collect();

        let available_ingredients = ids.trim().lines().map(|n| n.parse().unwrap()).collect();

        Ok(Self {
            fresh_ranges,
            available_ingredients,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../inputs/05.ex");

    #[test]
    fn test_example() {
        let (pt1, pt2) = run_on_input(EXAMPLE_INPUT);
        assert_eq!(3, pt1);
        assert_eq!(0, pt2);
    }
}
