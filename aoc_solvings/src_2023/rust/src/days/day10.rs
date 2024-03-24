use crate::etc::{Solution, SolutionPair};

use std::{collections::HashSet, fs::read_to_string, str::FromStr};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("inputs/day-10.txt")).unwrap();
    (Solution::from(part1(&input)), Solution::from(part2(&input)))
}

// Build the problem
// Find the loop
// traverse the loop from start to start
// count the steps
// return the number of steps / 2
fn part1(input: &str) -> u64 {
    // have a visited set to make sure we don't visit the same tile twice
    let problem = Problem::from_str(input).unwrap();
    let loop_path = find_loop(&problem);
    u64::try_from(loop_path.len() / 2).unwrap()
}

/// Returns the length of the loop from start to start
/// Uses a prev and current pointer to take only one step in a direction that is not the previous tile
/// Each pipe that is part of the loop (S included) will only have 2 valid neighbors
/// So we never need to backtrack and just need to advance without retracing a step
fn find_loop(problem: &Problem) -> Vec<Tile> {
    let start_tile = problem.get_tile(&problem.start).unwrap();
    let mut path = vec![start_tile.clone()];
    let mut previous = problem.get_tile(&problem.start).unwrap();
    let mut current = previous;
    let (row_limit, col_limit) = (problem.grid.len(), problem.grid[0].len());
    // move current to be one tile off of start
    // this is so we can start the loop
    current = current
        .loc
        .cardinal_neighbors()
        .map(|pos| problem.get_tile(&pos).unwrap())
        .filter(|&t| start_tile.is_connected(t))
        .take(1)
        .next()
        .unwrap();
    path.push(current.clone());
    loop {
        if current.loc == problem.start {
            break;
        }

        let new_tile = current
            .loc
            .cardinal_neighbors()
            .filter(|pos| pos != &previous.loc)
            .filter(|pos| pos.y < row_limit && pos.x < col_limit)
            .map(|pos| problem.get_tile(&pos).unwrap())
            .filter(|&other_tile| current.is_connected(other_tile))
            .take(1)
            .next();
        path.push(new_tile.unwrap().clone());
        previous = current;
        current = new_tile.unwrap();
    }
    // last element is going to be the start tile again
    // remove the duplicate start
    path.pop();
    path
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    const fn is_adjacent(&self, other: &Self) -> bool {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) == 1
    }
    fn cardinal_neighbors(&self) -> impl Iterator<Item = Self> {
        let x = self.x;
        let y = self.y;
        let mut neighbors = vec![Self { x: x + 1, y }, Self { x, y: y + 1 }];
        if x != 0 {
            neighbors.push(Self { x: x - 1, y });
        }
        if y != 0 {
            neighbors.push(Self { x, y: y - 1 });
        }
        neighbors.into_iter()
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    // grid is organized that grid[x][y] is the tile at (x, y)
    // first row is y = 0, first column is x = 0
    grid: Vec<Vec<Tile>>,
    start: Position,
}

impl Problem {
    fn get_tile(&self, loc: &Position) -> Option<&Tile> {
        self.grid.get(loc.y).and_then(|f| f.get(loc.x))
    }
}

