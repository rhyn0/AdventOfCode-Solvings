use itertools::Itertools;

use crate::etc::{Solution, SolutionPair};
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    fs::read_to_string,
    num::ParseIntError,
    str::FromStr,
    vec,
};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("inputs/day-12.txt")).unwrap();
    (Solution::from(part1(&input)), Solution::from(part2(&input)))
}

fn part1(input: &str) -> u64 {
    let problem = Problem::from_str(input).unwrap();
    problem
        .springs
        .iter()
        .map(|spring: &SpringArrangement| {
            let memo = &mut HashMap::new();
            num_valid_spring_combinations_per_line(&spring.line.0, &spring.recorded_broken, memo)
        })
        .sum()
}

fn valid_clean_spring_arrange(spring_line: &[SpringType], skip_length: usize) -> bool {
    // at the final clue
    // it's only valid if the leftover spring line after this point is all fixed or unknown
    if skip_length >= spring_line.len() {
        return true;
    }
    spring_line[skip_length..]
        .iter()
        .all(|&s| s != SpringType::Broken)
}

/// Returns the number of valid spring arrangements for the given line and broken groups
/// uses a memoized hashmap to store the number of valid arrangements for a given line and broken groups
fn num_valid_spring_combinations_per_line<'b>(
    spring_line: &[SpringType],
    broken_groups: &'b [usize],
    memo: &mut HashMap<(usize, &'b [usize]), u64>,
) -> u64 {
    // for each line, generate all possible combinations of left padded working springs
    // to the current group of broken springs and then count the number of valid combinations
    // for each line
    // recursive call
    let spring_line_len = spring_line.len();
    let total_broken = broken_groups.iter().sum::<usize>();
    // number of groups to process after this one
    let broken_groups_after = broken_groups.len().saturating_sub(1);
    memo.get(&(spring_line_len, broken_groups))
        .copied()
        .unwrap_or_else(move || {
            let result = match broken_groups {
                [] => panic!("no broken groups"),
                [clue] => {
                    // build all possible combinations of left pad working for the current broken group clue
                    (0..=(spring_line_len - total_broken - broken_groups_after))
                        .filter(|&work_pad| {
                            let spring_string = build_spring_string(work_pad, *clue);
                            let result = valid_spring_arrange(spring_line, &spring_string);
                            result && valid_clean_spring_arrange(spring_line, work_pad + clue + 1)
                        })
                        .count() as u64
                }
                [clue, rest @ ..] => {
                    // build all possible combinations of left pad working for the current broken group clue
                    (0..=(spring_line_len - total_broken - broken_groups_after))
                        .filter_map(|work_pad| {
                            let spring_string = build_spring_string(work_pad, *clue);
                            if valid_spring_arrange(spring_line, &spring_string) {
                                Some(num_valid_spring_combinations_per_line(
                                    &spring_line[work_pad + clue + 1..],
                                    rest,
                                    memo,
                                ))
                            } else {
                                None
                            }
                        })
                        .sum()
                }
            };
            memo.insert((spring_line_len, broken_groups), result);
            result
        })
}

fn build_spring_string(working_pad: usize, broken_group: usize) -> Vec<SpringType> {
    vec![SpringType::Fixed; working_pad]
        .into_iter()
        .chain(vec![SpringType::Broken; broken_group])
        // followed by one fixed to make sure that groups are properly seperated
        .chain([SpringType::Fixed])
        .collect()
}

fn valid_spring_arrange(spring_line: &[SpringType], suggested: &[SpringType]) -> bool {
    // check if the actual line matches the suggested line
    spring_line
        .iter()
        .zip(suggested.iter())
        .all(|(actual, suggested)| *actual == SpringType::Unknown || actual == suggested)
}

#[derive(Debug, Clone)]
pub struct Problem {
    springs: Vec<SpringArrangement>,
}

#[derive(Clone)]
struct SpringLine(Vec<SpringType>);

impl Debug for SpringLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self(springs) = self;
        let mut s = String::new();
        for spring in springs {
            s.push(match spring {
                SpringType::Fixed => '.',
                SpringType::Broken => '#',
                SpringType::Unknown => '?',
            });
        }
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone)]
struct SpringArrangement {
    recorded_broken: Vec<usize>,
    line: SpringLine,
}

