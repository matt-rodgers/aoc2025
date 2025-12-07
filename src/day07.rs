use std::collections::HashMap;

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

    // HashMap of (position, particle_count)
    let mut beam_positions = HashMap::new();
    beam_positions.insert(lines.next().unwrap().find('S').unwrap(), 1);

    let mut pt1 = 0;

    for line in lines {
        let mut new_beam_positions = HashMap::new();
        for (position, count) in beam_positions.iter() {
            match line.as_bytes().get(*position) {
                Some(b'^') => {
                    pt1 += 1;
                    *new_beam_positions.entry(*position + 1).or_insert(0) += count;
                    if *position > 0 {
                        *new_beam_positions.entry(*position - 1).or_insert(0) += count;
                    }
                }
                Some(b'.') => {
                    *new_beam_positions.entry(*position).or_insert(0) += count;
                }
                _ => {}
            }
        }

        beam_positions = new_beam_positions;
    }

    let pt2 = beam_positions.values().sum();

    (pt1, pt2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../inputs/07.ex");

    #[test]
    fn test_example() {
        let (pt1, pt2) = run_on_input(EXAMPLE_INPUT);
        assert_eq!(21, pt1);
        assert_eq!(40, pt2);
    }
}
