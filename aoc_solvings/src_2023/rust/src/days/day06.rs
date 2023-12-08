use std::{fs::read_to_string, str::FromStr};

use itertools::Itertools;

use crate::etc::{Solution, SolutionPair};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("inputs/day-06.txt")).unwrap();

    (Solution::from(part1(&input)), Solution::from(part2(&input)))
}

fn part1(input: &str) -> u64 {
    let problem = Problem::from_str(input).unwrap();
    problem.races.iter().map(ways_to_win_race).product()
}

/// Return number of ways that a toy boat can be operated to
/// beat the previous distance record.
fn ways_to_win_race(race: &Race) -> u64 {
    // don't bother calculating 0 and time charging
    // they yield 0 distance
    (1..race.time)
        .map(|time_charging| distance_traveled(time_charging, race.time))
        .filter(|calc_dist| calc_dist > &race.distance)
        .count() as u64
}

/// Return the distance traveled by a toy boat in `total_time` seconds, assuming
/// that it spent `time_charging` seconds increasing speed by 1 mm/s
/// without moving and `total_time - time_charging` seconds moving
/// at a speed of `time_charging` mm/s.
const fn distance_traveled(time_charging: u64, total_time: u64) -> u64 {
    (total_time - time_charging) * time_charging
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    races: Vec<Race>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Race {
    time: u64,
    distance: u64,
}

impl FromStr for Problem {
    type Err = peg::error::ParseError<peg::str::LineCol>;

    #[allow(clippy::ignored_unit_patterns)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        peg::parser! {
            grammar race_parser() for str {
                pub rule problem() -> Problem
                    = times:race_times() newline() distances:race_distances() newline()+ ![_] {
                        Problem { races: times.iter().zip(distances.iter()).map(|(time, dist)| Race { time: *time, distance: *dist }).collect()}
                    }

                rule race_times() -> Vec<u64>
                    = "Time:" __* times:(number() ++ __) { times }

                rule race_distances() -> Vec<u64>
                    = "Distance:" __* distances:(number() ++ __) { distances }

                rule number() -> u64
                    = n:$(['0'..='9']+) { n.parse().unwrap() }

                rule newline()
                    = ("\r"?"\n")+

                rule __()
                    = [' '| '\t'] +
            }
        }
        match race_parser::problem(s) {
            Ok(problem) => Ok(problem),
            Err(e) => Err(e),
        }
    }
}

fn part2(input: &str) -> u64 {
    let problem = Problem::from_str(input).unwrap();
    let rebuilt_race = build_single_race(&problem);

    ways_to_win_race(&rebuilt_race)
}

fn build_single_race(p: &Problem) -> Race {
    Race {
        time: p
            .races
            .iter()
            .map(|race| race.time.to_string())
            .join("")
            .parse::<u64>()
            .unwrap(),
        distance: p
            .races
            .iter()
            .map(|race| race.distance.to_string())
            .join("")
            .parse::<u64>()
            .unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_to_string("inputs/day-06.example.txt").unwrap();
        assert_eq!(part1(&input), 288);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("inputs/day-06.example.txt").unwrap();
        assert_eq!(part2(&input), 71503);
    }

    #[test]
    fn test_parse() {
        let input = read_to_string("inputs/day-06.example.txt").unwrap();
        let p = Problem::from_str(&input).unwrap();
        assert_eq!(p.races.len(), 3);
        assert_eq!(p.races[0].time, 7);
        assert_eq!(p.races[0].distance, 9);
    }
}
