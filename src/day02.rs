use crate::Aoc;

const INPUT: &str = include_str!("../inputs/02.in");

pub struct Day02;

impl Aoc for Day02 {
    fn run(&self) -> (String, String) {
        let (pt1, pt2) = run_on_input(INPUT);
        (pt1.to_string(), pt2.to_string())
    }
}

fn run_on_input(input: &str) -> (usize, usize) {
    let pt1 = input
        .trim()
        .split(',')
        .map(|range| {
            let (a, b) = range.split_once('-').unwrap();

            let ndigits_split = a.len() / 2;

            let mut first_half = if (a.len() % 2) != 0 {
                // If we had an odd number of digits, we should start at the next power of 10. e.g.
                // if we have 912, rather than the first half being 9 and the first n being 99,
                // we should start from 10 so that the first n is 1000 to avoid checking a load of
                // numbers that cannot possibly be the solution
                10usize.pow(ndigits_split as u32)
            } else {
                a.split_at(ndigits_split).0.parse().unwrap()
            };

            let mut count = 0;

            let a: usize = a.parse().unwrap();
            let b: usize = b.parse().unwrap();

            loop {
                let ndigits = count_base10_digits(first_half);
                let n = first_half * 10usize.pow(ndigits as u32) + first_half;

                if n > b {
                    break;
                }

                if n >= a {
                    count += n;
                }

                first_half += 1;
            }

            count
        })
        .sum();

    (pt1, 0)
}

fn count_base10_digits(n: usize) -> usize {
    let mut check_val = 10;
    let mut count = 1;
    while n >= check_val {
        count += 1;
        check_val *= 10;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../inputs/02.ex");

    #[test]
    fn test_example() {
        let (pt1, _pt2) = run_on_input(EXAMPLE_INPUT);
        assert_eq!(1227775554, pt1);
        // assert_eq!(0, pt2);
    }

    #[test]
    fn test_count_base10_digits() {
        assert_eq!(1, count_base10_digits(6));
        assert_eq!(2, count_base10_digits(23));
        assert_eq!(3, count_base10_digits(499));
        assert_eq!(4, count_base10_digits(1000));
        assert_eq!(11, count_base10_digits(12345678901));
    }
}
