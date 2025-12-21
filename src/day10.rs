use std::{collections::BTreeMap, str::FromStr};

use arrayvec::ArrayVec;
use itertools::Itertools;

use crate::Aoc;

const INPUT: &str = include_str!("../inputs/10.in");

const MAX_ARRAY_LEN: usize = 10;

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
                    // We can just use `combinations` rather than `combinations_with_replacement`
                    // here, since the buttons toggle the state and therefore pressing a button
                    // twice has no effect, pressing it three times has the same effect as pressing
                    // it once, and so on.
                    .combinations(i)
                    .any(|combinations| {
                        // Starting from all false and pressing buttons until the lights match the
                        // desired state is equivalent to starting at the desired state and pressing
                        // buttons until all lights are false (but the latter is simpler)
                        let mut state = machine.indicators.clone();
                        for button in combinations {
                            state.apply_button_press(button);
                            if state.0.iter().all(|light| !light) {
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

    let pt2 = machines.iter().map(|m| m.solve_pt2()).sum();

    (pt1, pt2)
}

#[derive(Debug, Clone)]
struct Machine {
    indicators: IndicatorLights,
    joltages: Joltages,
    buttons: Buttons,
}

impl Machine {
    fn solve_pt2(&self) -> usize {
        // This is all of the possible joltages that cna be produced by pressing each button at most
        // once, along with the number of button presses required to make that value.
        let pattern_costs = self
            .buttons
            .possible_single_press_joltages(self.joltages.0.len());

        let mut cache = BTreeMap::new();
        solve_single_recurse(self.joltages.clone(), &pattern_costs, &mut cache).unwrap()
    }
}

/// Starting from the target joltages, recursively subtract the possible button presses until we
/// find the mimimum possible number of presses to reach zero.
fn solve_single_recurse(
    joltages: Joltages,
    pattern_costs: &BTreeMap<Joltages, usize>,
    cache: &mut BTreeMap<Joltages, Option<usize>>,
) -> Option<usize> {
    // Early return if we already solved it
    if joltages.is_zero() {
        return Some(0);
    }

    // Early return if answer already cached
    if let Some(answer) = cache.get(&joltages) {
        return *answer;
    }

    let mut answer = None;
    for (pattern, cost) in pattern_costs.iter() {
        if let Some(mut new_joltages) = joltages.clone() - pattern.clone() {
            // At this stage, we only need to continue if everything is divisible by two. The reason
            // this works is that *any* sequence of button presses can be expressed as a set of
            // buttons pressed once, and a set of buttons pressed an even number of times.
            //
            // It is also guaranteed that any sequence of button presses where all buttons are
            // pressed an even number of times results in all joltages being even.
            //
            // Finally, if we have even joltages we know for sure that halving the joltages, finding
            // the number of button presses to reach the halved value and then doubling it will give
            // the optimum number of presses (ie. there cannot be a way involving an odd number of
            // button presses that 'beats' this route). This is only the case because each button
            // can only increment each joltage by 1, if we had increments greater than 1 this would
            // no longer be the case.
            //
            // Dividing it up this way greatly reduces the state-space we need to visit to find the
            // minumum number of button presses, since by pressing each button once and then only
            // considering possibilities where the remaining joltages are even we guarantee covering
            // every possible sequence of button presses that arrives at the answer without actually
            // having to fully check them all one button press at a time.
            //
            // The idea for this approach came from:
            // <https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/>
            if new_joltages.is_even() {
                new_joltages.halve();
                if let Some(sub_cost) = solve_single_recurse(new_joltages, pattern_costs, cache) {
                    let new_cost = cost + 2 * sub_cost;
                    match answer {
                        Some(ans) if ans > new_cost => answer = Some(new_cost),
                        None => answer = Some(new_cost),
                        _ => {}
                    }
                }
            }
        }
    }

    cache.insert(joltages, answer);
    answer
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let indicators = IndicatorLights(
            parts
                .next()
                .unwrap()
                .strip_prefix('[')
                .unwrap()
                .strip_suffix(']')
                .unwrap()
                .as_bytes()
                .iter()
                .map(|c| *c == b'#')
                .collect(),
        );

        let mut buttons = Vec::new();
        let mut joltages = ArrayVec::new();

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
            indicators,
            joltages: Joltages(joltages),
            buttons: Buttons(buttons),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IndicatorLights(ArrayVec<bool, MAX_ARRAY_LEN>);

impl IndicatorLights {
    fn apply_button_press(&mut self, button: &Button) {
        for index in button.0.iter() {
            self.0[*index] = !self.0[*index];
        }
    }
}

#[derive(Debug, Clone)]
struct Buttons(Vec<Button>);

impl Buttons {
    /// Return all possible joltages that can be produced by pressing each button a maximum of once,
    /// along with the number of presses needed to achieve that joltage
    fn possible_single_press_joltages(&self, joltage_len: usize) -> BTreeMap<Joltages, usize> {
        let mut out = BTreeMap::new();

        for n in 0..self.0.len() + 1 {
            for combo in self.0.iter().combinations(n) {
                let mut j = Joltages::new(joltage_len);
                for button in combo {
                    j.apply_button_press(button);
                }

                let entry = out.entry(j).or_insert(n);
                if n < *entry {
                    *entry = n;
                }
            }
        }

        out
    }
}

#[derive(Debug, Clone)]
struct Button(ArrayVec<usize, MAX_ARRAY_LEN>);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Joltages(ArrayVec<i64, MAX_ARRAY_LEN>);

impl Joltages {
    fn apply_button_press(&mut self, button: &Button) {
        for index in button.0.iter() {
            self.0[*index] += 1;
        }
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(|n| *n == 0)
    }

    fn is_even(&self) -> bool {
        self.0.iter().all(|n| *n % 2 == 0)
    }

    fn halve(&mut self) {
        for n in self.0.iter_mut() {
            *n /= 2;
        }
    }

    fn new(len: usize) -> Self {
        let mut av = ArrayVec::new();
        for _ in 0..len {
            av.push(0);
        }
        Self(av)
    }
}

impl std::ops::Sub<Joltages> for Joltages {
    type Output = Option<Joltages>;

    fn sub(mut self, rhs: Joltages) -> Self::Output {
        assert!(self.0.len() >= rhs.0.len());

        for (n, other) in self.0.iter_mut().zip(rhs.0.into_iter()) {
            if other > *n {
                return None;
            } else {
                *n -= other;
            }
        }

        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../inputs/10.ex");

    #[test]
    fn test_example() {
        let (pt1, pt2) = run_on_input(EXAMPLE_INPUT);
        assert_eq!(7, pt1);
        assert_eq!(33, pt2);
    }

    #[test]
    fn test_single_problematic() {
        let machine: Machine = "[#.#.#] (0,1,2,3,4) (0,2,4) (0,2,3) {29,17,29,20,26}"
            .parse()
            .unwrap();

        let pattern_costs = machine
            .buttons
            .possible_single_press_joltages(machine.joltages.0.len());

        // I don't actually know what the answer should be, we are just testing that it doesn't panic
        let _pt2 = machine.solve_pt2();
    }
}
