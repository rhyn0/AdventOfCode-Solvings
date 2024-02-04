use crate::etc::{Solution, SolutionPair};

use std::{collections::HashSet, fs::read_to_string, str::FromStr};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("inputs/day-21.txt")).unwrap();

    (Solution::from(part1(&input)), Solution::from(part2(&input)))
}

fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    run_simulation(&grid, 64)
}

/// Simulate letting a person go through one step Brownian motion for a given number of steps
/// Person can move in any of the 4 cardinal directions
/// But can not step on `GardenTiles::Rock`
///
/// Return the number of spaces that can be occupied after `num_moves`
fn run_simulation(grid: &Grid, num_moves: usize) -> usize {
    let mut occupied_spaces: HashSet<Position> = HashSet::new();
    occupied_spaces.insert(grid.starting_position);
    for _ in 0..num_moves {
        let new_occupied_spaces: HashSet<Position> = occupied_spaces
            .iter()
            .flat_map(|&pos| pos.cardinal_neighbors())
            .filter(|&pos| {
                grid.get(pos)
                    .map_or(false, |tile| tile != GardenTiles::Rock)
            })
            .collect();
        occupied_spaces = new_occupied_spaces;
    }
    occupied_spaces.len()
}

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<Vec<GardenTiles>>,
    starting_position: Position,
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(GardenTiles::from_char)
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>();
        match cells {
            Err(e) => Err(e),
            Ok(mut cells) => {
                // starting position needs to be known, and then transform into a normal plot
                let start_position = cells
                    .iter()
                    .enumerate()
                    .find(|&(_, row)| row.contains(&GardenTiles::Starting))
                    .map(|(row_idx, row)| {
                        (
                            row_idx,
                            row.iter()
                                .position(|&tile| tile == GardenTiles::Starting)
                                .unwrap(),
                        )
                    })
                    .unwrap();
                cells[start_position.0][start_position.1] = GardenTiles::Plot;
                Ok(Self {
                    cells,
                    starting_position: Position::new(start_position.0, start_position.1),
                })
            }
        }
    }
}

impl Grid {
    fn get(&self, pos: Position) -> Option<GardenTiles> {
        self.cells.get(pos.row)?.get(pos.col).copied()
    }
    fn num_rows(&self) -> usize {
        self.cells.len()
    }
    fn num_cols(&self) -> usize {
        self.cells.get(0).map_or(0, std::vec::Vec::len)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GardenTiles {
    Plot,
    Rock,
    Starting,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    const fn north(&self) -> Option<Self> {
        if self.row == 0 {
            None
        } else {
            Some(Self::new(self.row - 1, self.col))
        }
    }
    const fn south(&self) -> Self {
        Self::new(self.row + 1, self.col)
    }
    const fn east(&self) -> Self {
        Self::new(self.row, self.col + 1)
    }
    const fn west(&self) -> Option<Self> {
        if self.col == 0 {
            None
        } else {
            Some(Self::new(self.row, self.col - 1))
        }
    }
    fn cardinal_neighbors(&self) -> Vec<Self> {
        [
            self.north(),
            Some(self.south()),
            Some(self.east()),
            self.west(),
        ]
        .iter()
        .filter_map(|x| *x)
        .collect()
    }
}

impl GardenTiles {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            '.' => Ok(Self::Plot),
            'S' => Ok(Self::Starting),
            '#' => Ok(Self::Rock),
            _ => Err(format!("Invalid garden tile: {c}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SignedPosition {
    row: isize,
    col: isize,
}
impl SignedPosition {
    const fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
    fn cardinal_neighbors(&self) -> Vec<Self> {
        vec![
            Self::new(self.row - 1, self.col),
            Self::new(self.row + 1, self.col),
            Self::new(self.row, self.col - 1),
            Self::new(self.row, self.col + 1),
        ]
    }
}

#[derive(Debug, Clone)]
struct RepeatingGrid {
    grid: Grid,
}

impl RepeatingGrid {
    fn get(&self, pos: SignedPosition) -> GardenTiles {
        // let row = (pos.row % self.grid.num_rows() as isize) as usize;
        // let col = (pos.col % self.grid.num_cols() as isize) as usize;
        self.grid.get(self.convert_to_unsigned(pos)).unwrap()
    }
    fn convert_to_unsigned(&self, pos: SignedPosition) -> Position {
        let row = pos
            .row
            .rem_euclid(isize::try_from(self.grid.num_rows()).unwrap()) as usize;
        let col = pos
            .col
            .rem_euclid(isize::try_from(self.grid.num_cols()).unwrap()) as usize;
        Position::new(row, col)
    }
}

fn run_infinite_simulation(grid: &Grid, num_moves: usize) -> usize {
    let mut occupied_spaces: HashSet<SignedPosition> = HashSet::new();
    let repeated_grid = RepeatingGrid { grid: grid.clone() };
    occupied_spaces.insert(SignedPosition::new(
        isize::try_from(grid.starting_position.row).unwrap(),
        isize::try_from(grid.starting_position.col).unwrap(),
    ));
    for _ in 0..num_moves {
        let new_occupied_spaces: HashSet<_> = occupied_spaces
            .iter()
            .flat_map(|&pos| pos.cardinal_neighbors())
            .filter(|&pos| repeated_grid.get(pos) != GardenTiles::Rock)
            .collect();
        occupied_spaces = new_occupied_spaces;
    }
    occupied_spaces.len()
}

fn part2(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    let num_steps = 26_501_365usize;
    // 26501365 = 202300 * 131 + 65 where 131 is the dimension of the grid
    let base_steps = 65;
    // Throw some math at it since its a quadratic relation
    let n = num_steps / grid.num_rows();
    // this is a quadratic relation, so we need to solve a system of equations
    let a0 = run_infinite_simulation(&grid, base_steps);
    let a1 = run_infinite_simulation(&grid, base_steps + grid.num_rows());
    let a2 = run_infinite_simulation(&grid, base_steps + 2 * grid.num_rows());
    // transform the answers into the values of the quadratic relation
    let b0 = a0;
    let b1 = a1 - a0;
    let b2 = a2 - a1;
    b0 + b1 * n + (n * (n - 1)).div_euclid(2) * (b2 - b1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_to_string("inputs/day-21.example.txt").unwrap();

        let grid = Grid::from_str(&input).unwrap();

        assert_eq!(run_simulation(&grid, 1), 2);
        assert_eq!(run_simulation(&grid, 6), 16);
    }
    #[test]
    fn test_grid_input() {
        let input = read_to_string("inputs/day-21.txt").unwrap();
        let grid = Grid::from_str(&input).unwrap();
        assert_eq!(grid.num_rows(), 131);
    }
    #[test]
    fn test_infinite_grid() {
        let input = read_to_string("inputs/day-21.example.txt").unwrap();
        let grid = Grid::from_str(&input).unwrap();
        assert_eq!(run_infinite_simulation(&grid, 6), 16);
        assert_eq!(run_infinite_simulation(&grid, 10), 50);
        assert_eq!(run_infinite_simulation(&grid, 50), 1594);
        assert_eq!(run_infinite_simulation(&grid, 100), 6536);
        assert_eq!(run_infinite_simulation(&grid, 500), 167004);
    }
}
