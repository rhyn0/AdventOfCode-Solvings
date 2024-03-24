use crate::etc::{Solution, SolutionPair};
use std::{cmp::Ordering, collections::HashMap, fs::read_to_string, str::FromStr};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("inputs/day-08.txt")).unwrap();
    (Solution::from(part1(&input)), Solution::from(part2(&input)))
}

fn part1(input: &str) -> u64 {
    let problem = Problem::from_str(input).unwrap();
    // start at 'AAA'
    let current = "AAA".to_string();
    // while current != "ZZZ" {
    //     let (left, right) = problem.nodes.get(&current).unwrap();
    //     let direction = instructions.next().unwrap();
    //     current = match direction {
    //         Direction::Left => left.clone(),
    //         Direction::Right => right.clone(),
    //     };
    //     steps += 1;
    // }
    // steps
    walk_to_z_site(&problem, &current, None)
}

fn walk_to_z_site(problem: &Problem, start: &str, target: Option<&str>) -> u64 {
    // follow the instructions to choose LEFT (0) or RIGHT (1)
    // of each node after doing the hash map lookup
    // cycle the instructions if necessary
    // repeat until we reach 'ZZZ'
    // count the number of steps to reach the end
    let mut steps = 0;
    let mut current = start;
    let target = target.unwrap_or("ZZZ");
    let mut instructions_idx = problem.instructions.iter().cycle();
    loop {
        let (left, right) = problem.nodes.get(current).unwrap();
        current = match instructions_idx.next().unwrap() {
            Direction::Left => &left,
            Direction::Right => &right,
        };
        steps += 1;
        if current.ends_with(target) {
            break;
        }
    }

    steps
}

#[derive(Debug, Clone)]
pub struct Problem {
    instructions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

impl FromStr for Problem {
    type Err = peg::error::ParseError<peg::str::LineCol>;
    #[allow(clippy::ignored_unit_patterns)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        peg::parser! {
            grammar map_parser() for str {
                pub rule problem() -> Problem
                    = instructions:instruction() newline()  nodes:node() ++ "\n" newline()? ![_] { Problem { instructions, nodes: nodes.iter().cloned().collect() } }

                rule instruction() -> Vec<Direction>
                    = dir_str:(['R' | 'L']+) {
                        dir_str.iter().map(|c| match c {
                            'R' => Direction::Right,
                            'L' => Direction::Left,
                            _ => unreachable!(),
                        }).collect()
                    }

                rule node() -> (String, (String, String))
                    = from:node_name() __ "=" __ "(" to1:node_name() "," __ to2:node_name() ")" { (from, (to1, to2)) }

                rule node_name() -> String
                    = name:(['A'..='Z'| '0'..='9']+) { name.iter().collect() }

                rule __()
                    = [' ' | '\t']+

                rule newline()
                    = ['\n' | '\r']+
            }
        }
        match map_parser::problem(s) {
            Ok(problem) => Ok(problem),
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

fn part2(input: &str) -> u64 {
    // The number of nodes to start at is going to cause a huge explosion in processing time
    // considering the matching of their steps.
    // but we find the number of steps for a single node to reach its Z site
    // and then find the least common multiple of all of those sites to get the answer
    let problem = Problem::from_str(input).unwrap();
    // starting nodes are the ones that end in A
    let start_nodes = problem.nodes.keys().filter(|site| site.ends_with('A'));

    // then find the number of steps needed to get each site to its Z (end)
    let mut steps_for_start_points =
        start_nodes.map(|site| walk_to_z_site(&problem, site, Some("Z")));

    // return the least common multiple of step number per start point
    let init_steps = steps_for_start_points.next().unwrap();

    steps_for_start_points.fold(init_steps, lcm)
}

fn gcd(a: u64, b: u64) -> u64 {
    match a.cmp(&b) {
        Ordering::Equal => a,
        Ordering::Greater => {
            if b == 0 {
                a
            } else {
                gcd(a % b, b)
            }
        }
        Ordering::Less => {
            if a == 0 {
                b
            } else {
                gcd(a, b % a)
            }
        }
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = read_to_string("inputs/day-08.example.txt").unwrap();
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_parse_example() {
        let input = read_to_string("inputs/day-08.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        assert_eq!(
            problem.instructions,
            vec![Direction::Right, Direction::Left]
        );
        assert_eq!(problem.nodes.len(), 7);
        assert!(problem.nodes.contains_key("AAA"));
        assert!(problem.nodes.contains_key("ZZZ"));
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "};
        assert_eq!(part2(&input), 6);
    }
}
