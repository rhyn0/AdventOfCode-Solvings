use crate::etc::{Solution, SolutionPair};

use std::{
    borrow::BorrowMut,
    collections::{HashSet, VecDeque},
    fmt::Display,
    fs::read_to_string,
    hash::Hash,
    str::FromStr,
};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("inputs/day-16.txt")).unwrap();
    (Solution::from(part1(&input)), Solution::from(part2(&input)))
}

fn part1(input: &str) -> u32 {
    let mut problem = Problem::from_str(input).unwrap();
    // visited is a set of (position, direction) pairs
    // we only need to know unique positions, so we need to convert the set to a set of positions
    u32::try_from(problem.energize_grid()).unwrap()
}

#[derive(Debug, Clone)]
struct Problem {
    grid: Grid,
    light_beams: Vec<LightBeam>,
    visited: HashSet<(Position, Direction)>,
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_str(s)?;

        Ok(Self {
            grid,
            ..Default::default()
        })
    }
}

impl Default for Problem {
    fn default() -> Self {
        Self {
            grid: Grid::default(),
            light_beams: vec![LightBeam {
                position: Position { row: 0, col: 0 },
                direction: Direction::Right,
            }],
            visited: HashSet::new(),
        }
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        let visited_pos = self
            .visited
            .iter()
            .map(|(pos, _)| pos)
            .collect::<HashSet<_>>();
        for row in 0..self.grid.num_rows {
            for (pos, space) in self.grid.iter_positions(row) {
                if visited_pos.contains(&pos) {
                    result.push('#');
                } else {
                    result.push_str(&format!("{space}"));
                }
            }
            result.push('\n');
        }
        write!(f, "{result}")
    }
}

impl Problem {
    fn energize_grid(&mut self) -> usize {
        // need to modify the light beams to return the squares that they interact with
        // or steal it from their new positions.
        let visited = self.visited.borrow_mut();

        let mut light_beam_queue: VecDeque<LightBeam> = self.light_beams.iter().cloned().collect();

        while let Some(lb) = light_beam_queue.pop_front() {
            if visited.contains(&(lb.position, lb.direction)) {
                continue;
            }
            if let Some(space) = self.grid.get(lb.position) {
                visited.insert((lb.position, lb.direction));
                // all spaces need to have their bool value set to true
                let next_directions = lb.next_direction(*space);
                for &dir in &next_directions {
                    let new_lb = LightBeam {
                        position: lb.position,
                        direction: dir,
                    };
                    if let Some(position) = new_lb.next_position() {
                        light_beam_queue.push_back(LightBeam { position, ..new_lb });
                    }
                }
            }
        }
        self.visited
            .iter()
            .map(|(pos, _)| pos)
            .collect::<HashSet<_>>()
            .len()
    }
}

#[derive(Debug, Clone, Default)]
struct Grid {
    data: Vec<Vec<GridSpace>>,
    num_rows: usize,
    num_cols: usize,
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .filter(|s| !s.is_empty())
            .map(|line| {
                line.chars()
                    .map(GridSpace::from_char)
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        let num_rows = data.len();
        let num_cols = data[0].len();

        Ok(Self {
            data,
            num_rows,
            num_cols,
        })
    }
}

impl Grid {
    fn get(&self, position: Position) -> Option<&GridSpace> {
        self.data
            .get(position.row)
            .and_then(|row| row.get(position.col))
    }
    /// Iterate over all the positions in row-column order. Yielding the position and the `GridSpace` at that spot
    fn iter_positions(&self, row: usize) -> impl Iterator<Item = (Position, GridSpace)> + '_ {
        self.data
            .get(row)
            .unwrap()
            .iter()
            .enumerate()
            .map(move |(col, space)| (Position { row, col }, *space))
    }
    fn positions_inwards(&self) -> Vec<(Position, Direction)> {
        let mut result = Vec::with_capacity(2 * (self.num_rows + self.num_cols));
        for row in 0..self.num_rows {
            result.push((Position { row, col: 0 }, Direction::Right));
            result.push((
                Position {
                    row,
                    col: self.num_cols - 1,
                },
                Direction::Left,
            ));
        }
        for col in 0..self.num_cols {
            result.push((Position { row: 0, col }, Direction::Down));
            result.push((
                Position {
                    row: self.num_rows - 1,
                    col,
                },
                Direction::Up,
            ));
        }
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    const fn up(&self) -> Option<Self> {
        if self.row > 0 {
            Some(Self {
                row: self.row - 1,
                col: self.col,
            })
        } else {
            None
        }
    }
    const fn down(&self) -> Self {
        Self {
            row: self.row + 1,
            col: self.col,
        }
    }
    const fn right(&self) -> Self {
        Self {
            row: self.row,
            col: self.col + 1,
        }
    }
    const fn left(&self) -> Option<Self> {
        if self.col > 0 {
            Some(Self {
                row: self.row,
                col: self.col - 1,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GridSpace {
    Empty(bool),
    LeftMirror(bool),      // a /
    RightMirror(bool),     // a \
    HorizontalSplit(bool), // a -
    VerticalSplit(bool),   // a |
}

impl Display for GridSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty(_) => write!(f, "."),
            Self::LeftMirror(_) => write!(f, "/"),
            Self::RightMirror(_) => write!(f, "\\"),
            Self::HorizontalSplit(_) => write!(f, "-"),
            Self::VerticalSplit(_) => write!(f, "|"),
        }
    }
}

impl GridSpace {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            '.' => Ok(Self::Empty(false)),
            '/' => Ok(Self::LeftMirror(false)),
            '\\' => Ok(Self::RightMirror(false)),
            '-' => Ok(Self::HorizontalSplit(false)),
            '|' => Ok(Self::VerticalSplit(false)),
            _ => Err(format!("Invalid grid space character: {c}")),
        }
    }
}

impl FromStr for GridSpace {
    type Err = String;