impl FromStr for Problem {
    type Err = InputParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .take_while(|line| !line.is_empty())
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let kind = match c {
                            '.' => TileKind::Empty,
                            '|' => TileKind::Pipe(PipeKind::Vertical),
                            '-' => TileKind::Pipe(PipeKind::Horizontal),
                            'S' => TileKind::Pipe(PipeKind::Start),
                            'L' => TileKind::Pipe(PipeKind::LBend),
                            'J' => TileKind::Pipe(PipeKind::JBend),
                            '7' => TileKind::Pipe(PipeKind::SevenBend),
                            'F' => TileKind::Pipe(PipeKind::FBend),
                            _ => return Err(InputParserError::InvalidChar(c)),
                        };
                        Ok(Tile {
                            loc: Position { x, y },
                            kind,
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        let start = grid
            .iter()
            .flat_map(|row| row.iter())
            .find(|tile| tile.kind == TileKind::Pipe(PipeKind::Start))
            .map(|tile| tile.loc.clone())
            .unwrap();
        Ok(Self { grid, start })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
    loc: Position,
    kind: TileKind,
}

impl Tile {
    /// Returns true if the tile is connected to the given tile, false otherwise
    /// Is specifically for testing tiles on x=0 or y=0
    /// if the tile is on x=0 then specific types can connect to it
    /// same for the y=0 case
    fn edge_test_is_connected(&self, tile: &Self) -> bool {
        let (self_loc, other_loc) = (&self.loc, &tile.loc);
        let TileKind::Pipe(pipe_kind) = &self.kind else {
            unreachable!()
        };
        let TileKind::Pipe(other_pipe_kind) = &tile.kind else {
            unreachable!()
        };
        match (self_loc.x, self_loc.y) {
            (0, 0) => {
                match pipe_kind {
                    PipeKind::FBend => match other_pipe_kind {
                        // this one is always true because start tile can only be in available adja
                        PipeKind::Start => true,
                        PipeKind::Horizontal | PipeKind::SevenBend => other_loc.x == 1, // self.x == 0
                        PipeKind::Vertical | PipeKind::LBend => other_loc.y == 1, // self.y == 0
                        PipeKind::FBend | PipeKind::JBend => false,
                    },
                    PipeKind::Start => match other_pipe_kind {
                        PipeKind::Horizontal | PipeKind::SevenBend => other_loc.x == 1, // self.x == 0
                        PipeKind::Vertical | PipeKind::LBend => other_loc.y == 1, // self.y == 0
                        PipeKind::FBend | PipeKind::JBend => false,
                        // can't be multiple
                        PipeKind::Start => unreachable!("There is only one start tile!"),
                    },
                    PipeKind::Horizontal
                    | PipeKind::Vertical
                    | PipeKind::JBend
                    | PipeKind::SevenBend
                    | PipeKind::LBend => false,
                }
            }
            (0, y) => {
                match pipe_kind {
                    // prune out types that would then connect to something off the grid
                    PipeKind::Horizontal | PipeKind::JBend | PipeKind::SevenBend => false,
                    PipeKind::Vertical => match other_pipe_kind {
                        PipeKind::Vertical | PipeKind::Start => y.abs_diff(other_loc.y) == 1,
                        // pipes that connect to the bottom of vertical
                        PipeKind::LBend => y + 1 == other_loc.y,
                        // pipes that connect to the top of vertical
                        PipeKind::FBend => y - 1 == other_loc.y,
                        _ => false,
                    },
                    PipeKind::FBend => match other_pipe_kind {
                        PipeKind::Start => other_loc.y == y + 1 || other_loc.x == 1, // self.x == 0
                        // pipes that connect to the east of FBend
                        PipeKind::JBend | PipeKind::SevenBend | PipeKind::Horizontal => {
                            other_loc.x == 1 // self.x == 0
                        }
                        // pipes that connect to the south of FBend
                        PipeKind::LBend | PipeKind::Vertical => y + 1 == other_loc.y,
                        PipeKind::FBend => false,
                    },
                    PipeKind::LBend => match other_pipe_kind {
                        PipeKind::Start => other_loc.x == 1 || y - 1 == other_loc.y, // self.x == 0
                        // pipes that connect to the east of LBend
                        PipeKind::JBend | PipeKind::Horizontal | PipeKind::SevenBend => {
                            other_loc.x == 1 // self.x == 0
                        }
                        // pipes that connect to the north of LBend
                        PipeKind::FBend | PipeKind::Vertical => y - 1 == other_loc.y,
                        PipeKind::LBend => false,
                    },
                    // make sure connecting other_loc does not lead to off board
                    PipeKind::Start => match other_pipe_kind {
                        PipeKind::SevenBend | PipeKind::JBend | PipeKind::Horizontal => {
                            other_loc.x == 1
                        } // self.x == 0
                        PipeKind::Vertical => y.abs_diff(other_loc.y) == 1,
                        PipeKind::LBend => y + 1 == other_loc.y,
                        PipeKind::FBend => y - 1 == other_loc.y,
                        // can't be multiple
                        PipeKind::Start => unreachable!("There is only one start tile!"),
                    },
                }
            }
            (x, 0) => {
                match pipe_kind {
                    // prune out types that would then connect to something off the grid
                    PipeKind::JBend | PipeKind::LBend | PipeKind::Vertical => false,
                    PipeKind::Horizontal => match other_pipe_kind {
                        PipeKind::Start | PipeKind::Horizontal => x.abs_diff(other_loc.x) == 1,
                        // pipes that connect to the left of horizontal
                        PipeKind::FBend => x - 1 == other_loc.x,
                        // pipes that connect to the right of horizontal
                        PipeKind::SevenBend => x + 1 == other_loc.x,
                        PipeKind::Vertical | PipeKind::LBend | PipeKind::JBend => false,
                    },
                    PipeKind::FBend => match other_pipe_kind {
                        // prune out types that would then connect to something off the grid
                        PipeKind::Start => x + 1 == other_loc.x || other_loc.y == 1, // self.y == 0
                        // pipes that connect to the east of FBend
                        PipeKind::SevenBend | PipeKind::Horizontal => x + 1 == other_loc.x,
                        // pipes that connect to the south of FBend
                        PipeKind::LBend | PipeKind::JBend | PipeKind::Vertical => other_loc.y == 1, // self.y == 0
                        PipeKind::FBend => false,
                    },
                    PipeKind::SevenBend => match other_pipe_kind {
                        // prune out types that would then connect to something off the grid
                        PipeKind::Start => x - 1 == other_loc.x || other_loc.y == 1, // self.y == 0
                        // pipes that connect to the west of SevenBend
                        PipeKind::FBend | PipeKind::Horizontal => x - 1 == other_loc.x,
                        // pipes that connect to the south of SevenBend
                        PipeKind::JBend | PipeKind::Vertical | PipeKind::LBend => other_loc.y == 1, // self.y == 0
                        PipeKind::SevenBend => false,
                    },
                    PipeKind::Start => match other_pipe_kind {
                        PipeKind::Horizontal => x.abs_diff(other_loc.x) == 1,
                        // must be below start
                        PipeKind::Vertical | PipeKind::JBend | PipeKind::LBend => other_loc.y == 1, // self.y == 0
                        PipeKind::SevenBend => x + 1 == other_loc.x,
                        PipeKind::FBend => x - 1 == other_loc.x,
                        // can't be multiple
                        PipeKind::Start => unreachable!("There is only one start tile!"),
                    },
                }
            }
            _ => unreachable!(),
        }
    }
    /// Returns true if the tile is connected to the given tile
    /// Takes into consideration: physical location (cardinal direction) and pipe type
    fn is_connected(&self, tile: &Self) -> bool {
        let self_loc = &self.loc;
        let other_loc = &tile.loc;
        // make sure the tiles are adjacent
        // only adjacent in cardinal directions so combos are (1, 0), (-1, 0), (0, 1), (0, -1)
        // also check that both tiles are on grid x > 0 and y > 0
        // this off grid negatively is enforced by type guarantees
        if !self_loc.is_adjacent(other_loc) {
            return false;
        }
        // empty tiles can't connect to anything
        if self.kind == TileKind::Empty || tile.kind == TileKind::Empty {
            return false;
        }
        // double check that we won't have underflow issues
        // pipes on x=0 or y=0 can connect to other pipes
        // but to avoid underflow issues we need to test specifically for those cases
        if self_loc.x == 0 || self_loc.y == 0 {
            return self.edge_test_is_connected(tile);
        }

        let TileKind::Pipe(pipe_kind) = &self.kind else {
            unreachable!()
        };
        let TileKind::Pipe(other_pipe_kind) = &tile.kind else {
            unreachable!()
        };
        // TODO: should all the subtraction tests be a checked_sub?
        match pipe_kind {
            // only to pipes of east or west connector type, make sure that tile is x adjacent
            PipeKind::Horizontal => match other_pipe_kind {
                PipeKind::Horizontal | PipeKind::Start => self_loc.x.abs_diff(other_loc.x) == 1,
                // pipes that connect to the left of horizontal
                PipeKind::LBend | PipeKind::FBend => self_loc.x - 1 == other_loc.x,
                // pipes that connect to the right of horizontal
                PipeKind::JBend | PipeKind::SevenBend => self_loc.x + 1 == other_loc.x,
                PipeKind::Vertical => false,
            },
            // only pipes that connect to north or south connector type, make sure that other_loc is y adjacent
            PipeKind::Vertical => match other_pipe_kind {
                PipeKind::Vertical | PipeKind::Start => self_loc.y.abs_diff(other_loc.y) == 1,
                // pipes that connect to the top of vertical
                PipeKind::JBend | PipeKind::LBend => self_loc.y + 1 == other_loc.y,
                // pipes that connect to the bottom of vertical
                PipeKind::FBend | PipeKind::SevenBend => self_loc.y - 1 == other_loc.y,
                PipeKind::Horizontal => false,
            },
            // only pipes that connect to south or east connector type, make sure that other_loc is x or y adjacent
            PipeKind::FBend => match other_pipe_kind {
                PipeKind::JBend | PipeKind::Start => {
                    self_loc.x + 1 == other_loc.x || self_loc.y + 1 == other_loc.y
                }
                // pipes that connect to the east of FBend
                PipeKind::SevenBend | PipeKind::Horizontal => self_loc.x + 1 == other_loc.x,
                // pipes that connect to the south of FBend
                PipeKind::LBend | PipeKind::Vertical => self_loc.y + 1 == other_loc.y,
                PipeKind::FBend => false,
            },
            // only pipes that connect on north or west
            PipeKind::JBend => match other_pipe_kind {
                PipeKind::FBend | PipeKind::Start => {
                    self_loc.x - 1 == other_loc.x || self_loc.y - 1 == other_loc.y
                }
                // pipes that connect to the west of JBend
                PipeKind::Horizontal | PipeKind::LBend => self_loc.x - 1 == other_loc.x,
                // pipes that connect to the north of JBend
                PipeKind::Vertical | PipeKind::SevenBend => self_loc.y - 1 == other_loc.y,
                PipeKind::JBend => false,
            },
            // only pipes that have a south or east connector
            PipeKind::LBend => match other_pipe_kind {
                PipeKind::SevenBend | PipeKind::Start => {
                    self_loc.x + 1 == other_loc.x || self_loc.y - 1 == other_loc.y
                }
                // pipes that connect to the east of LBend
                PipeKind::JBend | PipeKind::Horizontal => self_loc.x + 1 == other_loc.x,
                // pipes that connect to the north of LBend
                PipeKind::FBend | PipeKind::Vertical => self_loc.y - 1 == other_loc.y,
                PipeKind::LBend => false,
            },
            // only pipes that have a east or north connector
            PipeKind::SevenBend => match other_pipe_kind {
                PipeKind::LBend | PipeKind::Start => {
                    self_loc.x - 1 == other_loc.x || self_loc.y + 1 == other_loc.y
                }
                // pipes that connect to the west of SevenBend
                PipeKind::FBend | PipeKind::Horizontal => self_loc.x - 1 == other_loc.x,
                // pipes that connect to the south of SevenBend
                PipeKind::JBend | PipeKind::Vertical => self_loc.y + 1 == other_loc.y,
                PipeKind::SevenBend => false,
            },
            // start is a wild card and impossible to consider in the self to other tile connection
            // so return the result of the reverse connection
            PipeKind::Start => tile.is_connected(self),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TileKind {
    Empty,
    Pipe(PipeKind),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PipeKind {
    Horizontal,
    Vertical,
    LBend,     // north to east connector
    JBend,     // North to west connector
    SevenBend, // South to west connector
    FBend,     // South to east connector
    // there is only 1 Start tile, but it can connect to any other tile
    Start, // S where the 'player' starts, can be any type of type
}

#[derive(Debug, PartialEq, Eq)]
pub enum InputParserError {
    InvalidChar(char),
}

fn part2(input: &str) -> u64 {
    let problem = Problem::from_str(input).unwrap();
    let loop_path = find_loop(&problem);
    let loop_positions: HashSet<Position> = loop_path.iter().map(|tile| tile.loc.clone()).collect();
    // More info: https://en.wikipedia.org/wiki/Nonzero-rule
    // use non zero parity to detect changes in y due to path components and then use the state of parity
    // to determine if we are inside or outside the loop
    let mut contained_tiles = 0;
    for row in problem.grid {
        let mut non_zero_parity = false;
        for tile in row {
            if loop_positions.contains(&tile.loc) {
                // if we are on the loop, we can change the parity of the current path on this row
                // only if the current tile alters our y position
                non_zero_parity = match tile.kind {
                    TileKind::Pipe(PipeKind::Vertical | PipeKind::LBend | PipeKind::JBend) => {
                        !non_zero_parity
                    }
                    _ => non_zero_parity,
                };
            } else {
                // if a tile is not on the loop, it could add grouped tiles based on parity
                if non_zero_parity {
                    contained_tiles += 1;
                }
            }
        }
    }
    contained_tiles
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = read_to_string("inputs/day-10.example.txt").unwrap();
        assert_eq!(part1(&input), 4);
    }
    #[test]
    fn test_from_str_problem() {
        let input = read_to_string("inputs/day-10.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        assert_eq!(problem.grid.len(), 5);
        assert_eq!(problem.grid[0].len(), 5);
        assert!(problem
            .grid
            .iter()
            .flat_map(|row| row.iter())
            .any(|tile| { tile.kind == TileKind::Pipe(PipeKind::Start) }));
        assert!(
            problem.grid.get(1).unwrap().get(1).unwrap().kind == TileKind::Pipe(PipeKind::Start)
        );
        assert_eq!(problem.start, Position { x: 1, y: 1 });
    }

    #[test]
    fn test_connection() {
        let input = read_to_string("inputs/day-10.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        let start = problem
            .grid
            .iter()
            .flat_map(|row| row.iter())
            .find(|tile| tile.kind == TileKind::Pipe(PipeKind::Start))
            .unwrap();
        let start_neigbhors = problem
            .grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|tile| start.loc.is_adjacent(&tile.loc));
        let mut connected_neighbors = start_neigbhors.filter(|tile| start.is_connected(tile));
        assert_eq!(connected_neighbors.clone().count(), 2);
        assert_eq!(
            connected_neighbors.next().unwrap().clone(),
            Tile {
                loc: Position { x: 2, y: 1 },
                kind: TileKind::Pipe(PipeKind::Horizontal),
            }
        );
        assert_eq!(
            connected_neighbors.next().unwrap().clone(),
            Tile {
                loc: Position { x: 1, y: 2 },
                kind: TileKind::Pipe(PipeKind::Vertical),
            }
        );
    }
    #[test]
    fn test_seven_vert_connection() {
        // test that these connect, without triggering edge case
        let input = indoc! {"
            ...
            .7.
            .|.
            .S.
        "};
        let problem = Problem::from_str(&input).unwrap();
        let seven_tile = problem.get_tile(&Position { x: 1, y: 1 }).unwrap();
        assert!(seven_tile.is_connected(problem.get_tile(&Position { x: 1, y: 2 }).unwrap()));
    }
    #[test]
    fn test_connection_edge() {
        // all problems must contain S, we don't need it here to test that FJ combo on top edge is "not connected"
        // not connected because the path goes off grid
        let input = indoc! {"
            FJS
        "};
        let problem = Problem::from_str(&input).unwrap();
        let tile = problem.get_tile(&Position { x: 0, y: 0 }).unwrap();
        let other_tile = problem.get_tile(&Position { x: 1, y: 0 }).unwrap();
        assert_eq!(tile.is_connected(other_tile), false);
    }
    #[test]
    fn test_path_search() {
        let input = read_to_string("inputs/day-10.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        assert_eq!(find_loop(&problem).len(), 8);
    }
    #[test]
    fn test_path_passby_start() {
        // make sure that if path loops back to adjacent to start
        // we only return if the adjacent tile is connected to start
        let input = indoc! {"
            .....
            .F--7
            .|S-J
            .LJ..
        "};
        let problem = Problem::from_str(&input).unwrap();
        assert_eq!(find_loop(&problem).len(), 10);
    }
    #[test]
    fn test_part2() {
        let input = read_to_string("inputs/day-10.example.txt").unwrap();
        assert_eq!(part2(&input), 1);
    }
    #[test]
    fn test_part2_description() {
        let input = indoc! {"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "};
        assert_eq!(part2(&input), 4);
    }
    #[test]
    fn test_part2_description_2() {
        let input = indoc! {"
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
        "};
        assert_eq!(part2(&input), 8);
    }
    #[test]
    fn test_part2_description_junk() {
        let input = indoc! {"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "};
        assert_eq!(part2(&input), 10);
    }
}
