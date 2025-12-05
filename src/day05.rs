use std::{ops::RangeInclusive, str::FromStr};

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
        mut fresh_ranges,
        available_ingredients,
    } = input.parse().unwrap();

    let pt1 = available_ingredients
        .iter()
        .filter(|id| fresh_ranges.iter().any(|range| range.contains(id)))
        .count();

    fresh_ranges.sort_unstable_by_key(|range| *range.start());
    let mut fresh_ranges = fresh_ranges.into_iter();

    let mut merged_ranges = Vec::new();
    let mut current_merged = fresh_ranges.next().unwrap();

    for next_range in fresh_ranges {
        match merge_ranges(current_merged, next_range) {
            Ok(merged) => {
                // If the ranges can be merged, update to include th current range
                current_merged = merged;
            }
            Err((a, b)) => {
                // If the ranges cannot be merged, push the existing merged range into the list and
                // start a new merged range
                merged_ranges.push(a);
                current_merged = b;
            }
        }
    }

    merged_ranges.push(current_merged);

    let pt2 = merged_ranges.into_iter().flatten().count();

    (pt1, pt2)
}

#[derive(Debug, Clone)]
struct Inventory {
    fresh_ranges: Vec<RangeInclusive<u64>>,
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

// Try to merge two ranges, returning either the merged range or the two original ranges back again
// if they could not be merged.
fn merge_ranges(
    a: RangeInclusive<u64>,
    b: RangeInclusive<u64>,
) -> Result<RangeInclusive<u64>, (RangeInclusive<u64>, RangeInclusive<u64>)> {
    // Must be sorted first
    assert!(a.start() <= b.start());

    if a.end() >= b.start() {
        let end = a.end().max(b.end());
        Ok(*a.start()..=*end)
    } else {
        Err((a, b))
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
        assert_eq!(14, pt2);
    }

    #[test]
    fn test_merge_ranges() {
        assert_eq!(Ok(1..=10), merge_ranges(1..=5, 3..=10));
        assert_eq!(Ok(3..=11), merge_ranges(3..=6, 6..=11));
        assert_eq!(Ok(5..=20), merge_ranges(5..=20, 8..=10));
    }
}
