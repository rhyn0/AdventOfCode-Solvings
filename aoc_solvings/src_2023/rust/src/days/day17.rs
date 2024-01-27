use itertools::Itertools;

use crate::etc::{Solution, SolutionPair};
use std::{
    collections::{BinaryHeap, HashMap},
    fmt::{Debug, Display},
    fs::read_to_string,
    hash::Hash,
    str::FromStr,
};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("inputs/day-17.txt")).unwrap();
    (Solution::from(part1(&input)), Solution::from(part2(&input)))
}

fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    // start in top left
    let start = Position { row: 0, col: 0 };
    let starts = vec![
        ZigZagVector::new(Vector::new(start, Direction::Right)),
        ZigZagVector::new(Vector::new(start, Direction::Down)),
    ];

    let cost_fn = |zzvec: ZigZagVector| -> usize { grid.get(&zzvec.position()).unwrap() as usize };
    let is_end_fn = |zzvec: ZigZagVector| -> bool {
        zzvec.position()
            == (Position {
                row: grid.num_rows - 1,
                col: grid.num_cols - 1,
            })
    };
    let adjacency_fn = |zzvec: ZigZagVector| -> Vec<ZigZagVector> {
        let position_option_vec = if zzvec.consecutive_straight < 3 {
            vec![
                zzvec.straight().ok(),
                zzvec.turn_right().ok(),
                zzvec.turn_left().ok(),
            ]
        } else {
            vec![zzvec.turn_left().ok(), zzvec.turn_right().ok()]
        };
        position_option_vec
            .into_iter()
            .flatten()
            .filter(|zzvec| {
                // let through only positions that are in the grid
                grid.get(&zzvec.position()).is_some()
            })
            .collect_vec()
    };

    let djikstra = DjikstraSolver::new(&adjacency_fn, &cost_fn, &is_end_fn);
    djikstra.cost(starts).unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.col, self.row)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vector {
    position: Position,
    direction: Direction,
}

impl Vector {
    const fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
    fn straight(&self) -> Result<Self, ()> {
        let position = match self.direction {
            Direction::Up if self.position.row > 0 => Ok(Position {
                row: self.position.row - 1,
                col: self.position.col,
            }),
            Direction::Down => Ok(Position {
                row: self.position.row + 1,
                col: self.position.col,
            }),
            Direction::Left if self.position.col > 0 => Ok(Position {
                row: self.position.row,
                col: self.position.col - 1,
            }),
            Direction::Left | Direction::Up => Err(()),
            Direction::Right => Ok(Position {
                row: self.position.row,
                col: self.position.col + 1,
            }),
        };
        position.map(|position| Self {
            position,
            direction: self.direction,
        })
    }
    fn right(&self) -> Result<Self, ()> {
        Self {
            position: self.position,
            direction: match self.direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
        }
        .straight()
    }
    fn left(&self) -> Result<Self, ()> {
        Self {
            position: self.position,
            direction: match self.direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
        }
        .straight()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ZigZagVector {
    vector: Vector,
    consecutive_straight: usize,
}

impl ZigZagVector {
    const fn new(vector: Vector) -> Self {
        Self {
            vector,
            consecutive_straight: 0,
        }
    }
    fn straight(&self) -> Result<Self, ()> {
        let vector = self.vector.straight();
        vector.map(|vector| Self {
            vector,
            consecutive_straight: self.consecutive_straight + 1,
        })
    }
    //  Turn functions, change direction and move one square in that new direction

    fn turn_right(&self) -> Result<Self, ()> {
        let vector = self.vector.right();
        vector.map(|vector| Self {
            vector,
            consecutive_straight: 1,
        })
    }
    fn turn_left(&self) -> Result<Self, ()> {
        let vector = self.vector.left();
        vector.map(|vector| Self {
            vector,
            consecutive_straight: 1,
        })
    }
    const fn position(&self) -> Position {
        self.vector.position
    }
}

#[derive(Debug, Clone)]
struct Grid {
    data: Vec<Vec<u8>>,
    num_rows: usize,
    num_cols: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.iter().try_for_each(|row| {
            row.iter().try_for_each(|cell| write!(f, "{cell}"))?;
            writeln!(f)
        })
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| (c as u8) - 48u8) // 48 is the ascii code for '0'
                    .collect_vec()
            })
            .collect_vec();
        let num_rows = data.len();
        let num_cols = data[0].len();
        Ok(Self {
            data,
            num_rows,
            num_cols,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct DjikstraState<T> {
    data: T,
    distance: usize,
}

impl<T> DjikstraState<T> {
    const fn new(data: T, distance: usize) -> Self {
        Self { data, distance }
    }
}

impl<T> Ord for DjikstraState<T>
where
    T: PartialEq + Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // do the opposite comparison so that the BinaryHeap is a min heap
        other.distance.cmp(&self.distance)
    }
}

