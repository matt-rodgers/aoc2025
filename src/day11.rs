use std::collections::HashMap;

use crate::Aoc;

const INPUT: &str = include_str!("../inputs/11.in");

pub struct Day11;

impl Aoc for Day11 {
    fn run(&self) -> (String, String) {
        let (pt1, pt2) = run_on_input(INPUT);
        (pt1.to_string(), pt2.to_string())
    }
}

fn run_on_input(input: &str) -> (u64, u64) {
    let devices: HashMap<&str, Vec<&str>> = input
        .trim()
        .lines()
        .map(|line| {
            let (device, rest) = line.split_once(": ").unwrap();
            let outputs = rest.split_whitespace().collect();
            (device, outputs)
        })
        .collect();

    let pt1 = count_paths("you", "out", &devices);

    // If we assume there are no cycles, we can only go from either fft -> dac OR dac --> fft.
    // Figure out which one, and then go from svr --> xxx --> xxx --> out.
    let dac_to_fft_paths = count_paths("dac", "fft", &devices);
    let fft_to_dac_paths = count_paths("fft", "dac", &devices);
    let pt2 = if dac_to_fft_paths == 0 {
        count_paths("svr", "fft", &devices) * fft_to_dac_paths * count_paths("dac", "out", &devices)
    } else if fft_to_dac_paths == 0 {
        count_paths("svr", "dac", &devices) * dac_to_fft_paths * count_paths("fft", "out", &devices)
    } else {
        panic!("Either dac --> fft or fft --> dac must be zero to avoid cycles")
    };

    (pt1, pt2)
}

fn count_paths(from: &str, to: &str, edges: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut memo = HashMap::new();
    count_paths_memo(from, to, edges, &mut memo)
}

fn count_paths_memo<'a>(
    from: &'a str,
    to: &'a str,
    edges: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    match memo.get(from) {
        Some(n) => *n,
        None => match edges.get(from) {
            None => 0,
            Some(outputs) => {
                let n = outputs
                    .iter()
                    .map(|output| {
                        if output == &to {
                            1
                        } else {
                            count_paths_memo(output, to, edges, memo)
                        }
                    })
                    .sum();
                memo.insert(from, n);
                n
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../inputs/11.ex");
    const EXAMPLE_INPUT_2: &str = include_str!("../inputs/11.2.ex");

    #[test]
    fn test_example() {
        let (pt1, _) = run_on_input(EXAMPLE_INPUT);
        assert_eq!(5, pt1);
    }

    #[test]
    fn test_example_2() {
        let (_, pt2) = run_on_input(EXAMPLE_INPUT_2);
        assert_eq!(2, pt2);
    }
}
