use crate::etc::{Solution, SolutionPair};
use std::{collections::HashSet, fs::read_to_string, ops::RangeInclusive, str::FromStr};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("inputs/day-11.txt")).unwrap();
    (
        Solution::from(part1(&input)),
        Solution::from(part2(&input, None)),
    )
}

fn part1(input: &str) -> u64 {
    let problem = Problem::from_str(input).unwrap();
    // get the sum of all pathways between all galaxies
    generate_pairs(&problem.galaxies)
        .iter()
        .map(|(a, b)| {
            manhattan_distance_with_expansion(
                &a.position,
                &b.position,
                &problem.expanded_rows,
                &problem.expanded_cols,
                1,
            )
        })
        .sum()
}

fn generate_pairs(galaxies: &[Galaxy]) -> Vec<(Galaxy, Galaxy)> {
    let mut pairs = Vec::new();
    for (i, galaxy_a) in galaxies.iter().enumerate() {
        for galaxy_b in &galaxies[i + 1..] {
            pairs.push((galaxy_a.clone(), galaxy_b.clone()));
        }
    }
    pairs
}

fn manhattan_distance_with_expansion(
    a: &Position,
    b: &Position,
    expanded_rows: &HashSet<u64>,
    expanded_cols: &HashSet<u64>,
    space_expansion_modifier: u64,
) -> u64 {
    row_major_manhattan_path(*a..=*b)
        .skip(1) // skip the starting position
        .map(|pos| {
            let mut dist = 1;
            // if we cross a row or column that has been expanded, we add the modifier - 1 to the distance
            // possible to cross an intersection of expanded row column which means we need to add twice
            if expanded_rows.contains(&pos.y) {
                dist += space_expansion_modifier - 1;
            }
            if expanded_cols.contains(&pos.x) {
                dist += space_expansion_modifier - 1;
            }
            dist
        })
        .sum()
}

/// Return the row ordered path from `start` to `end` (Inclusive).
/// Prioritizes doing the x axis first, then the y axis.
fn row_major_manhattan_path(range: RangeInclusive<Position>) -> impl Iterator<Item = Position> {
    let row_range = range.clone();
    let row_start = range.start().x.min(range.end().x);
    let row_end = range.start().x.max(range.end().x);
    let col_start = range.start().y.min(range.end().y);
    let col_end = range.start().y.max(range.end().y);
    let row_iter = (row_start..=row_end).map(move |x| Position {
        x,
        y: row_range.start().y,
    });
    let col_iter = ((col_start + 1)..=col_end).map(move |y| Position {
        x: range.end().x,
        y,
    });
    row_iter.chain(col_iter)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: u64,
    y: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Galaxy {
    position: Position,
    id: u64,
}

#[derive(Debug, Clone)]
pub struct Problem {
    galaxies: Vec<Galaxy>,
    expanded_cols: HashSet<u64>,
    expanded_rows: HashSet<u64>,
}

impl FromStr for Problem {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxies = Vec::new();
        let lines = s.lines().filter(|line| !line.is_empty());

        let mut original_width = 0;
        let mut original_height = 0;
        let mut galaxy_id = 1;
        lines.enumerate().for_each(|(y, line)| {
            original_height += 1;
            original_width = line.len() as u64;
            line.chars().enumerate().for_each(|(x, c)| match c {
                '#' => {
                    galaxies.push(Galaxy {
                        position: Position {
                            x: x as u64,
                            y: y as u64,
                        },
                        id: galaxy_id,
                    });
                    galaxy_id += 1;
                }
                '.' => {}
                _ => panic!("Invalid character"),
            });
        });
        let mut expanded_cols = (0..original_width).collect::<HashSet<_>>();
        let mut expanded_rows = (0..original_height).collect::<HashSet<_>>();
        for galaxy in &galaxies {
            expanded_cols.remove(&galaxy.position.x);
            expanded_rows.remove(&galaxy.position.y);
        }
        Ok(Self {
            galaxies,
            expanded_cols,
            expanded_rows,
        })
    }
}