impl<T> PartialOrd for DjikstraState<T>
where
    T: PartialEq + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct DjikstraSolver<'a, T> {
    adjacency_fn: &'a dyn Fn(T) -> Vec<T>,
    cost_fn: &'a dyn Fn(T) -> usize,
    is_end_fn: &'a dyn Fn(T) -> bool,
}

impl<'a, T> DjikstraSolver<'a, T>
where
    T: Eq + PartialEq + Hash + Clone + Debug,
{
    pub fn new(
        adjacency_fn: &'a dyn Fn(T) -> Vec<T>,
        cost_fn: &'a dyn Fn(T) -> usize,
        is_end_fn: &'a dyn Fn(T) -> bool,
    ) -> Self {
        Self {
            adjacency_fn,
            cost_fn,
            is_end_fn,
        }
    }

    pub fn cost(&self, starts: Vec<T>) -> Option<usize> {
        let mut dist_map: HashMap<T, usize> = HashMap::new();
        let mut heap = BinaryHeap::new();

        for start in starts {
            dist_map.insert(start.clone(), 0);
            heap.push(DjikstraState::new(start.clone(), 0));
        }

        while let Some(DjikstraState {
            data: node,
            distance: cost,
        }) = heap.pop()
        {
            if (self.is_end_fn)(node.clone()) {
                return Some(cost);
            }

            let dist = *dist_map.get(&node).unwrap_or(&usize::MAX);
            if cost > dist {
                continue;
            }

            for neighbour in (self.adjacency_fn)(node) {
                let neighbour_cost = (self.cost_fn)(neighbour.clone());
                let next = DjikstraState::new(neighbour, cost + neighbour_cost);

                let dist_to_next = dist_map.get(&next.data).unwrap_or(&usize::MAX);
                if next.distance < *dist_to_next {
                    *dist_map.entry(next.clone().data).or_insert(usize::MAX) = next.distance;

                    heap.push(next);
                }
            }
        }

        None
    }
}

impl Grid {
    fn get(&self, position: &Position) -> Option<u8> {
        self.data
            .get(position.row)
            .and_then(|row| row.get(position.col))
            .copied()
    }
}

