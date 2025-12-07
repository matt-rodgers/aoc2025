use std::collections::HashSet;

use crate::Aoc;

const INPUT: &str = include_str!("../inputs/07.in");

pub struct Day07;

impl Aoc for Day07 {
    fn run(&self) -> (String, String) {
        let (pt1, pt2) = run_on_input(INPUT);
        (pt1.to_string(), pt2.to_string())
    }
}

fn run_on_input(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    let mut beam_positions = HashSet::new();
    beam_positions.insert(lines.next().unwrap().find('S').unwrap());
    let mut pt1 = 0;

    for line in lines {
        let mut new_beam_positions = HashSet::new();
        for position in beam_positions.iter() {
            match line.as_bytes().get(*position) {
                Some(b'^') => {
                    pt1 += 1;
                    new_beam_positions.insert(*position + 1);
                    if *position > 0 {
                        new_beam_positions.insert(position - 1);
                    }
                }
                Some(b'.') => {
                    new_beam_positions.insert(*position);
                }
                _ => {}
            }
        }

        beam_positions = new_beam_positions;
    }

    (pt1, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../inputs/07.ex");

    #[test]
    fn test_example() {
        let (pt1, pt2) = run_on_input(EXAMPLE_INPUT);
        assert_eq!(21, pt1);
        assert_eq!(0, pt2);
    }
}
