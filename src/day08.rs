use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::Aoc;

const INPUT: &str = include_str!("../inputs/08.in");

pub struct Day08;

impl Aoc for Day08 {
    fn run(&self) -> (String, String) {
        let (pt1, pt2) = run_on_input(INPUT, 1000);
        (pt1.to_string(), pt2.to_string())
    }
}

fn run_on_input(input: &str, merge_count: usize) -> (usize, usize) {
    // Structure the data as a map of circuit_id: nodes_in_circuit
    let mut circuits: HashMap<usize, HashSet<Point3D>> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| (i, HashSet::from([line.parse().unwrap()])))
        .collect();

    let mut existing_connections = HashSet::new();

    let mut pt1 = 0;
    let mut i = 1;
    let mut latest_merge: Option<ClosestPair> = None;

    while circuits.len() > 1 {
        let merged = merge_closest(&mut circuits, &mut existing_connections);
        latest_merge.replace(merged);

        if i == merge_count {
            let mut sizes: Vec<_> = circuits.values().map(|v| v.len()).collect();
            sizes.sort_unstable();
            pt1 = (0..3).fold(1, |accum, _| accum * sizes.pop().unwrap());
        }
        i += 1;
    }

    dbg!(&latest_merge);
    let latest_merge = latest_merge.unwrap();
    let pt2 = latest_merge.a.1.x * latest_merge.b.1.x;

    (pt1, pt2 as usize)
}

fn merge_closest(
    circuits: &mut HashMap<usize, HashSet<Point3D>>,
    existing_connections: &mut HashSet<(Point3D, Point3D)>,
) -> ClosestPair {
    let mut closest_pair: Option<ClosestPair> = None;

    for (id, circuit) in circuits.iter() {
        for node in circuit.iter() {
            // For this node, iterate over all nodes in *all* circuits. If we connect two nodes
            // that are already in the same circuit, this still counts as a connection but does not
            // change the list of circuits at all.
            for (other_id, other_circuit) in circuits.iter() {
                for other_node in other_circuit.iter() {
                    if node != other_node
                        && !are_already_connected(
                            existing_connections,
                            node.clone(),
                            other_node.clone(),
                        )
                    {
                        let distance = node.euclidean_distance(other_node);
                        let should_replace = match &closest_pair {
                            None => true,
                            Some(closest) => distance < closest.distance,
                        };

                        if should_replace {
                            closest_pair.replace(ClosestPair {
                                a: (*id, node.clone()),
                                b: (*other_id, other_node.clone()),
                                distance,
                            });
                        }
                    }
                }
            }
        }
    }

    // Merge the closest pairs, if they're not already part of the same circuit
    let closest = closest_pair.take().unwrap();
    add_connection(
        existing_connections,
        closest.a.1.clone(),
        closest.b.1.clone(),
    );
    if closest.a.0 != closest.b.0 {
        let a = circuits.remove(&closest.a.0).unwrap();
        circuits.entry(closest.b.0).and_modify(|set| set.extend(a));
    }

    closest
}

fn are_already_connected(
    existing_connections: &HashSet<(Point3D, Point3D)>,
    a: Point3D,
    b: Point3D,
) -> bool {
    let (larger, smaller) = if a > b { (a, b) } else { (b, a) };
    existing_connections.contains(&(larger, smaller))
}

fn add_connection(existing_connections: &mut HashSet<(Point3D, Point3D)>, a: Point3D, b: Point3D) {
    let (larger, smaller) = if a > b { (a, b) } else { (b, a) };
    existing_connections.insert((larger, smaller));
}

#[derive(Debug, Clone)]
struct ClosestPair {
    a: (usize, Point3D),
    b: (usize, Point3D),
    distance: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Point3D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let (x, yz) = s.split_once(',').unwrap();
        let (y, z) = yz.split_once(',').unwrap();
        Ok(Point3D {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        })
    }
}

impl Point3D {
    fn euclidean_distance(&self, other: &Point3D) -> f64 {
        let xdiff = self.x - other.x;
        let ydiff = self.y - other.y;
        let zdiff = self.z - other.z;
        let sum_of_squares = (xdiff.pow(2) + ydiff.pow(2) + zdiff.pow(2)) as f64;
        sum_of_squares.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../inputs/08.ex");

    #[test]
    fn test_example() {
        let (pt1, pt2) = run_on_input(EXAMPLE_INPUT, 10);
        assert_eq!(40, pt1);
        assert_eq!(25272, pt2);
    }
}