fn part2(input: &str, space_expansion_mod: Option<u64>) -> u64 {
    let problem = Problem::from_str(input).unwrap();
    generate_pairs(&problem.galaxies)
        .iter()
        .map(|(a, b)| {
            manhattan_distance_with_expansion(
                &a.position,
                &b.position,
                &problem.expanded_rows,
                &problem.expanded_cols,
                space_expansion_mod.unwrap_or(1_000_000),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_to_string("inputs/day-11.example.txt").unwrap();
        assert_eq!(part1(&input), 374);
    }
    #[test]
    fn test_problem_parse() {
        let input = read_to_string("inputs/day-11.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        assert!(problem.expanded_cols.len() > 0);
        assert!(problem.expanded_rows.len() > 0);
        assert_eq!(problem.galaxies.len(), 9);
    }
    #[test]
    fn test_path_length() {
        let input = read_to_string("inputs/day-11.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        let galaxy_5 = problem.galaxies.iter().find(|g| g.id == 5).unwrap();
        let galaxy_9 = problem.galaxies.iter().find(|g| g.id == 9).unwrap();
        assert_eq!(
            manhattan_distance_with_expansion(
                &galaxy_5.position,
                &galaxy_9.position,
                &problem.expanded_rows,
                &problem.expanded_cols,
                1
            ),
            9
        )
    }
    #[test]
    fn test_path_length_1_7() {
        let input = read_to_string("inputs/day-11.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        let galaxy_1 = problem.galaxies.iter().find(|g| g.id == 1).unwrap();
        let galaxy_7 = problem.galaxies.iter().find(|g| g.id == 7).unwrap();
        assert_eq!(
            manhattan_distance_with_expansion(
                &galaxy_1.position,
                &galaxy_7.position,
                &problem.expanded_rows,
                &problem.expanded_cols,
                1
            ),
            15
        )
    }
    #[test]
    fn test_path_length_3_6() {
        let input = read_to_string("inputs/day-11.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        let galaxy_3 = problem.galaxies.iter().find(|g| g.id == 3).unwrap();
        let galaxy_6 = problem.galaxies.iter().find(|g| g.id == 6).unwrap();
        assert_eq!(
            manhattan_distance_with_expansion(
                &galaxy_3.position,
                &galaxy_6.position,
                &problem.expanded_rows,
                &problem.expanded_cols,
                1
            ),
            17
        )
    }
    #[test]
    fn test_path_length_8_9() {
        let input = read_to_string("inputs/day-11.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        let galaxy_8 = problem.galaxies.iter().find(|g| g.id == 8).unwrap();
        let galaxy_9 = problem.galaxies.iter().find(|g| g.id == 9).unwrap();
        assert_eq!(
            manhattan_distance_with_expansion(
                &galaxy_8.position,
                &galaxy_9.position,
                &problem.expanded_rows,
                &problem.expanded_cols,
                1
            ),
            5
        )
    }
    #[test]
    fn test_path_length_1_3() {
        let input = read_to_string("inputs/day-11.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        let galaxy_1 = problem.galaxies.iter().find(|g| g.id == 1).unwrap();
        let galaxy_3 = problem.galaxies.iter().find(|g| g.id == 3).unwrap();
        assert_eq!(
            manhattan_distance_with_expansion(
                &galaxy_1.position,
                &galaxy_3.position,
                &problem.expanded_rows,
                &problem.expanded_cols,
                1
            ),
            6
        )
    }
    #[test]
    fn test_row_major() {
        let result = row_major_manhattan_path(Position { x: 0, y: 0 }..=Position { x: 2, y: 2 })
            .collect::<Vec<_>>();
        assert_eq!(
            result,
            vec![
                Position { x: 0, y: 0 },
                Position { x: 1, y: 0 },
                Position { x: 2, y: 0 },
                Position { x: 2, y: 1 },
                Position { x: 2, y: 2 },
            ]
        )
    }
    #[test]
    fn test_manhattan_creation() {
        // we expect there to be 36 pairs for the example
        let input = read_to_string("inputs/day-11.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        let pairs = generate_pairs(&problem.galaxies);
        assert_eq!(pairs.len(), 36);
    }
    #[test]
    fn test_part2() {
        let input = read_to_string("inputs/day-11.example.txt").unwrap();
        assert_eq!(part2(&input, Some(10)), 1030);
    }
}
