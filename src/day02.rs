use std::collections::HashSet;

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
    let mut pt1 = 0;
    let mut pt2 = 0;

    for range in input.trim().split(',') {
        let (a, b) = range.split_once('-').unwrap();
        let an: usize = a.parse().unwrap();
        let bn: usize = b.parse().unwrap();

        // Part 1
        let start_at: usize = a.split_at(a.len() / 2).0.parse().unwrap_or(1);
        for n in start_at.. {
            let repeat_twice = RepeatPatternIter::new(n).nth(1).unwrap();

            if repeat_twice > bn {
                break;
            }

            if repeat_twice >= an {
                pt1 += repeat_twice;
            }
        }

        // Part 2
        let mut already_found = HashSet::new();
        let upper_bound = b.split_at(b.len().div_ceil(2)).0.parse::<usize>().unwrap() + 1;
        for n in 1..=upper_bound {
            for pattern in RepeatPatternIter::new(n).skip(1) {
                if pattern > bn {
                    break;
                }

                if pattern >= an && !already_found.contains(&pattern) {
                    already_found.insert(pattern);
                    pt2 += pattern;
                }
            }
        }
    }

    (pt1, pt2)
}

#[derive(Debug)]
struct RepeatPatternIter {
    n: usize,
    repeats: usize,
    digits: usize,
}

impl RepeatPatternIter {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            repeats: 1,
            digits: count_base10_digits(n),
        }
    }
}

impl Iterator for RepeatPatternIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut item = self.n;
        for i in 1..self.repeats {
            item += self.n * 10usize.pow((self.digits * i) as u32);
        }
        self.repeats += 1;
        Some(item)
    }
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
        let (pt1, pt2) = run_on_input(EXAMPLE_INPUT);
        assert_eq!(1227775554, pt1);
        assert_eq!(4174379265, pt2);
    }

    #[test]
    fn test_count_base10_digits() {
        assert_eq!(1, count_base10_digits(6));
        assert_eq!(2, count_base10_digits(23));
        assert_eq!(3, count_base10_digits(499));
        assert_eq!(4, count_base10_digits(1000));
        assert_eq!(11, count_base10_digits(12345678901));
    }

    #[test]
    fn test_repeat_pattern_iter() {
        let mut rp = RepeatPatternIter::new(1);
        assert_eq!(Some(1), rp.next());
        assert_eq!(Some(11), rp.next());
        assert_eq!(Some(111), rp.next());
        assert_eq!(Some(1111), rp.next());

        let mut rp = RepeatPatternIter::new(321);
        assert_eq!(Some(321), rp.next());
        assert_eq!(Some(321321), rp.next());
        assert_eq!(Some(321321321), rp.next());
        assert_eq!(Some(321321321321), rp.next());
    }
}
