use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

use itertools::Itertools;

use crate::Aoc;

const INPUT: &str = include_str!("../inputs/08.in");

pub struct Day08;

impl Aoc for Day08 {
    fn run(&self) -> (String, String) {
        let (pt1, pt2) = run_on_input(INPUT, 1000);
        (pt1.to_string(), pt2.to_string())
    }
}

fn run_on_input(input: &str, merge_count: usize) -> (usize, i64) {
    let nodes: Vec<Point3D> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut connections: Vec<(f64, &Point3D, &Point3D)> = nodes
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let distance = a.euclidean_distance(b);
            (distance, a, b)
        })
        .collect();

    // Sort largest to smallest
    connections.sort_unstable_by(|a, b| b.0.total_cmp(&a.0));

    let mut ds = DisjointSet::new(nodes.clone());

    // Make the number of connections required for part 1
    for _ in 0..merge_count {
        let (_, a, b) = connections.pop().unwrap();
        ds.merge(a, b);
    }

    // Get the sets of connected nodes at this point in time, and multiply the largest 3.
    let mut sets: Vec<_> = ds.as_sets().into_values().map(|set| set.len()).collect();
    sets.sort_unstable();
    let pt1 = sets.iter().rev().take(3).product();

    // Do the rest of the connections until we have one giant set
    let pt2 = loop {
        let (_, a, b) = connections
            .pop()
            .expect("must reach pt2 answer before exhausting connections");

        if matches!(ds.merge(a, b), Some(FullyMerged)) {
            break a.x * b.x;
        }
    };

    (pt1, pt2)
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

#[derive(Debug, Clone)]
struct DisjointSet<T>(HashMap<T, DisjointSetItem<T>>);

impl<T: Hash + Eq + Clone> DisjointSet<T> {
    pub fn new(items: impl IntoIterator<Item = T>) -> Self {
        Self(
            items
                .into_iter()
                .map(|item| (item, Default::default()))
                .collect(),
        )
    }

    /// Find the root node of the tree containing this node
    fn find_root(&self, node: T) -> Option<T> {
        let item = self.0.get(&node)?;

        if let Some(ref parent) = item.parent {
            self.find_root(parent.clone())
        } else {
            Some(node)
        }
    }

    /// Merge the trees containing nodes a and b
    pub fn merge(&mut self, a: &T, b: &T) -> Option<FullyMerged> {
        let root_a = self
            .find_root(a.clone())
            .expect("must merge items that are in set");
        let root_b = self
            .find_root(b.clone())
            .expect("must merge items that are in set");

        if root_a == root_b {
            if self.0.get(&root_a).unwrap().size == self.0.len() as u64 {
                return Some(FullyMerged);
            } else {
                return None;
            }
        }

        let size_a = self.0.get(&root_a).unwrap().size;
        let size_b = self.0.get(&root_b).unwrap().size;
        let total_size = size_a + size_b;

        if size_a > size_b {
            *self.0.get_mut(&root_b).unwrap() = DisjointSetItem {
                parent: Some(root_a.clone()),
                size: total_size,
            };
            self.0.get_mut(&root_a).unwrap().size = total_size;
        } else {
            *self.0.get_mut(&root_a).unwrap() = DisjointSetItem {
                parent: Some(root_b.clone()),
                size: total_size,
            };
            self.0.get_mut(&root_b).unwrap().size = total_size;
        }

        if total_size == self.0.len() as u64 {
            Some(FullyMerged)
        } else {
            None
        }
    }

    pub fn as_sets(&self) -> HashMap<T, HashSet<T>> {
        let mut out: HashMap<T, HashSet<T>> = HashMap::new();

        for node in self.0.keys() {
            let root = self.find_root(node.clone()).unwrap();
            out.entry(root).or_default().insert(node.clone());
        }

        out
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FullyMerged;

#[derive(Debug, Clone)]
struct DisjointSetItem<T> {
    pub parent: Option<T>,
    pub size: u64,
}

impl<T> Default for DisjointSetItem<T> {
    fn default() -> Self {
        Self {
            parent: None,
            size: 1,
        }
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

    #[test]
    fn test_disjoint_set() {
        let mut ds = DisjointSet::new(['a', 'b', 'c', 'd', 'e']);
        assert_eq!(None, ds.merge(&'a', &'b'));
        assert_eq!(None, ds.merge(&'c', &'d'));
        assert_eq!(None, ds.merge(&'c', &'e'));

        // These ones already have the same parent
        assert_eq!(None, ds.merge(&'d', &'e'));

        // There should be two sets at this stage
        assert_eq!(ds.as_sets().len(), 2);

        assert_eq!(Some(FullyMerged), ds.merge(&'a', &'d'));
        assert_eq!(ds.as_sets().len(), 1);
    }
}
