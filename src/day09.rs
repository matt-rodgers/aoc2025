use crate::Aoc;
use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/09.in");

pub struct Day09;

impl Aoc for Day09 {
    fn run(&self) -> (String, String) {
        let (pt1, pt2) = run_on_input(INPUT);
        (pt1.to_string(), pt2.to_string())
    }
}

fn run_on_input(input: &str) -> (i64, usize) {
    let coordinates: Vec<(i64, i64)> = input
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let largest_area = coordinates
        .iter()
        .tuple_combinations()
        .map(|(a, b)| area_between_corners(*a, *b))
        .max()
        .unwrap();

    (largest_area, 0)
}

fn area_between_corners(a: (i64, i64), b: (i64, i64)) -> i64 {
    let xdiff = a.0 - b.0;
    let ydiff = a.1 - b.1;
    (xdiff.abs() + 1) * (ydiff.abs() + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../inputs/09.ex");

    #[test]
    fn test_example() {
        let (pt1, pt2) = run_on_input(EXAMPLE_INPUT);
        assert_eq!(50, pt1);
        assert_eq!(0, pt2);
    }

    #[test]
    fn test_area_between_corners() {
        assert_eq!(50, area_between_corners((2, 5), (11, 1)));
    }
}