    /// Only consumes first character of given string to build the space.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_char(s.chars().next().unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LightBeam {
    position: Position,
    direction: Direction,
}

impl Default for LightBeam {
    fn default() -> Self {
        Self {
            position: Position { row: 0, col: 0 },
            direction: Direction::Right,
        }
    }
}

impl LightBeam {
    fn next_direction(&self, space: GridSpace) -> Vec<Direction> {
        match space {
            GridSpace::Empty(_) => vec![self.direction],
            GridSpace::LeftMirror(_) => match self.direction {
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down],
                Direction::Right => vec![Direction::Up],
            },
            GridSpace::RightMirror(_) => match self.direction {
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up],
                Direction::Right => vec![Direction::Down],
            },
            GridSpace::HorizontalSplit(_) => match self.direction {
                Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left | Direction::Right => vec![self.direction],
            },
            GridSpace::VerticalSplit(_) => match self.direction {
                Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
                Direction::Up | Direction::Down => vec![self.direction],
            },
        }
    }
    const fn next_position(&self) -> Option<Position> {
        match self.direction {
            Direction::Up => self.position.up(),
            Direction::Down => Some(self.position.down()),
            Direction::Left => self.position.left(),
            Direction::Right => Some(self.position.right()),
        }
    }
}

fn part2(input: &str) -> u32 {
    // light beam start point is configurable
    // has to be on an edge location, and pointing inwards
    let start_problem = Problem::from_str(input).unwrap();

    let start_positions = start_problem.grid.positions_inwards();

    let max_energize = start_positions
        .iter()
        .map(|(pos, dir)| {
            let mut problem = Problem {
                grid: start_problem.grid.clone(),
                light_beams: vec![LightBeam {
                    position: *pos,
                    direction: *dir,
                }],
                ..Default::default()
            };
            problem.energize_grid()
        })
        .max()
        .unwrap();
    u32::try_from(max_energize).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_to_string("inputs/day-16.example.txt").unwrap();
        assert_eq!(part1(&input), 46);
    }
    #[test]
    fn test_grid_parse() {
        let input = read_to_string("inputs/day-16.example.txt").unwrap();
        let grid = Grid::from_str(&input).unwrap();

        assert_eq!(grid.num_rows, 10);
        assert_eq!(grid.num_cols, 10);
        assert_eq!(
            grid.data[0],
            vec![
                GridSpace::Empty(false),
                GridSpace::VerticalSplit(false),
                GridSpace::Empty(false),
                GridSpace::Empty(false),
                GridSpace::Empty(false),
                GridSpace::RightMirror(false),
                GridSpace::Empty(false),
                GridSpace::Empty(false),
                GridSpace::Empty(false),
                GridSpace::Empty(false),
            ]
        );
    }
    #[test]
    fn test_problem_parse() {
        let input = read_to_string("inputs/day-16.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();

        assert_eq!(problem.grid.num_rows, 10);
        assert_eq!(problem.grid.num_cols, 10);
        assert_eq!(problem.light_beams.len(), 1);
        assert_eq!(problem.light_beams[0].position.row, 0);
        assert_eq!(problem.light_beams[0].position.col, 0);
        assert_eq!(problem.light_beams[0].direction, Direction::Right);

        assert_eq!(
            problem.grid.data[0],
            vec![
                GridSpace::Empty(false),
                GridSpace::VerticalSplit(false),
                GridSpace::Empty(false),
                GridSpace::Empty(false),
                GridSpace::Empty(false),
                GridSpace::RightMirror(false),
                GridSpace::Empty(false),
                GridSpace::Empty(false),
                GridSpace::Empty(false),
                GridSpace::Empty(false),
            ]
        );
    }
    #[test]
    fn test_part2() {
        let input = read_to_string("inputs/day-16.example.txt").unwrap();

        assert_eq!(part2(&input), 51);
    }
}
