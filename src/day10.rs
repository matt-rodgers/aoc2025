use std::str::FromStr;

use itertools::Itertools;

use crate::Aoc;

const INPUT: &str = include_str!("../inputs/10.in");

pub struct Day10;

impl Aoc for Day10 {
    fn run(&self) -> (String, String) {
        let (pt1, pt2) = run_on_input(INPUT);
        (pt1.to_string(), pt2.to_string())
    }
}

fn run_on_input(input: &str) -> (usize, usize) {
    let machines: Vec<Machine> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let pt1 = machines
        .iter()
        .map(|machine| {
            for i in 1.. {
                // i is the number of button presses. Is it possible to make the light match with this
                // number of presses?
                if machine
                    .buttons
                    .0
                    .iter()
                    .combinations_with_replacement(i)
                    .any(|combinations| {
                        let mut state = machine.initial_state.clone();
                        for button in combinations {
                            state.apply_button_press(button);
                            if state == machine.desired_state {
                                return true;
                            }
                        }
                        false
                    })
                {
                    return i;
                }
            }

            panic!("we hit max usize and still didn't find a combination that works...");
        })
        .sum();

    (pt1, 0)
}

#[derive(Debug, Clone)]
struct Machine {
    initial_state: IndicatorLights,
    desired_state: IndicatorLights,
    buttons: Buttons,
    joltages: Joltages,
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let desired_state = IndicatorLights(
            parts
                .next()
                .unwrap()
                .strip_prefix('[')
                .unwrap()
                .strip_suffix(']')
                .unwrap()
                .as_bytes()
                .iter()
                .map(|c| if *c == b'#' { true } else { false })
                .collect(),
        );

        let initial_state = IndicatorLights(vec![false; desired_state.0.len()]);

        let mut buttons = Vec::new();
        let mut joltages = Vec::new();

        for part in parts {
            if part.starts_with('(') {
                let button = part
                    .strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect();
                buttons.push(Button(button));
            } else {
                for joltage in part
                    .strip_prefix('{')
                    .unwrap()
                    .strip_suffix('}')
                    .unwrap()
                    .split(',')
                    .map(|n| n.parse().unwrap())
                {
                    joltages.push(joltage);
                }
            }
        }

        Ok(Machine {
            initial_state,
            desired_state,
            buttons: Buttons(buttons),
            joltages: Joltages(joltages),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IndicatorLights(Vec<bool>);

impl IndicatorLights {
    fn apply_button_press(&mut self, button: &Button) {
        for index in button.0.iter() {
            self.0[*index] = !self.0[*index];
        }
    }
}

#[derive(Debug, Clone)]
struct Buttons(Vec<Button>);

#[derive(Debug, Clone)]
struct Button(Vec<usize>);

#[derive(Debug, Clone)]
struct Joltages(Vec<usize>);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../inputs/10.ex");

    #[test]
    fn test_example() {
        let (pt1, pt2) = run_on_input(EXAMPLE_INPUT);
        assert_eq!(7, pt1);
        assert_eq!(0, pt2);
    }
}
