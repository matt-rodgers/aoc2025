use crate::Aoc;

const INPUT: &str = include_str!("../inputs/03.in");

pub struct Day03;

impl Aoc for Day03 {
    fn run(&self) -> (String, String) {
        let (pt1, pt2) = run_on_input(INPUT);
        (pt1.to_string(), pt2.to_string())
    }
}

fn run_on_input(input: &str) -> (usize, usize) {
    let pt1 = input
        .trim()
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let (highest, position) = first_maximum(line.iter().copied());

            // If the highest number is in the last position, it can't be the first number of the two
            // digit result. So find the next highest, skipping the last position.
            let (highest, position) = if position == line.len() - 1 {
                first_maximum(line.iter().copied().take(line.len() - 1))
            } else {
                (highest, position)
            };

            let (highest_of_remaining, _) = first_maximum(line.iter().copied().skip(position + 1));

            let n = highest * 10 + highest_of_remaining;

            n as usize
        })
        .sum();

    (pt1, 0)
}

fn first_maximum<T: Iterator<Item = u8>>(iter: T) -> (u8, usize) {
    let mut highest = b'0';
    let mut position = 0;
    for (i, c) in iter.enumerate() {
        if c > highest {
            highest = c;
            position = i;
        }
    }

    (highest - b'0', position)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../inputs/03.ex");

    #[test]
    fn test_example() {
        let (pt1, pt2) = run_on_input(EXAMPLE_INPUT);
        assert_eq!(357, pt1);
        assert_eq!(0, pt2);
    }
}