impl SpringArrangement {
    fn with_repetition(self, repeated: usize) -> Self {
        let length = (self.line.0.len() + 1) * repeated - 1;
        let broken_group_len = self.recorded_broken.len();
        let springs = self.line.0.iter().chain(&[SpringType::Unknown]).cycle();
        let line = springs.take(length).copied().collect_vec();
        let recorded_broken = self
            .recorded_broken
            .iter()
            .cycle()
            .take(broken_group_len * repeated)
            .copied()
            .collect_vec();
        Self {
            line: SpringLine(line),
            recorded_broken,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpringType {
    Fixed,
    Broken,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProblemParseError {
    InvalidSpringType(char),
    InvalidSpringGroupRecord(String),
}

impl Problem {
    pub fn line_from_str(s: &str) -> Result<Vec<SpringType>, ProblemParseError> {
        s.chars()
            .map(|c| match c {
                '#' => Ok(SpringType::Broken),
                '.' => Ok(SpringType::Fixed),
                '?' => Ok(SpringType::Unknown),
                _ => Err(ProblemParseError::InvalidSpringType(c)),
            })
            .collect()
    }
}

impl FromStr for Problem {
    type Err = ProblemParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // springs are arranged on a line
        // each line consists of # broken springs and . fixed springs
        // and there are spots where we don't know the state of the spring ?
        // each line is then summed up at the end as a summary of the broken spring groups
        // which are perfectly accurate
        let springs = s
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut line = line.split(' ');
                let spring_str = line.next().unwrap();
                let recorded_broken = line
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|s| {
                        s.parse::<usize>().map_err(|_: ParseIntError| {
                            ProblemParseError::InvalidSpringGroupRecord(s.to_owned())
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let line_spring_type = Self::line_from_str(spring_str)?;
                // collect contiguous UnknownSpring type into a range
                Ok(SpringArrangement {
                    recorded_broken,
                    line: SpringLine(line_spring_type),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { springs })
    }
}

fn part2(input: &str) -> u64 {
    let num_repetitions = 5;
    let problem = Problem::from_str(input).unwrap();
    problem
        .springs
        .iter()
        .map(|s| s.clone().with_repetition(num_repetitions))
        .map(|spring: SpringArrangement| {
            let memo = &mut HashMap::new();
            num_valid_spring_combinations_per_line(&spring.line.0, &spring.recorded_broken, memo)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_to_string("inputs/day-12.example.txt").unwrap();
        assert_eq!(part1(&input), 21);
    }
    #[test]
    fn test_parse() {
        let input = read_to_string("inputs/day-12.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        assert_eq!(problem.springs.len(), 6);
        assert_eq!(problem.springs[0].recorded_broken, vec![1, 1, 3]);
    }
    #[test]
    fn test_valid_spring_arrange() {
        // test what happens when given the first line from example
        // which was ???.### and the broken groups are 1, 1, 3
        // this has only one possible arrangement
        // length of the line is 7, the sum of broken is 5, and there are 2 broken groups after the first
        let actual = Problem::line_from_str("???.###").unwrap();
        let suggested_0 = Problem::line_from_str("#.").unwrap();
        // suggested_below are valid in this test but should not be generated actually since it cuts the second group
        let suggested_1 = Problem::line_from_str(".#.").unwrap();
        let suggested_2 = Problem::line_from_str("..#.").unwrap();
        for suggest in [suggested_0, suggested_1, suggested_2] {
            assert!(valid_spring_arrange(&actual, &suggest));
        }
    }
    #[test]
    fn test_valid_spring_arrange_02() {
        // test what happens when given the first line from example
        // which was .??..??...?##. and the broken groups are 1,1,3
        // this are 4 possible arrangements
        // length of the line is 14, the sum of broken is 5, and there are 2 broken groups after the first
        let actual = Problem::line_from_str(".??..??...?##.").unwrap();
        // this creates 8 possible arrangements that we would generate and want to test
        let broken_group = 1;
        let valid_arrange = (0..8)
            .map(|working_pad| build_spring_string(working_pad, broken_group))
            .filter(|suggest| valid_spring_arrange(&actual, &suggest))
            .count();
        assert_eq!(valid_arrange, 4);
    }
    #[test]
    fn test_valid_combinations_example_first() {
        let input = read_to_string("inputs/day-12.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        let spring_line = &problem.springs[0].line;
        let memo = &mut HashMap::new();
        assert_eq!(
            num_valid_spring_combinations_per_line(
                &spring_line.0,
                problem.springs[0].recorded_broken.as_slice(),
                memo
            ),
            1
        );
    }
    #[test]
    fn test_valid_combinations_example_per_line() {
        let input = read_to_string("inputs/day-12.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        let expected_combinations = vec![1, 4, 1, 1, 4, 10];
        let memo = &mut HashMap::new();

        problem.springs.iter().zip(expected_combinations).for_each(
            |(spring, expected_combinations)| {
                assert_eq!(
                    num_valid_spring_combinations_per_line(
                        &spring.line.0,
                        spring.recorded_broken.as_slice(),
                        memo
                    ),
                    expected_combinations
                );
            },
        );
    }
    #[test]
    fn test_spring_line_dbg() {
        let given_spring = ".#.#.#".to_owned();
        let spring_line = SpringLine(Problem::line_from_str(&given_spring).unwrap());
        assert_eq!(format!("{:?}", spring_line), given_spring);
    }
    #[test]
    fn test_broken_spring_matching() {
        // make sure that all broken strings are consumed according to clues
        let spring_line = Problem::line_from_str(".??..??...?##.").unwrap();
        let broken_groups = vec![2];
        let memo = &mut HashMap::new();
        // since only one broken group, there is 1 possible arrangement
        assert_eq!(
            num_valid_spring_combinations_per_line(
                spring_line.as_slice(),
                broken_groups.as_slice(),
                memo
            ),
            1
        );
    }
    #[test]
    fn test_clean_spring_test() {
        // make sure that we dont have an index error when we validate at the base case
        let spring_line = Problem::line_from_str("##.").unwrap();
        let broken_groups = vec![2];
        let memo = &mut HashMap::new();
        // since only one broken group, there is 1 possible arrangement
        // will create 2 possible arrangements but only one is valid
        // the first one since being valid '##.' try to index the spring line at 3.. which is out of bounds
        assert_eq!(
            num_valid_spring_combinations_per_line(
                spring_line.as_slice(),
                broken_groups.as_slice(),
                memo
            ),
            1
        );
    }
    #[test]
    fn test_part2() {
        let input = read_to_string("inputs/day-12.example.txt").unwrap();
        assert_eq!(part2(&input), 525152);
    }
    #[test]
    fn test_repetitions() {
        let spring_line = Problem::line_from_str("???.###").unwrap();
        let broken_groups = vec![1, 1, 3];
        let spring_arrangement = SpringArrangement {
            recorded_broken: broken_groups,
            line: SpringLine(spring_line),
        }
        .with_repetition(2);
        assert_eq!(spring_arrangement.line.0.len(), 15);
    }
}
