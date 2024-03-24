use itertools::Itertools;

use crate::etc::{Solution, SolutionPair};

use core::fmt;
use std::{char, collections::HashMap, fs::read_to_string, str::FromStr};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("./inputs/day-14.txt")).unwrap();
    (
        Solution::from(part01(&input)),
        Solution::from(part02(&input)),
    )
}

fn part01(input: &str) -> u32 {
    let problem = Problem::from_str(input).unwrap();
    // this only needs to move 'once' to get the answer
    let next_state = problem.grid.next();
    next_state.score_board()
}

#[derive(Debug, Clone)]
pub struct Problem {
    grid: StonePulleyGrid,
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_rows = s.lines().filter(|s| !s.is_empty()).count();
        let num_cols = {
            let first_line = s.lines().next().ok_or("Empty input")?;
            first_line.trim().len()
        };
        let spaces = s
            .lines()
            .filter(|s| !s.is_empty())
            .map(|l| {
                l.chars()
                    .map(|c| StoneType::try_from(c).unwrap())
                    .collect_vec()
            })
            .collect();

        Ok(Self {
            grid: StonePulleyGrid {
                num_rows,
                num_cols,
                stones: spaces,
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Position {
    fn north(&self) -> Result<Self, String> {
        match self.y {
            0 => Err("This is the northern most rows".to_owned()),
            _ => Ok(Self {
                x: self.x,
                y: self.y - 1,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StonePulleyGrid {
    num_rows: usize,
    num_cols: usize,
    stones: Vec<Vec<StoneType>>,
}

impl fmt::Display for StonePulleyGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.stones
                .iter()
                .map(|row| { row.iter().map(std::string::ToString::to_string).join("") })
                .join("\n")
        )
    }
}

impl StonePulleyGrid {
    /// Return the load score of the current board state
    ///
    /// Score is based on `StoneType::Round` pieces being farther from the bottom
    /// Each `StoneType::Round` gets 1 point for each row away it is from the "off board" last row
    /// The score total is the sum of points of all `StoneType::Round`
    ///
    /// E.g. A Round rock on row 0 in a 4x4 grid would be worth 4 points. Since it is on 0 (inclusive), and there are 3 rows of board between it and bottom edge
    fn score_board(&self) -> u32 {
        self.stones
            .iter()
            .enumerate()
            .flat_map(|(row, stone_row)| {
                stone_row
                    .iter()
                    .enumerate()
                    .map(move |(col, stone)| (stone, Position { x: col, y: row }))
            })
            .filter(|&(stone, _)| *stone == StoneType::Round)
            .fold(0, |acc, (_, position)| {
                acc + u32::try_from(self.num_rows - position.y).unwrap()
            })
    }
    /// Advance board state to the next state by moving pieces as far "up" (y-zero) as possible
    fn next(self) -> Self {
        let mut board = self.stones.clone();
        for row_offset in 1..self.num_rows {
            // offset means how many rows to skip from the bottom
            for row_idx in 1..=(self.num_rows - row_offset) {
                // for each row, check each stone
                for col in 0..self.num_cols {
                    // look in the row below the current row, to see if we can move a stone from next to here
                    let stone = &board[row_idx][col];
                    // if the stone is a round stone, check if it can move north
                    if stone == &StoneType::Round {
                        let position = Position { x: col, y: row_idx };
                        let new_position = position.north();
                        if let Ok(new_position) = new_position {
                            // if it can move north, check if the new position is empty
                            if board[new_position.y][new_position.x] == StoneType::Empty {
                                // if it is empty, move the stone to the new position
                                board[new_position.y][new_position.x] = StoneType::Round;
                                board[position.y][position.x] = StoneType::Empty;
                            }
                        }
                    }
                }
            }
        }
        Self {
            stones: board,
            ..self
        }
    }
    /// Rotate the grid 90 degrees clockwise
    /// The previous left side becomes the top of the grid
    fn rot_90_cw(self) -> Self {
        let mut new_stones = vec![vec![StoneType::Empty; self.num_rows]; self.num_cols];
        for (row_idx, row) in self.stones.iter().enumerate() {
            for (col_idx, stone) in row.iter().enumerate() {
                let new_row_idx = col_idx;
                let new_col_idx = self.num_rows - row_idx - 1;
                new_stones[new_row_idx][new_col_idx] = stone.clone();
            }
        }
        Self {
            stones: new_stones,
            ..self
        }
    }
    fn hash(&self) -> String {
        self.stones
            .iter()
            .enumerate()
            .flat_map(|(row, stone_row)| {
                stone_row.iter().enumerate().map(move |(col, stone)| {
                    (
                        stone,
                        Position {
                            x: col,
                            y: self.num_rows - row,
                        },
                    )
                })
            })
            .filter(|&(stone, _)| *stone == StoneType::Round)
            .map(|(_, position)| position.to_string())
            .join("|")
    }
    /// Move the board to the next position after a set of 4 tilt and 90 degree CW rotation
    /// Repeated `num_cycles` of times
    fn cycle(self, num_cycles: usize) -> Self {
        let num_directions_per_cycle = 4;

        (0..num_cycles).fold(self, |acc, _| {
            (0..num_directions_per_cycle).fold(acc, |sub_acc, _| sub_acc.next().rot_90_cw())
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum StoneType {
    Empty,
    Fixed,
    Round,
}

impl fmt::Display for StoneType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Fixed => write!(f, "#"),
            Self::Round => write!(f, "O"),
        }
    }
}

impl FromStr for StoneType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Empty),
            "#" => Ok(Self::Fixed),
            "O" => Ok(Self::Round),
            _ => Err(format!("Invalid stone space: {s}")),
        }
    }
}

impl TryFrom<char> for StoneType {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Self::from_str(&value.to_string())
    }
}

fn part02(input: &str) -> u32 {
    let problem = Problem::from_str(input).unwrap();
    let total_cycles: i64 = 1_000_000_000;

    // in the simplest case, we could have a 2x2 grid with 1 round stone
    // for each cycle the stone will move in a counter clockwise direction around the grid
    // then return to the same spot each turn
    // to skip these cycles being simulated, we can keep a history of the board state
    let mut history: HashMap<String, i64> = HashMap::new();
    let mut cycle_found = false;
    // start at one to properly calculate the cycle length
    // the calculation takes place after each cycle, so we can do this or
    // litter +1 in the loop
    let mut current_cycle = 1;
    let mut board = problem.grid;
    while current_cycle < total_cycles {
        board = board.cycle(1);

        if !cycle_found {
            let hash = board.hash();
            if let Some(position) = history.get(&hash) {
                let cycle_length = current_cycle - position - 1;
                let remaining_cycles = total_cycles - current_cycle;
                let cycles_to_skip = remaining_cycles / cycle_length;

                current_cycle += cycles_to_skip * cycle_length;
                cycle_found = true;
                continue;
            }
            history.insert(hash, current_cycle - 1);
        }
        current_cycle += 1;
    }

    board.score_board()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use indoc::indoc;

    use super::*;

    #[test]
    fn test_part1() {
        let input = read_to_string("./inputs/day-14.example.txt").unwrap();
        assert_eq!(part01(&input), 136);
    }

    #[test]
    fn test_problem_parse() {
        let input = read_to_string("./inputs/day-14.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();

        assert_eq!(problem.grid.num_rows, 10);
        assert_eq!(problem.grid.num_cols, 10);
        assert_eq!(
            problem
                .grid
                .stones
                .get(0)
                .and_then(|row| row.get(0))
                .unwrap()
                .to_owned(),
            StoneType::Round
        );
    }

    #[test]
    fn test_rock_move_fail_north() {
        // make sure Err is returned when rocks can't move anymore
        let input = indoc::indoc! {"
            O.........
            "};
        let problem = Problem::from_str(&input).unwrap();
        let new_board = problem.grid.clone().next();
        assert_eq!(
            problem
                .grid
                .stones
                .get(0)
                .and_then(|row| row.get(0))
                .unwrap()
                .to_owned(),
            StoneType::Round
        );
        assert_eq!(new_board.stones, problem.grid.stones);
        assert_eq!(problem.grid.stones.len(), 1);
    }
    #[test]
    fn test_rock_move_fail_east() {
        let input = indoc::indoc! {"
            .........O
            "};
        let problem = Problem::from_str(&input).unwrap();
        let new_board = problem.grid.clone().next();
        assert_eq!(
            problem
                .grid
                .stones
                .get(0)
                .and_then(|row| row.get(9))
                .unwrap()
                .to_owned(),
            StoneType::Round
        );
        assert_eq!(new_board.stones, problem.grid.stones);
    }
    #[test]
    fn test_position_to_string() {
        assert_eq!(Position { x: 0, y: 0 }.to_string(), "0,0".to_owned());
    }
    #[test]
    fn test_part2() {
        let input = read_to_string("./inputs/day-14.example.txt").unwrap();
        assert_eq!(part02(&input), 64);
    }
    #[test]
    fn test_grid_rotate() {
        let input = indoc::indoc! {"
            O..
            .O.
            ...
            "};
        let problem = Problem::from_str(&input).unwrap();
        let new_board = problem.grid.rot_90_cw();
        assert_eq!(
            new_board.stones,
            vec![
                vec![StoneType::Empty, StoneType::Empty, StoneType::Round],
                vec![StoneType::Empty, StoneType::Round, StoneType::Empty],
                vec![StoneType::Empty, StoneType::Empty, StoneType::Empty]
            ]
        );
    }
    #[test]
    fn test_one_tilt() {
        let problem = Problem::from_str(
            read_to_string("./inputs/day-14.example.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();
        let expected_state = Problem::from_str(indoc! {"
            OOOO.#.O..
            OO..#....#
            OO..O##..O
            O..#.OO...
            ........#.
            ..#....#.#
            ..O..#.O.O
            ..O.......
            #....###..
            #....#....
            "})
        .unwrap();
        let post_tilt = problem.grid.clone().next();
        assert_ne!(post_tilt, problem.grid);
        assert_eq!(post_tilt, expected_state.grid);
    }
    #[test]
    fn test_quarter_cycle() {
        let problem = Problem::from_str(
            read_to_string("./inputs/day-14.example.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();
        // hand rotated
        let expected_state = Problem::from_str(indoc! {"
            ##....OOOO
            .......OOO
            ..OO#....O
            ......#..O
            .......O#.
            ##.#..O#.#
            .#....O#..
            .#.O#....O
            .....#....
            ...O#..O#.
            "})
        .unwrap();
        let post_tilt = problem.grid.clone().next().rot_90_cw();
        assert_ne!(post_tilt, problem.grid);
        assert_eq!(post_tilt, expected_state.grid);
    }
    #[test]
    fn test_given_cycle() {
        let problem = Problem::from_str(
            read_to_string("./inputs/day-14.example.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();
        let expected_after_cycle = Problem::from_str(indoc! {"
            .....#....
            ....#...O#
            ...OO##...
            .OO#......
            .....OOO#.
            .O#...O#.#
            ....O#....
            ......OOOO
            #...O###..
            #..OO#....
            "})
        .unwrap();

        let actual_board = problem.grid.cycle(1);
        assert_eq!(actual_board, expected_after_cycle.grid);
    }
    #[test]
    fn test_given_second_cycle() {
        let problem = Problem::from_str(
            read_to_string("./inputs/day-14.example.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();
        let expected_after_cycle = Problem::from_str(indoc! {"
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #..OO###..
            #.OOO#...O
            "})
        .unwrap();

        let actual_board = problem.grid.cycle(2);
        assert_eq!(actual_board, expected_after_cycle.grid);
    }
    #[test]
    fn test_given_third_cycle() {
        let problem = Problem::from_str(
            read_to_string("./inputs/day-14.example.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();
        let expected_after_cycle = Problem::from_str(indoc! {"
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #...O###.O
            #.OOO#...O
            "})
        .unwrap();

        let actual_board = problem.grid.cycle(3);
        assert_eq!(actual_board, expected_after_cycle.grid);
    }
}
