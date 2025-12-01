const INPUT: &str = include_str!("../inputs/01.in");

pub fn run() {
    let (pt1, pt2) = run_on_input(INPUT);
    println!("pt1: {}, pt2: {}", pt1, pt2);
}

fn run_on_input(input: &str) -> (usize, usize) {
    let mut pointing_at = 50;
    let mut pt2 = 0;

    let pt1 = input
        .trim()
        .lines()
        .map(|line| {
            let (direction, amount) = line.split_at(1);
            let amount: i64 = amount.parse().unwrap();

            // If we are already at zero and rotate left, div_euclid would count us as crossing zero
            // but we have already counted this in the previous step.
            if direction == "L" && pointing_at == 0 {
                pt2 -= 1;
            }

            match direction {
                "R" => pointing_at += amount,
                "L" => pointing_at -= amount,
                _ => panic!("should only be L or R as first character"),
            };

            let zero_crossings = pointing_at.div_euclid(100).abs();
            pointing_at = pointing_at.rem_euclid(100);
            pt2 += zero_crossings as usize;

            // Handle special case where we land at zero without crossing it
            if direction == "L" && pointing_at == 0 {
                pt2 += 1;
            }

            pointing_at
        })
        .filter(|n| *n == 0)
        .count();

    (pt1, pt2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../inputs/01.ex");

    #[test]
    fn test_example() {
        let (pt1, pt2) = run_on_input(EXAMPLE_INPUT);
        assert_eq!(3, pt1);
        assert_eq!(6, pt2);
    }

    #[test]
    fn test_multiple_crossings() {
        let input = "R1000";
        let (_, pt2) = run_on_input(input);
        assert_eq!(10, pt2);

        let input = "L1000";
        let (_, pt2) = run_on_input(input);
        assert_eq!(10, pt2);
    }

    #[test]
    fn test_correctly_handles_left_rotate_to_exactly_0() {
        let input = "L50";
        let (_, pt2) = run_on_input(input);
        assert_eq!(1, pt2);

        let input = "L150";
        let (_, pt2) = run_on_input(input);
        assert_eq!(2, pt2);
    }
}
