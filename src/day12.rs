use std::{collections::HashMap, str::FromStr};

use crate::Aoc;

const INPUT: &str = include_str!("../inputs/12.in");

pub struct Day12;

impl Aoc for Day12 {
    fn run(&self) -> (String, String) {
        let (pt1, pt2) = run_on_input(INPUT);
        (pt1.to_string(), pt2.to_string())
    }
}

fn run_on_input(input: &str) -> (usize, usize) {
    let (presents, regions) = input.trim().rsplit_once("\n\n").unwrap();

    let regions: Vec<Region> = regions.lines().map(|line| line.parse().unwrap()).collect();

    // We don't actually need the presents for anything, all we're really doing here is asserting
    // some assumptions about the input.
    let _presents: HashMap<usize, Present> = presents
        .split("\n\n")
        .map(|chunk| {
            let (index, rest) = chunk.split_once(':').unwrap();
            (index.parse().unwrap(), rest.trim().parse().unwrap())
        })
        .collect();

    let pt1 = regions
        .iter()
        .filter(|region| {
            // If each present is in its own 3x3 square, and there are less presents than 3x3
            // squares in the region, then they definitely all fit. Note that this is a lower bound,
            // it's possible to have regions that fit all the presents but do not have enough 3x3
            // squares, but in the case of the actual input this case does not seem to happen.
            let num_squares_in_region = (region.width / 3) * (region.height / 3);
            let total_count: usize = region.counts.iter().copied().sum();
            total_count <= num_squares_in_region
        })
        .count();

    (pt1, 0)
}

#[derive(Debug, Clone)]
struct Present;

impl FromStr for Present {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (width, height) = s.trim().lines().fold((0, 0), |(w, h), line| {
            let width = line.len();
            (w.max(width), h + 1)
        });

        // Based on inspection of the input, everything is 3x3.
        assert_eq!(3, width);
        assert_eq!(3, height);

        Ok(Present)
    }
}

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

impl FromStr for Region {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (width, rest) = s.split_once('x').unwrap();
        let (height, rest) = rest.split_once(": ").unwrap();
        let counts = rest
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Ok(Region {
            width: width.parse().unwrap(),
            height: height.parse().unwrap(),
            counts,
        })
    }
}
