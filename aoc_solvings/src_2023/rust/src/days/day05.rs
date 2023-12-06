use itertools::Itertools;

use crate::etc::{Solution, SolutionPair};
use std::{fs::read_to_string, ops::Range, str::FromStr};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("inputs/day-05.txt")).unwrap();

    (Solution::from(part1(&input)), Solution::from(part2(&input)))
}
fn part1(input: &str) -> i64 {
    let problem = Problem::from_str(input).unwrap();
    *problem.get_seed_locations().iter().min().unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    seeds: Vec<i64>,
    maps: Vec<AlmanacMap>,
}

impl FromStr for Problem {
    type Err = peg::error::ParseError<peg::str::LineCol>;
    #[allow(clippy::ignored_unit_patterns)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        peg::parser! {
            grammar almanac_parser() for str {
                rule newline()
                    = ("\r"?"\n")+
                rule __()
                    = [' ' | '\t']+

                rule number() -> i64
                    = n:$(['0'..='9']+) { n.parse().unwrap() }

                rule seeds() -> Vec<i64>
                    = "seeds:" __ seed:(number() ++ __) { seed }

                rule map_title() -> String
                    = title:$(['a'..='z'| '-']+) __ "map:" { title.to_string() }

                rule map_line() -> (Range<i64>, i64)
                    = destination:number() __ source:number() __ range:number() { (source..(source + range), destination - source) }

                rule map_lines() -> Vec<(Range<i64>, i64)>
                    = lines:(map_line() ++ newline()) { lines }

                rule map() -> AlmanacMap
                    = title:map_title() newline() map:map_lines() { AlmanacMap { title: title.clone(), map, is_location: title.contains("location") } }

                #[no_eof]
                pub rule problem() -> Problem
                    = seeds:seeds() newline() maps:(map() ++ newline())  { Problem { seeds, maps } }
            }
        }
        match almanac_parser::problem(s) {
            Ok(problem) => Ok(problem),
            Err(e) => Err(e),
        }
    }
}

impl Problem {
    fn get_seed_locations(&self) -> Vec<i64> {
        self.seeds
            .iter()
            .map(|seed| self.maps.iter().fold(*seed, |acc, map| map.get_result(acc)))
            .collect_vec()
    }
    fn get_paired_seed_locations(&self) -> Vec<i64> {
        // TODO: use interval collapsing to make this more efficient
        self.seeds
            .iter()
            .copied()
            .tuples::<(i64, i64)>()
            .flat_map(|(start, length)| (start..(start + length)).collect_vec())
            .map(|seed| self.maps.iter().fold(seed, |acc, map| map.get_result(acc)))
            .collect_vec()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AlmanacMap {
    // bounded ranges mapped to the offset for this specific map
    title: String,
    map: Vec<(Range<i64>, i64)>,
    is_location: bool,
}

impl AlmanacMap {
    fn get_result(&self, input: i64) -> i64 {
        for (range, offset) in &self.map {
            if range.contains(&input) {
                return input + offset;
            }
        }
        // if seed is not contained by any range, it is a no op
        input
    }
}

fn part2(input: &str) -> i64 {
    let problem = Problem::from_str(input).unwrap();
    *problem.get_paired_seed_locations().iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    #[test]
    fn test_parse() {
        let input = indoc! {"
            seeds: 1 2 3 4 5

            seed-to-soil map:
            1 0 1

            soil-to-location map:
            2 4 1
        "};
        let problem = Problem::from_str(input).unwrap();
        assert_eq!(problem.seeds, vec![1, 2, 3, 4, 5]);
        assert_eq!(problem.maps.len(), 2);
        assert_eq!(problem.maps[0].title, "seed-to-soil");
        assert_eq!(problem.maps[0].map.len(), 1);
        assert_eq!(problem.maps[1].is_location, true);
    }

    #[test]
    fn test_part1() {
        let input = read_to_string("./inputs/day-05.example.txt").unwrap();
        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("./inputs/day-05.example.txt").unwrap();
        assert_eq!(part2(&input), 46);
    }
}
