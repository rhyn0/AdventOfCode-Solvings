use crate::etc::{Solution, SolutionPair};
use std::cmp::max;
use std::fs::read_to_string;
use std::str::FromStr;
use strum_macros::EnumString;

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("./inputs/day-02.txt")).unwrap();
    let problem = Problem::from_str(&input).unwrap();
    (
        Solution::from(part01(&problem)),
        Solution::from(part02(&problem)),
    )
}

fn part01(input: &Problem) -> u32 {
    input
        .games
        .iter()
        .filter(|&game| {
            game.pulls.iter().all(|pull| {
                pull.red <= input.red && pull.green <= input.green && pull.blue <= input.blue
            })
        })
        .map(|game| game.id)
        .sum()
}

fn part02(input: &Problem) -> u32 {
    input
        .games
        .iter()
        .map(|game| {
            let (red, green, blue) = game.pulls.iter().fold((0, 0, 0), |acc, pull| {
                (
                    max(acc.0, pull.red),
                    max(acc.1, pull.green),
                    max(acc.2, pull.blue),
                )
            });
            cube_power(red, green, blue)
        })
        .sum()
}

const fn cube_power(red: u32, green: u32, blue: u32) -> u32 {
    red * green * blue
}

#[derive(Debug, Clone)]
pub struct Problem {
    blue: u32,
    green: u32,
    red: u32,
    games: Vec<Game>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    id: u32,
    pulls: Vec<CubePull>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CubePull {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString)]
enum CubeColor {
    #[strum(serialize = "blue")]
    Blue,
    #[strum(serialize = "green")]
    Green,
    #[strum(serialize = "red")]
    Red,
}

impl Default for Problem {
    fn default() -> Self {
        Self {
            red: 12,
            green: 13,
            blue: 14,
            games: Vec::new(),
        }
    }
}

impl FromStr for Problem {
    type Err = String;
    /**
     * Parse a string into a Problem.
     * A problem is made up of multiple games, which reside on separate lines.
     * A game is identified by a number before a colon
     * Each game has a set of pulls (varying number) which are semi colon separated
     * Each pull has a set of cubes (varying number) which are comma separated
     * Each pull is a set of cubes pulled and the number pulled with that color
     */
    #[allow(clippy::ignored_unit_patterns)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        peg::parser! {
            grammar problem_parser() for str {
                rule number() -> u32
                    = n:$(['0'..='9']+) { n.parse().unwrap() }

                rule whitespace()
                    = [' ' | '\t' | '\r']

                rule newline()
                    = ['\n' | '\r']+

                rule color() -> CubeColor
                    = c:$(['a'..='z']+) { CubeColor::from_str(c).unwrap() }

                rule cube() -> (u32, CubeColor)
                    = num:number() whitespace()+ col:color() { (num, col) }

                rule pull() -> CubePull
                    = r:cube() ++ ("," whitespace()) {
                        let mut red = 0;
                        let mut green = 0;
                        let mut blue = 0;
                        for (num, col) in r {
                            match col {
                                CubeColor::Red => red += num,
                                CubeColor::Green => green += num,
                                CubeColor::Blue => blue += num,
                            }
                        }
                        CubePull { red, green, blue }
                     }

                rule game(p: &mut Problem)
                    = "Game " n:number() ": " pulls:pull() ++ (";" whitespace()) { p.games.push(Game { id: n, pulls }) }

                pub rule problem(p: &mut Problem)
                    = (game(p) newline()*)+ ![_]
            }
        };
        let mut problem = Self::default();
        match problem_parser::problem(s, &mut problem) {
            Ok(()) => Ok(problem),
            Err(e) => Err(format!("Failed to parse problem: {e}")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_problem_fromstr() {
        let input = read_to_string("./inputs/day-02.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        // test that number of games is correct
        assert_eq!(problem.games.len(), 5);
        // test that number of pulls for each game is correct
        assert_eq!(problem.games[0].pulls.len(), 3);
        assert_eq!(problem.games[1].pulls.len(), 3);
        assert_eq!(problem.games[2].pulls.len(), 3);
        assert_eq!(problem.games[3].pulls.len(), 3);
        assert_eq!(problem.games[4].pulls.len(), 2);
    }

    #[test]
    fn test_part01() {
        let input = read_to_string("./inputs/day-02.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        assert_eq!(part01(&problem), 8_u32);
    }
    #[test]
    fn test_part02() {
        let input = read_to_string("./inputs/day-02.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        assert_eq!(part02(&problem), 2286_u32);
    }
    #[test]
    fn test_solve() {
        let (p1, p2) = solve(Some("./inputs/day-02.example.txt"));
        assert_eq!(p1, Solution::from(8_u32));
        assert_eq!(p2, Solution::from(2286_u32));
    }
}
