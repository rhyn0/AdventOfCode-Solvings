use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::etc::{Solution, SolutionPair};

pub fn solve(path: Option<&str>) -> SolutionPair {
    let input = std::fs::read_to_string(path.unwrap_or("inputs/day-25.txt")).unwrap();
    (Solution::from(part1(&input)), Solution::from(0))
}

fn part1(input: &str) -> usize {
    let problem: Problem = input.parse().unwrap();
    let desired_diff = 3;
    let mut s_graph = problem.vertex.keys().cloned().collect::<HashSet<_>>();
    while s_graph
        .iter()
        .map(|k| problem.vertex[k].difference(&s_graph).count())
        .sum::<usize>()
        != desired_diff
    {
        let max_node = s_graph
            .iter()
            .max_by_key(|&k| problem.vertex[k].difference(&s_graph).count())
            .unwrap()
            .clone();
        s_graph.remove(&max_node);
    }
    let black_tree_len = problem
        .vertex
        .keys()
        .cloned()
        .collect::<HashSet<_>>()
        .difference(&s_graph)
        .count();
    black_tree_len * s_graph.len()
}

#[derive(Debug, Clone)]
struct Problem {
    vertex: HashMap<String, HashSet<String>>,
}

impl FromStr for Problem {
    type Err = ();

    /// Example line looks like below:
    ///     id: id1 id2 id3
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vertex = HashMap::new();

        for line in s.lines() {
            let (from, tos) = line.split_once(':').ok_or(())?;
            let from = from.trim().to_string();
            for to in tos.trim().split(' ') {
                vertex
                    .entry(to.to_string())
                    .or_insert_with(HashSet::new)
                    .insert(from.clone());
                vertex
                    .entry(from.clone())
                    .or_insert_with(HashSet::new)
                    .insert(to.to_string());
            }
        }

        Ok(Self { vertex })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part1() {
        let input = read_to_string("inputs/day-25.example.txt").unwrap();
        assert_eq!(part1(&input), 54);
    }
    #[test]
    fn test_parse() {
        let input = read_to_string("inputs/day-25.example.txt").unwrap();
        let problem: Problem = input.parse().unwrap();
        assert_eq!(problem.vertex.len(), 15);
    }
}
