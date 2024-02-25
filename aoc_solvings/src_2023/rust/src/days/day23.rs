use itertools::Itertools;

use crate::etc::{Solution, SolutionPair};

use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Debug, Display},
    fs::read_to_string,
    str::FromStr,
};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("inputs/day-23.txt")).unwrap();

    (Solution::from(part1(&input)), Solution::from(part2(&input)))
}

fn part1(input: &str) -> usize {
    let grid = input.parse::<Grid>().unwrap();
    let start = grid.start_position();
    let end = grid.end_position();

    let reduced_neighbors: HashMap<Position, (HashSet<Position>, usize)> =
        build_reduced_neighbors(&grid, start, end, None);
    travel_longest_path(&reduced_neighbors, start)
}
fn travel_longest_path(
    graph: &HashMap<Position, (HashSet<Position>, usize)>,
    curr_pos: Position,
) -> usize {
    let (neighbors, steps) = graph.get(&curr_pos).unwrap();
    if neighbors.len() == 1 {
        return *steps;
    }
    let mut max_steps = 0;
    for neighbor in neighbors {
        let steps_to_next = travel_longest_path(graph, *neighbor);
        max_steps = max_steps.max(steps_to_next);
    }
    max_steps + steps
}
/// build out a mapping of position to possible moves and how many moves between key to set of values
/// e.g. (0, 0) -> ({(0, 1), (1, 0)}, 1) - this means that from 0, 0 there are 2 possible moves, and each of them takes 1 step to get to
/// we have the extra step information to remove entries that are forced moves
fn build_reduced_neighbors(
    grid: &Grid,
    starting_position: Position,
    stop_position: Position,
    prior_graph: Option<HashMap<Position, (HashSet<Position>, usize)>>,
) -> HashMap<Position, (HashSet<Position>, usize)> {
    let mut prior_graph = prior_graph.unwrap_or_default();
    let mut previous_position = starting_position;
    let mut current_position = starting_position;
    let mut current_step = 1;
    loop {
        let mut orig_neighbors = grid
            .grid_neighbors(current_position)
            .iter()
            .copied()
            .collect::<HashSet<_>>();
        orig_neighbors.remove(&previous_position);

        if orig_neighbors.is_empty() {
            // hit a dead end, but it might be the end
            if current_position == stop_position {
                prior_graph.insert(
                    starting_position,
                    (HashSet::from([stop_position]), current_step - 1),
                );
            }
            return prior_graph;
        }
        if orig_neighbors.len() == 1 {
            // forced singular move, cycle back after updating position change
            previous_position = current_position;
            current_position = *orig_neighbors.iter().next().unwrap();
            current_step += 1;
            continue;
        }
        prior_graph.insert(
            starting_position,
            (
                orig_neighbors
                    .iter()
                    .copied()
                    .collect::<HashSet<Position>>(),
                current_step,
            ),
        );
        for neighbor in orig_neighbors {
            prior_graph = build_reduced_neighbors(grid, neighbor, stop_position, Some(prior_graph));
        }
        return prior_graph;
    }
}

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<Vec<HikingCell>>,
    num_rows: usize,
    num_cols: usize,
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        let num_cols = cells.get(0).unwrap().len();
        let num_rows = cells.len();
        Ok(Self {
            cells,
            num_rows,
            num_cols,
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.cells {
            for cell in line {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    /// Return the starting point in the grid
    ///
    /// Guaranteed to be a singular `HikingCell::Path` in the first row
    fn start_position(&self) -> Position {
        let line = self.cells.get(0).unwrap();
        // find the first path cell
        let col = line
            .iter()
            .position(|cell| *cell == HikingCell::Path)
            .unwrap();
        Position { row: 0, col }
    }

    /// Return the end position in the grid
    ///
    /// Guaranteed to be a singular `HikingCell::Path` in the last row
    fn end_position(&self) -> Position {
        let line = self.cells.get(self.num_rows - 1).unwrap();
        // find the first path cell
        let col = line
            .iter()
            .position(|cell| *cell == HikingCell::Path)
            .unwrap();
        Position {
            row: self.num_rows - 1,
            col,
        }
    }

    /// Return the cell at a given `Position`
    fn get(&self, pos: Position) -> HikingCell {
        self.cells[pos.row][pos.col]
    }

    /// Return valid moves from a given position
    ///
    /// Moves that are on slopes MUST move in the direction of the slope
    fn grid_neighbors(&self, pos: Position) -> Vec<Position> {
        let mut neighbors = vec![];
        match self.get(pos) {
            HikingCell::Path => {
                // from a path cell, we can move to any adjacent cell that is not a forest
                // but if we want to move to a Slope, then we must move in the same direction as that slope
                for neighbor in pos.neighbors() {
                    if neighbor.row < self.num_rows && neighbor.col < self.num_cols {
                        let neighbor_cell = self.get(neighbor);
                        if neighbor_cell == HikingCell::Path {
                            neighbors.push(neighbor);
                        } else if let HikingCell::Slope(dir) = neighbor_cell {
                            match dir {
                                Direction::Down => {
                                    // on down slope, we have to move down onto it
                                    if neighbor.row > pos.row {
                                        neighbors.push(neighbor);
                                    }
                                }
                                Direction::Left => {
                                    // on left slope, we have to move left onto it
                                    if neighbor.col < pos.col {
                                        neighbors.push(neighbor);
                                    }
                                }
                                Direction::Right => {
                                    if neighbor.col > pos.col {
                                        neighbors.push(neighbor);
                                    }
                                }
                                Direction::Up => {
                                    if neighbor.row < pos.row {
                                        neighbors.push(neighbor);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            HikingCell::Slope(dir) => {
                let possible_neighbor = match dir {
                    Direction::Down => Some(Position {
                        row: pos.row + 1,
                        col: pos.col,
                    }),
                    Direction::Left if pos.col > 0 => Some(Position {
                        row: pos.row,
                        col: pos.col - 1,
                    }),
                    Direction::Up if pos.row > 0 => Some(Position {
                        row: pos.row - 1,
                        col: pos.col,
                    }),
                    Direction::Right => Some(Position {
                        row: pos.row,
                        col: pos.col + 1,
                    }),
                    Direction::Left | Direction::Up => None,
                };
                if possible_neighbor.is_none() {
                    return vec![];
                }
                let neighbor = possible_neighbor.unwrap();
                if neighbor.row < self.num_rows && neighbor.col < self.num_cols {
                    neighbors.push(neighbor);
                }
            }
            HikingCell::Forest => {}
        }
        neighbors
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}

impl Position {
    fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = vec![];
        if self.row > 0 {
            neighbors.push(Self {
                row: self.row - 1,
                col: self.col,
            });
        }
        neighbors.push(Self {
            row: self.row + 1,
            col: self.col,
        });
        if self.col > 0 {
            neighbors.push(Self {
                row: self.row,
                col: self.col - 1,
            });
        }
        neighbors.push(Self {
            row: self.row,
            col: self.col + 1,
        });
        neighbors
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HikingCell {
    Path,
    Forest,
    Slope(Direction),
}

impl FromStr for HikingCell {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Path),
            "#" => Ok(Self::Forest),
            ">" => Ok(Self::Slope(Direction::Right)),
            "<" => Ok(Self::Slope(Direction::Left)),
            "^" => Ok(Self::Slope(Direction::Up)),
            "v" => Ok(Self::Slope(Direction::Down)),
            _ => panic!("Invalid character in input"),
        }
    }
}

impl Display for HikingCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Path => write!(f, "."),
            Self::Forest => write!(f, "#"),
            Self::Slope(Direction::Up) => write!(f, "^"),
            Self::Slope(Direction::Down) => write!(f, "v"),
            Self::Slope(Direction::Left) => write!(f, "<"),
            Self::Slope(Direction::Right) => write!(f, ">"),
        }
    }
}

fn initialize_corridor_graph(grid: &Grid) -> HashMap<Position, Vec<(Position, usize)>> {
    let mut graph: HashMap<Position, Vec<(Position, usize)>> = HashMap::new();
    for (row, col) in (0..grid.num_rows).cartesian_product(0..grid.num_cols) {
        let pos = Position { row, col };
        let neighbors = if grid.get(pos) == HikingCell::Forest {
            // forest cells have no neighbors
            continue;
        } else {
            pos.neighbors()
                .iter()
                .filter(|&pos| {
                    pos.row < grid.num_rows
                        && pos.col < grid.num_cols
                        && grid.get(*pos) != HikingCell::Forest
                })
                .copied()
                .collect::<HashSet<_>>()
        };
        graph.insert(pos, neighbors.into_iter().map(|p| (p, 1)).collect());
    }
    graph
}

fn part2(input: &str) -> usize {
    let grid = input.parse::<Grid>().unwrap();
    let start = grid.start_position();
    let end = grid.end_position();

    let mut graph = initialize_corridor_graph(&grid);

    // find cells where its a singular path (corridor)
    let corridor_cells: Vec<Position> = graph
        .iter()
        .filter(|(_, neighbors)| neighbors.len() == 2)
        .map(|(pos, _)| *pos)
        .collect();
    for pos in corridor_cells {
        // update the only neighbors of this cell to include the steps from these corridor spots
        // we also remove this corridor cell from the graph
        let neighbors = graph.remove(&pos).unwrap();
        let (first_neighbor, distance_1) = neighbors.get(0).unwrap();
        let (second_neighbor, distance_2) = neighbors.get(1).unwrap();
        let n1 = graph.get_mut(first_neighbor).unwrap();
        // find the position in the vector where the corridor cell is in their neighbor
        if let Some(i) = n1.iter().position(|&(inner_pos, _)| inner_pos == pos) {
            // join together the cells directly, preserving the distance by increasing it
            n1[i] = (*second_neighbor, distance_1 + distance_2);
        }
        let n2 = graph.get_mut(second_neighbor).unwrap();
        if let Some(i) = n2.iter().position(|&(inner_pos, _)| inner_pos == pos) {
            n2[i] = (*first_neighbor, distance_1 + distance_2);
        }
    }

    dfs(&graph, &mut HashSet::new(), &end, &start).unwrap()
}

fn dfs(
    graph: &HashMap<Position, Vec<(Position, usize)>>,
    seen: &mut HashSet<Position>,
    goal: &Position,
    curr: &Position,
) -> Option<usize> {
    if curr == goal {
        return Some(0);
    }
    let mut max_dist: Option<usize> = None;
    for &(next, d) in graph.get(curr).unwrap() {
        if seen.contains(&next) {
            continue;
        }
        seen.insert(next);
        if let Some(child_dist) = dfs(graph, seen, goal, &next) {
            max_dist = Some(max_dist.unwrap_or_default().max(d + child_dist));
        }
        seen.remove(&next);
    }
    max_dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_to_string("inputs/day-23.example.txt").unwrap();
        assert_eq!(part1(&input), 94);
    }
    #[test]
    fn test_part2() {
        let input = read_to_string("inputs/day-23.example.txt").unwrap();
        assert_eq!(part2(&input), 154);
    }
    #[test]
    fn test_parse() {
        let input = read_to_string("inputs/day-23.example.txt").unwrap();
        let grid = input.parse::<Grid>().unwrap();
        assert_eq!(grid.num_rows, 23);
        assert_eq!(grid.num_cols, 23);
    }
    #[test]
    fn test_grid_special_positions() {
        let input = read_to_string("inputs/day-23.example.txt").unwrap();
        let grid = input.parse::<Grid>().unwrap();
        assert_eq!(grid.start_position(), Position { row: 0, col: 1 });
        assert_eq!(grid.end_position(), Position { row: 22, col: 21 });
    }
}
