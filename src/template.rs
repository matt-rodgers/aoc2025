use crate::Aoc;

const INPUT: &str = include_str!("../inputs/XX.in");

pub struct DayXX;

impl Aoc for DayXX {
    fn run(&self) -> (String, String) {
        let (pt1, pt2) = run_on_input(INPUT);
        (pt1.to_string(), pt2.to_string())
    }
}

fn run_on_input(input: &str) -> (usize, usize) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../inputs/XX.ex");

    #[test]
    fn test_example() {
        let (pt1, pt2) = run_on_input(EXAMPLE_INPUT);
        assert_eq!(0, pt1);
        assert_eq!(0, pt2);
    }
}
