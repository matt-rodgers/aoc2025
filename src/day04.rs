use std::{convert::Infallible, str::FromStr};

use crate::Aoc;

const INPUT: &str = include_str!("../inputs/04.in");

pub struct Day04;

impl Aoc for Day04 {
    fn run(&self) -> (String, String) {
        let (pt1, pt2) = run_on_input(INPUT);
        (pt1.to_string(), pt2.to_string())
    }
}

fn run_on_input(input: &str) -> (usize, usize) {
    let mut grid: Grid = input.parse().unwrap();
    let pt1 = grid.removable_rolls().count();

    let mut pt2 = 0;

    loop {
        let removable: Vec<_> = grid.removable_rolls().collect();
        if removable.is_empty() {
            break;
        }

        for (x, y) in removable {
            grid.remove(x, y);
            pt2 += 1;
        }
    }

    (pt1, pt2)
}

#[derive(Debug)]
struct Grid {
    points: Vec<Vec<u8>>,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.points.get(y).and_then(|line| line.get(x)).copied()
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut u8> {
        self.points.get_mut(y).and_then(|line| line.get_mut(x))
    }

    fn remove(&mut self, x: usize, y: usize) {
        if let Some(point) = self.get_mut(x, y) {
            // Replace with 'x' as in example so that we can visualise which points are removed
            *point = b'x';
        }
    }

    fn is_removable(&self, x: usize, y: usize) -> bool {
        neighboring_points_iter(x, y)
            .filter(|(xn, yn)| self.get(*xn, *yn).map(|p| p == b'@').unwrap_or(false))
            .count()
            < 4
    }

    fn removable_rolls(&self) -> impl Iterator<Item = (usize, usize)> {
        self.points.iter().enumerate().flat_map(move |(y, line)| {
            line.iter().enumerate().filter_map(move |(x, point)| {
                // Only count points that are rolls of paper
                if *point != b'@' {
                    return None;
                }

                if self.is_removable(x, y) {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
    }
}

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<Vec<u8>> = s.trim().lines().map(|l| l.as_bytes().into()).collect();
        Ok(Self { points })
    }
}

fn neighboring_points_iter(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    const OFFSETS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    OFFSETS.iter().filter_map(move |(xo, yo)| {
        let xn = (x as isize + xo).try_into().ok()?;
        let yn = (y as isize + yo).try_into().ok()?;
        Some((xn, yn))
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../inputs/04.ex");

    #[test]
    fn test_example() {
        let (pt1, pt2) = run_on_input(EXAMPLE_INPUT);
        assert_eq!(13, pt1);
        assert_eq!(43, pt2);
    }

    #[test]
    fn test_neighboring_points_iter() {
        let neighbors: HashSet<(usize, usize)> = neighboring_points_iter(0, 0).collect();
        let expected: HashSet<(usize, usize)> = HashSet::from([(1, 0), (1, 1), (0, 1)]);
        assert_eq!(neighbors, expected);

        let neighbors: HashSet<(usize, usize)> = neighboring_points_iter(2, 2).collect();
        let expected: HashSet<(usize, usize)> = HashSet::from([
            (1, 1),
            (2, 1),
            (3, 1),
            (1, 2),
            (3, 2),
            (1, 3),
            (2, 3),
            (3, 3),
        ]);
        assert_eq!(neighbors, expected);
    }
}
