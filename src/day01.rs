const INPUT: &str = include_str!("../inputs/01.in");

pub fn run() {
    let (pt1, pt2) = run_on_input(INPUT);
    println!("pt1: {}, pt2: {}", pt1, pt2);
}

fn run_on_input(input: &str) -> (usize, usize) {
    let mut pointing_at = 50;

    let pt1 = input
        .trim()
        .lines()
        .map(|line| {
            let (direction, amount) = line.split_at(1);
            let amount: i64 = amount.parse().unwrap();

            match direction {
                "R" => pointing_at += amount,
                "L" => pointing_at -= amount,
                _ => panic!("should only be L or R as first character"),
            };

            pointing_at = pointing_at.rem_euclid(100);
            pointing_at
        })
        .filter(|n| *n == 0)
        .count();

    (pt1, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../inputs/01.ex");

    #[test]
    fn test_example() {
        let (pt1, _) = run_on_input(EXAMPLE_INPUT);
        assert_eq!(3, pt1);
    }
}
