use itertools::Itertools;

use crate::etc::{Solution, SolutionPair};
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    str::FromStr,
};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input_str = read_to_string(input_path.unwrap_or("./inputs/day-04.txt")).unwrap();
    (
        Solution::from(part1(&input_str)),
        Solution::from(part2(&input_str)),
    )
}

fn wins_per_scratcher(scratcher: &Scratcher) -> u32 {
    u32::try_from(
        scratcher
            .winning_numbers
            .iter()
            .filter(|&number| scratcher.player_numbers.contains(number))
            .count(),
    )
    .unwrap()
}

fn part1(input: &str) -> u32 {
    let problem = Problem::from_str(input).unwrap();
    problem
        .scratchers
        .iter()
        .map(|scratcher| {
            let num_winning = wins_per_scratcher(scratcher);
            if num_winning == 0 {
                0
            } else {
                2_u32.pow(num_winning - 1)
            }
        })
        .sum()
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Problem {
    scratchers: Vec<Scratcher>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Scratcher {
    id: u32,
    winning_numbers: HashSet<u32>,
    player_numbers: HashSet<u32>,
}

impl FromStr for Problem {
    type Err = peg::error::ParseError<peg::str::LineCol>;
    #[allow(clippy::ignored_unit_patterns)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // input is a list of scratchers which exist on their own line
        // a game is identified by the prefix of Card #:
        // followed by a list of numbers which are the winning numbers
        // seperated by a |
        // followed by a list of numbers which are the players numbers
        peg::parser!(
            grammar scratcher_parser() for str {
            rule number() -> u32
                = n:$(['0'..='9']+) { n.parse().unwrap() }

            rule __()
                = [' ' | '\n' | '\t']+

            rule numbers() -> HashSet<u32>
                = n:number() ** __ { n.into_iter().collect() }

            rule scratcher() -> Scratcher
                = "Card" __ id:number() ":" __ winning_numbers:numbers() __ "|" __ player_numbers:numbers() {
                    Scratcher {
                        id,
                        winning_numbers,
                        player_numbers,
                    }
                }

            #[no_eof]
            pub rule scratchers(p: &mut Problem)
                = s:scratcher() ++ "\n" { p.scratchers = s; }
        });
        let mut problem = Self::default();

        match scratcher_parser::scratchers(s, &mut problem) {
            Ok(()) => Ok(problem),
            Err(e) => Err(e),
        }
    }
}

fn part2(input: &str) -> u32 {
    let problem = Problem::from_str(input).unwrap();
    let wins_per_scratcher = problem
        .scratchers
        .iter()
        .map(|scratcher| (scratcher.id, wins_per_scratcher(scratcher)))
        .collect::<HashMap<u32, u32>>();

    let mut num_scratchers = problem
        .scratchers
        .iter()
        // 1 here counts for the original scratcher
        .map(|s| (s.id, 1))
        .collect::<HashMap<u32, u32>>();
    wins_per_scratcher
        .iter()
        // make sure we do Card 1 before we do Card 2
        .sorted_by_key(|&(k, _)| k)
        .for_each(|(id, wins)| {
            let current_num_scratcher = num_scratchers.get(id).unwrap();
            for _ in 0..*current_num_scratcher {
                for copy_id in (id + 1)..=(*id + *wins) {
                    let num_scratcher = num_scratchers.get_mut(&copy_id).unwrap();
                    *num_scratcher += 1;
                }
            }
        });
    // return total number of scratcher cards
    num_scratchers.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example() {
        let input = read_to_string("./inputs/day-04.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        assert_eq!(problem.scratchers.len(), 6);
        assert_eq!(problem.scratchers[0].id, 1);
        assert_eq!(problem.scratchers[0].winning_numbers.len(), 5);
        assert_eq!(problem.scratchers[0].player_numbers.len(), 8);
    }

    #[test]
    fn test_part1() {
        let input = read_to_string("./inputs/day-04.example.txt").unwrap();
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("./inputs/day-04.example.txt").unwrap();
        assert_eq!(part2(&input), 30);
    }
}
