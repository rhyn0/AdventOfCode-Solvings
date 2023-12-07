use itertools::Itertools;

use crate::etc::{Solution, SolutionPair};
use std::{cell::RefCell, fs::read_to_string, ops::Range, str::FromStr};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("inputs/day-05.txt")).unwrap();

    (Solution::from(part1(&input)), Solution::from(part2(&input)))
}
fn part1(input: &str) -> u64 {
    let problem = Problem::from_str(input).unwrap();
    *problem.get_seed_locations().iter().min().unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    seeds: Vec<u64>,
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

                rule number() -> u64
                    = n:$(['0'..='9']+) { n.parse().unwrap() }

                rule seeds() -> Vec<u64>
                    = "seeds:" __ seed:(number() ++ __) { seed }

                rule map_title() -> String
                    = title:$(['a'..='z'| '-']+) __ "map:" { title.to_string() }

                rule map_line() -> (Range<u64>, u64)
                    = destination:number() __ source:number() __ range:number() { (source..(source + range), destination) }

                rule map_lines() -> Vec<(Range<u64>, u64)>
                    = lines:(map_line() ++ newline()) { lines }

                rule map() -> AlmanacMap
                    = title:map_title() newline() map:map_lines() { AlmanacMap { title, map } }

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
    fn get_seed_locations(&self) -> Vec<u64> {
        self.seeds
            .iter()
            .map(|seed| self.maps.iter().fold(*seed, |acc, map| map.get_result(acc)))
            .collect_vec()
    }
    fn get_paired_seed_locations(&self) -> impl Iterator<Item = u64> + '_ {
        self.seeds
            .iter()
            .copied()
            .tuples::<(u64, u64)>()
            .map(|(start, length)| (start..(start + length)))
            .flat_map(|seed_range| {
                self.maps.iter().fold(vec![seed_range], |acc, map| {
                    acc.into_iter()
                        .flat_map(|curr_seed_range| {
                            let curr_seed_range_cell = RefCell::new(curr_seed_range);
                            let map_range_output = map
                                .map
                                .iter()
                                .take_while(|_| !curr_seed_range_cell.borrow().is_empty())
                                .fold(
                                    Vec::with_capacity(6),
                                    |mut from_acc, (map_range, destination_start)| {
                                        let mut curr_seed_range = curr_seed_range_cell.borrow_mut();
                                        if curr_seed_range.start < map_range.start {
                                            // there are seeds that are outside the map range
                                            // these are mapped to the same location
                                            // so we save this range and trim down our current one
                                            from_acc.push(
                                                curr_seed_range.start
                                                    ..(curr_seed_range.end.min(map_range.start)),
                                            );
                                            curr_seed_range.start = map_range.start;
                                        }

                                        // length is number of seeds that we are mapping in this iteration
                                        // so from start of the seed range to the minimum of the end of the seed range and the end of the map range
                                        let len = curr_seed_range
                                            .end
                                            .min(map_range.end)
                                            .saturating_sub(curr_seed_range.start);
                                        if len > 0 {
                                            let mapped_to = *destination_start
                                                + curr_seed_range.start
                                                - map_range.start;
                                            from_acc.push(mapped_to..(mapped_to + len));
                                            curr_seed_range.start += len;
                                        }
                                        from_acc
                                    },
                                );
                            // if there is no output, it is because that almanac map doesn't alter the input values
                            // so we just return the input range
                            if map_range_output.is_empty() {
                                vec![curr_seed_range_cell.into_inner()]
                            } else {
                                map_range_output
                            }
                        })
                        .collect()
                })
            })
            .map(|range| range.start)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AlmanacMap {
    // bounded ranges mapped to the offset for this specific map
    title: String,
    map: Vec<(Range<u64>, u64)>,
}

impl AlmanacMap {
    fn get_result(&self, input: u64) -> u64 {
        for (range, mapped_to_start) in &self.map {
            if range.contains(&input) {
                // distance traveled in input range
                let distance = input - range.start;
                return mapped_to_start + distance;
            }
        }
        // if seed is not contained by any range, it is a no op
        input
    }
}

fn part2(input: &str) -> u64 {
    let mut problem = Problem::from_str(input).unwrap();
    problem
        .maps
        .iter_mut()
        .for_each(|map| map.map.sort_by_key(|(range, _)| range.start));
    problem.get_paired_seed_locations().min().unwrap()
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
