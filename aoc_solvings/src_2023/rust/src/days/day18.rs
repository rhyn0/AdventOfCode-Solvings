use itertools::Itertools;

use crate::etc::{Solution, SolutionPair};
use std::{
    fs::read_to_string,
    ops::{Add, Div},
    str::FromStr,
};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("inputs/day-18.txt")).unwrap();

    (Solution::from(part1(&input)), Solution::from(part2(&input)))
}

#[allow(clippy::cast_sign_loss)]
fn part1(input: &str) -> usize {
    let instruction_list = parse_plain_dig_instructions(input);
    let mut current_position = Position { row: 0, col: 0 };
    let mut path: Vec<Position> = Vec::new();

    path.push(current_position);
    for instruction in instruction_list {
        let new_position = current_position.follow_instruction(instruction);
        path.push(new_position);
        current_position = new_position;
    }
    area_contained(&path) as usize
}
#[allow(dead_code)]
fn print_path(path: &[Position]) {
    let (
        Position {
            row: min_row,
            col: min_col,
        },
        Position {
            row: max_row,
            col: max_col,
        },
    ) = bounds_of_area(path);
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            let position = Position { row, col };
            if path.contains(&position) {
                eprint!("#");
            } else {
                eprint!(".");
            }
        }
        eprintln!();
    }
}

/// Return top left and bottom right `Position` representing the bounds of the area
#[allow(dead_code)]
fn bounds_of_area(path: &[Position]) -> (Position, Position) {
    let min_row = path.iter().map(|pos| pos.row).min().unwrap();
    let max_row = path.iter().map(|pos| pos.row).max().unwrap();
    let min_col = path.iter().map(|pos| pos.col).min().unwrap();
    let max_col = path.iter().map(|pos| pos.col).max().unwrap();
    (
        Position {
            row: min_row,
            col: min_col,
        },
        Position {
            row: max_row,
            col: max_col,
        },
    )
}
/// Return the number of squares contained in the area
///
/// More info: <https://en.wikipedia.org/wiki/Shoelace_formula>
fn area_contained(path: &Vec<Position>) -> isize {
    let mut local_path = path.clone();
    if path[0] != path[path.len() - 1] {
        local_path.push(path[0]);
    }
    let (area, perimeter) = local_path.iter().tuple_windows().fold(
        (0isize, 0isize),
        |(area, perimeter), (first, second)| {
            let area = area + (first.row * second.col - first.col * second.row);
            let perimeter = perimeter + first.manhattan_distance(*second);
            (area, perimeter)
        },
    );
    area.abs().add(perimeter).div(2).add(1)
}

#[derive(Debug, Clone, Copy)]
struct DigInstruction(Direction, isize);

impl FromStr for DigInstruction {
    type Err = String;

    /// Given a line of input, parse it into a `DigInstruction`.
    ///
    /// An example line is formatted as such: `R 8 (#111111)`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(' ');
        let direction = match parts.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err("Invalid direction".to_string()),
        };
        let distance = parts
            .next()
            .unwrap()
            .parse()
            .map_err(|_| "Unable to parse integer".to_string())?;

        Ok(Self(direction, distance))
    }
}

fn parse_plain_dig_instructions(input: &str) -> Vec<DigInstruction> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect::<Vec<_>>()
}

fn _hex_to_direction_distance(hex: &str) -> DigInstruction {
    let (_, _, color_string) = hex.split_whitespace().collect_tuple().unwrap();
    // hex part begins with a '(#' and ends with ')' so we skip it
    // hex string for color is 6 characters long
    let hex: String = color_string.chars().skip(2).take(6).collect();
    let distance_str = hex.chars().take(hex.len() - 1).collect::<String>();
    let direction_bit = hex.chars().last().unwrap().to_string().parse().unwrap();
    let direction = match direction_bit {
        0 => Direction::Right,
        1 => Direction::Down,
        2 => Direction::Left,
        3 => Direction::Up,
        _ => panic!("Invalid direction"),
    };
    let distance = isize::from_str_radix(&distance_str, 16).unwrap();

    DigInstruction(direction, distance)
}

fn parse_mixed_dig_instructions(input: &str) -> Vec<DigInstruction> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(_hex_to_direction_distance)
        .collect::<Vec<_>>()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    row: isize,
    col: isize,
}

impl Position {
    const fn follow_instruction(self, instruction: DigInstruction) -> Self {
        match instruction.0 {
            Direction::Up => Self {
                row: self.row - instruction.1,
                col: self.col,
            },
            Direction::Down => Self {
                row: self.row + instruction.1,
                col: self.col,
            },
            Direction::Left => Self {
                row: self.row,
                col: self.col - instruction.1,
            },
            Direction::Right => Self {
                row: self.row,
                col: self.col + instruction.1,
            },
        }
    }
    const fn manhattan_distance(self, other: Self) -> isize {
        (self.row - other.row).abs() + (self.col - other.col).abs()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[allow(clippy::cast_sign_loss)]
fn part2(input: &str) -> usize {
    let instruction_list = parse_mixed_dig_instructions(input);

    let mut current_position = Position { row: 0, col: 0 };
    let mut path: Vec<Position> = Vec::new();
    for instruction in instruction_list {
        let new_position = current_position.follow_instruction(instruction);
        path.push(new_position);
        current_position = new_position;
    }

    area_contained(&path) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_to_string("inputs/day-18.example.txt").unwrap();

        assert_eq!(part1(&input), 62);
    }
    #[test]
    fn test_part2() {
        let input = read_to_string("inputs/day-18.example.txt").unwrap();

        assert_eq!(part2(&input), 952408144115);
    }
}
