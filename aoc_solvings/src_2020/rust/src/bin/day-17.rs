use std::{collections::HashMap, env};

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    let problem = ProblemStatement::parse_input(&input_str);
    println!("Part 1: {}", problem.part1());
    println!("Part 2: {}", problem.part2());
}

fn get_input(input_file: &String) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Point {
    pub fn new(x: isize, y: isize, z: Option<isize>, w: Option<isize>) -> Self {
        Self {
            x,
            y,
            z: z.unwrap_or(0),
            w: w.unwrap_or(0),
        }
    }
    pub fn neighbors(
        &self,
        include_4d: Option<bool>,
    ) -> impl Iterator<Item = (isize, isize, isize, isize)> + '_ {
        let include_4d = include_4d.unwrap_or(false);
        // create product between the x, y, z inclusive ranges
        // then filter out its own coordinates
        // only use the w dimension if include_4d is true
        (self.x - 1..=self.x + 1)
            .flat_map(move |x| (self.y - 1..=self.y + 1).map(move |y| (x, y)))
            .flat_map(move |(x, y)| (self.z - 1..=self.z + 1).map(move |z| (x, y, z)))
            .flat_map(move |(x, y, z)| {
                if include_4d {
                    (self.w - 1..=self.w + 1)
                        .map(move |w| (x, y, z, w))
                        .collect_vec()
                } else {
                    vec![(x, y, z, 0)]
                }
            })
            .filter(move |(x, y, z, w)| {
                !(*x == self.x && *y == self.y && *z == self.z && *w == self.w)
            })
    }
    pub const fn as_tuple(&self) -> (isize, isize, isize, isize) {
        (self.x, self.y, self.z, self.w)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Cube {
    point: Point,
    active: bool,
}

impl Cube {
    pub fn neighbors(
        &self,
        include_4d: Option<bool>,
    ) -> impl Iterator<Item = (isize, isize, isize, isize)> + '_ {
        // create product between the x, y, z inclusive ranges
        // then filter out its own coordinates
        self.point.neighbors(include_4d)
    }
    pub fn activate(&mut self, active_neighbors: usize) {
        if self.active && !(2..=3).contains(&active_neighbors) {
            self.active = false;
        } else if !self.active && active_neighbors == 3 {
            self.active = true;
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ProblemStatement {
    curr_cycle: usize,
    known_cubes: HashMap<(isize, isize, isize, isize), Cube>,
}

impl ProblemStatement {
    /// # Panics
    /// Panics if input has characters other than those allowed ('#' or '.')
    #[must_use]
    pub fn parse_input(input: &str) -> Self {
        // make assumption that given 2D slice of 3D and that Point defaults are sensible
        let known_cubes = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let active = match c {
                            '#' => true,
                            '.' => false,
                            _ => panic!("Unknown character"),
                        };
                        let loc_point =
                            Point::new(x.try_into().unwrap(), y.try_into().unwrap(), None, None);
                        (
                            loc_point.as_tuple(),
                            Cube {
                                point: loc_point,
                                active,
                            },
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Self {
            curr_cycle: 0,
            known_cubes,
        }
    }

    fn get_cube(&self, x: isize, y: isize, z: isize, w: Option<isize>) -> Cube {
        let w = w.unwrap_or(0);
        self.known_cubes
            .get(&(x, y, z, w))
            .cloned()
            .unwrap_or_else(|| Cube {
                point: Point::new(x, y, Some(z), Some(w)),
                active: false,
            })
    }

    fn run_cycle(mut self, include_4d: Option<bool>) -> Self {
        let mut new_cubes = HashMap::new();
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;
        let mut min_z = isize::MAX;
        let mut max_z = isize::MIN;
        let mut min_w = isize::MAX;
        let mut max_w = isize::MIN;
        for cube in self.known_cubes.values() {
            min_x = min_x.min(cube.point.x);
            max_x = max_x.max(cube.point.x);
            min_y = min_y.min(cube.point.y);
            max_y = max_y.max(cube.point.y);
            min_z = min_z.min(cube.point.z);
            max_z = max_z.max(cube.point.z);
            min_w = min_w.min(cube.point.w);
            max_w = max_w.max(cube.point.w);
        }
        if !include_4d.unwrap_or(false) {
            min_w = 1;
            max_w = -1;
        }
        for x in min_x - 1..=max_x + 1 {
            for y in min_y - 1..=max_y + 1 {
                for z in min_z - 1..=max_z + 1 {
                    for w in min_w - 1..=max_w + 1 {
                        let cube = self.get_cube(x, y, z, Some(w));
                        let active_neighbors = cube
                            .neighbors(include_4d)
                            .filter(|(x, y, z, w)| self.get_cube(*x, *y, *z, Some(*w)).active)
                            .collect_vec();
                        let mut new_cube = cube.clone();
                        new_cube.activate(active_neighbors.len());
                        new_cubes.insert((x, y, z, w), new_cube);
                    }
                }
            }
        }

        self.known_cubes = new_cubes;
        self.curr_cycle += 1;
        self
    }

    fn count_active_cubes(&self) -> usize {
        self.known_cubes.values().filter(|cube| cube.active).count()
    }

    #[must_use]
    pub fn part1(&self) -> usize {
        let mut problem = self.clone();
        for _ in 0..6 {
            problem = problem.run_cycle(None);
        }
        problem.count_active_cubes()
    }

    #[must_use]
    pub fn part2(&self) -> usize {
        let mut problem = self.clone();
        for _ in 0..6 {
            problem = problem.run_cycle(Some(true));
        }
        problem.count_active_cubes()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    const TEST_FILE: &str = "inputs/test-17.txt";

    #[test]
    fn test_neighbor_generation_3d() {
        let loc = Point {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        };
        let neighbors: HashSet<_> = loc.neighbors(None).collect();
        assert_eq!(neighbors.len(), 26);
        assert_eq!(neighbors.contains(&(0, 0, 0, 0)), false);
    }

    #[test]
    fn test_neighbor_generation_4d() {
        let loc = Point {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        };
        let neighbors: HashSet<_> = loc.neighbors(Some(true)).collect();
        assert_eq!(neighbors.len(), 80);
        assert_eq!(neighbors.contains(&(0, 0, 0, 0)), false);
    }

    #[test]
    fn test_cube_active_activate() {
        let mut cube = Cube {
            point: Point {
                x: 0,
                y: 0,
                z: 0,
                w: 0,
            },
            active: true,
        };
        cube.activate(2);
        assert!(cube.active);
        cube.activate(3);
        assert!(cube.active);
        for neighbors in 0..=1 {
            let mut new_cube = cube.clone();
            new_cube.activate(neighbors);
            assert_eq!(new_cube.active, false);
        }
        // each cube can have at most 80 neighbors in 4d space
        // 26 neighbors in 3d space
        for neighbors in 4..=80 {
            let mut new_cube = cube.clone();
            new_cube.activate(neighbors);
            assert_eq!(new_cube.active, false);
        }
    }

    #[test]
    fn test_cube_inactive_activate() {
        let mut cube = Cube {
            point: Point {
                x: 0,
                y: 0,
                z: 0,
                w: 0,
            },
            active: false,
        };
        cube.activate(3);
        assert!(cube.active);
        cube.active = false;
        for neighbors in 0..=2 {
            let mut new_cube = cube.clone();
            new_cube.activate(neighbors);
            assert_eq!(new_cube.active, false);
        }
        for neighbors in 4..=80 {
            let mut new_cube = cube.clone();
            new_cube.activate(neighbors);
            assert_eq!(new_cube.active, false);
        }
    }

    #[test]
    fn test_part1_example() {
        let problem = ProblemStatement::parse_input(&get_input(&TEST_FILE.to_string()));
        assert_eq!(problem.part1(), 112);
    }

    #[test]
    fn test_part2_single_cycle() {
        let problem = ProblemStatement::parse_input(&get_input(&TEST_FILE.to_string()));
        assert_eq!(problem.count_active_cubes(), 5);
        let problem = problem.run_cycle(Some(true));
        assert_eq!(problem.count_active_cubes(), 29);
    }

    #[test]
    fn test_part2_two_cycle() {
        let mut problem = ProblemStatement::parse_input(&get_input(&TEST_FILE.to_string()));
        assert_eq!(problem.count_active_cubes(), 5);
        for _ in 0..2 {
            problem = problem.run_cycle(Some(true));
        }
        assert_eq!(problem.count_active_cubes(), 60);
    }

    #[test]
    fn test_part2_example() {
        let problem = ProblemStatement::parse_input(&get_input(&TEST_FILE.to_string()));
        assert_eq!(problem.part2(), 848);
    }
}