fn part2(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    // start in top left
    let start = Position { row: 0, col: 0 };
    let starts = vec![
        ZigZagVector::new(Vector::new(start, Direction::Right)),
        ZigZagVector::new(Vector::new(start, Direction::Down)),
    ];

    let cost_fn = |zzvec: ZigZagVector| -> usize { grid.get(&zzvec.position()).unwrap() as usize };
    let is_end_fn = |zzvec: ZigZagVector| -> bool {
        zzvec.position()
            == (Position {
                row: grid.num_rows - 1,
                col: grid.num_cols - 1,
            })
        // ultra crucible can't stop unless it has moved 4 times in straight direction
        && zzvec.consecutive_straight >= 4
    };
    let adjacency_fn = |zzvec: ZigZagVector| -> Vec<ZigZagVector> {
        let position_option_vec = if zzvec.consecutive_straight < 4 {
            vec![zzvec.straight().ok()]
        } else if zzvec.consecutive_straight >= 4 && zzvec.consecutive_straight < 10 {
            vec![
                zzvec.straight().ok(),
                zzvec.turn_right().ok(),
                zzvec.turn_left().ok(),
            ]
        } else {
            vec![zzvec.turn_left().ok(), zzvec.turn_right().ok()]
        };
        position_option_vec
            .into_iter()
            .flatten()
            .filter(|zzvec| {
                // let through only positions that are in the grid
                grid.get(&zzvec.vector.position).is_some()
            })
            .collect_vec()
    };

    let djikstra = DjikstraSolver::new(&adjacency_fn, &cost_fn, &is_end_fn);
    djikstra.cost(starts).unwrap()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_part1() {
        let input = read_to_string("inputs/day-17.example.txt").unwrap();
        assert_eq!(part1(&input), 102);
    }
    #[test]
    fn test_grid_to_string() {
        let input = indoc! {"
            123
            456
        "};
        let grid = Grid::from_str(input).unwrap();
        assert_eq!(grid.to_string(), input);
    }
    #[test]
    fn test_zigzag_fails() {
        // test that zigzag can't djikstra to end because it can't go straight
        let input = indoc! {"
            12345
        "};
        let grid = Grid::from_str(input).unwrap();
        let start = Position { row: 0, col: 0 };
        let cost_fn =
            |zzvec: ZigZagVector| -> usize { grid.get(&zzvec.position()).unwrap() as usize };
        let is_end_fn = |zzvec: ZigZagVector| -> bool {
            zzvec.position()
                == (Position {
                    row: grid.num_rows - 1,
                    col: grid.num_cols - 1,
                })
        };
        let adjacency_fn =
            |zzvec: ZigZagVector| -> Vec<ZigZagVector> { zzvec.on_board_neighbors(&grid) };

        let total_dist = DjikstraSolver::new(&adjacency_fn, &cost_fn, &is_end_fn).cost(vec![
            ZigZagVector::new(Vector::new(start, Direction::Right)),
            ZigZagVector::new(Vector::new(start, Direction::Down)),
        ]);
        assert!(total_dist.is_none())
    }
    #[test]
    fn test_zigzag_turn_fails() {
        // test that zigzag can't djikstra to end because it can't go straight
        let input = indoc! {"
            15555
            22222
        "};
        let grid = Grid::from_str(input).unwrap();
        let start = Position { row: 0, col: 0 };
        let cost_fn =
            |zzvec: ZigZagVector| -> usize { grid.get(&zzvec.position()).unwrap() as usize };
        let is_end_fn = |zzvec: ZigZagVector| -> bool {
            zzvec.position()
                == (Position {
                    row: grid.num_rows - 1,
                    col: grid.num_cols - 1,
                })
        };
        let adjacency_fn =
            |zzvec: ZigZagVector| -> Vec<ZigZagVector> { zzvec.on_board_neighbors(&grid) };

        let total_dist = DjikstraSolver::new(&adjacency_fn, &cost_fn, &is_end_fn).cost(vec![
            ZigZagVector::new(Vector::new(start, Direction::Right)),
            ZigZagVector::new(Vector::new(start, Direction::Down)),
        ]);
        // its reachable, but needs to pick up a 5 from top row
        assert_eq!(total_dist, Some(13));
    }
    #[test]
    fn test_zigzag_path() {
        let input = indoc! {"
            241343231
            321545353
        "};
        // expected Path: length 32
        // >>>34^>>>
        // 32v>>>35v

        assert_eq!(part1(input), 32);
    }
    #[test]
    fn test_part2() {
        let input = read_to_string("inputs/day-17.example.txt").unwrap();
        assert_eq!(part2(&input), 94);
    }
}
