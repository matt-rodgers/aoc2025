use crate::Aoc;

const INPUT: &str = include_str!("../inputs/03.in");

pub struct Day03;

impl Aoc for Day03 {
    fn run(&self) -> (String, String) {
        let (pt1, pt2) = run_on_input(INPUT);
        (pt1.to_string(), pt2.to_string())
    }
}

fn run_on_input(input: &str) -> (u64, u64) {
    let pt1 = input
        .trim()
        .lines()
        .map(|line| make_highest_number(line.as_bytes(), 2))
        .sum();

    let pt2 = input
        .trim()
        .lines()
        .map(|line| make_highest_number(line.as_bytes(), 12))
        .sum();

    (pt1, pt2)
}

fn make_highest_number(line: &[u8], ndigits: usize) -> u64 {
    assert!(line.len() >= ndigits);

    let mut n = 0;
    let mut start_index = 0;

    for i in (0..ndigits).rev() {
        let end_index = line.len() - i;
        let (highest, position) = first_maximum(line[start_index..end_index].iter().copied());
        n = (n * 10) + highest as u64;
        start_index += position + 1;
    }

    n
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
        assert_eq!(3121910778619, pt2);
    }
}
