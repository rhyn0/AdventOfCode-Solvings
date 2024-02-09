use itertools::Itertools;

use crate::etc::{Solution, SolutionPair};

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    fs::read_to_string,
    num::ParseIntError,
    str::FromStr,
};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("inputs/day-22.txt")).unwrap();
    (Solution::from(part1(&input)), Solution::from(part2(&input)))
}

fn part1(input: &str) -> usize {
    let bricks = parse_input(input);
    let mut new_bricks = brick_fall(bricks);
    new_bricks.sort_by_key(|b| b.start.z);

    let (key_supports_all_values, key_supported_by_values) = build_support_maps(&new_bricks);
    // for every brick, check if the bricks it supports are supported by 2 or more bricks in total
    (0..new_bricks.len())
        .filter(|idx| {
            key_supports_all_values
                .get(idx)
                .cloned()
                .unwrap_or_default()
                .iter()
                .all(|supported_idx| {
                    key_supported_by_values
                        .get(supported_idx)
                        .map_or(false, |set| set.len() >= 2)
                })
        })
        .count()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec3D {
    x: usize,
    y: usize,
    z: usize,
}

impl Display for Vec3D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}
impl FromStr for Vec3D {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(',');
        let x = parts
            .next()
            .ok_or("missing x")?
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let y = parts
            .next()
            .ok_or("missing y")?
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let z = parts
            .next()
            .ok_or("missing z")?
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        Ok(Self { x, y, z })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SandBrick {
    start: Vec3D,
    end: Vec3D,
}

impl Display for SandBrick {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

impl FromStr for SandBrick {
    type Err = String;

    /// Parse a sand brick from a line
    /// Example: 1,0,1~1,2,1
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split('~');
        let start = parts.next().ok_or("missing start")?.parse()?;
        let end = parts.next().ok_or("missing end")?.parse()?;
        Ok(Self { start, end })
    }
}

impl SandBrick {
    /// Return whether two bricks overlap in the x and y axis
    fn overlaps(&self, other: &Self) -> bool {
        let x_max_start = self.start.x.max(other.start.x);
        let x_min_end = self.end.x.min(other.end.x);
        let y_max_start = self.start.y.max(other.start.y);
        let y_min_end = self.end.y.min(other.end.y);
        x_max_start <= x_min_end && y_max_start <= y_min_end
    }
}

fn parse_input(input: &str) -> Vec<SandBrick> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn brick_fall(bricks: Vec<SandBrick>) -> Vec<SandBrick> {
    let mut new_bricks = bricks;
    new_bricks.sort_by_key(|b| b.start.z);
    for idx in 0..new_bricks.len() {
        let mut brick = new_bricks[idx];
        let mut max_z = 1;
        for other_brick in new_bricks.iter().take(idx) {
            if brick.overlaps(other_brick) {
                max_z = max_z.max(other_brick.end.z + 1);
            }
        }
        brick.end.z -= brick.start.z - max_z;
        brick.start.z = max_z;
        new_bricks[idx] = brick;
    }
    new_bricks
}

fn build_support_maps(
    bricks: &[SandBrick],
) -> (
    HashMap<usize, HashSet<usize>>,
    HashMap<usize, HashSet<usize>>,
) {
    // now create a map of which bricks support which bricks and the bricks that support it
    let mut key_supports_all_values = HashMap::new();
    let mut key_supported_by_values = HashMap::new();
    bricks.iter().enumerate().for_each(|(idx, upper_brick)| {
        bricks
            .to_owned()
            .iter()
            .enumerate()
            .take(idx)
            .for_each(|(idx2, other_brick)| {
                if upper_brick.overlaps(other_brick) && upper_brick.start.z == other_brick.end.z + 1
                {
                    key_supports_all_values
                        .entry(idx2)
                        .or_insert_with(HashSet::new)
                        .insert(idx);
                    key_supported_by_values
                        .entry(idx)
                        .or_insert_with(HashSet::new)
                        .insert(idx2);
                }
            });
    });
    (key_supports_all_values, key_supported_by_values)
}

fn part2(input: &str) -> usize {
    let bricks = parse_input(input);
    let mut new_bricks = brick_fall(bricks);
    new_bricks.sort_by_key(|b| b.start.z);
    let total_bricks = new_bricks.len();
    let (key_supports_all_values, key_supported_by_values) = build_support_maps(&new_bricks);
    // visit each brick and build a queue of bricks to visit
    // only visit bricks if there is one thing supporting it, at first
    (0..total_bricks)
        .map(|idx| {
            let mut queue = VecDeque::new();
            queue.extend(
                key_supports_all_values
                    .get(&idx)
                    .cloned()
                    .unwrap_or_default()
                    .iter()
                    .filter(|&&i| key_supported_by_values.get(&i).unwrap().len() == 1),
            );
            let mut visited: HashSet<usize> = queue.iter().copied().collect();
            visited.insert(idx);
            while let Some(current_idx) = queue.pop_front() {
                let supported_bricks = key_supports_all_values
                    .get(&current_idx)
                    .cloned()
                    .unwrap_or_default();
                let bricks_that_will_fall =
                    supported_bricks.difference(&visited).copied().collect_vec();
                for next_idx in bricks_that_will_fall {
                    let supporting_bricks_are_falling = key_supported_by_values
                        .get(&next_idx)
                        .cloned()
                        .unwrap_or_default()
                        .is_subset(&visited);
                    if supporting_bricks_are_falling {
                        queue.push_back(next_idx);
                        visited.insert(next_idx);
                    }
                }
            }
            visited.len() - 1
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_part1() {
        let input = read_to_string("inputs/day-22.example.txt").unwrap();
        assert_eq!(part1(&input), 5);
    }
    #[test]
    fn test_part2() {
        let input = read_to_string("inputs/day-22.example.txt").unwrap();
        assert_eq!(part2(&input), 7);
    }
    #[test]
    fn test_parse() {
        let input = read_to_string("inputs/day-22.example.txt").unwrap();
        let bricks = parse_input(&input);
        assert_eq!(bricks.len(), 7);
        assert_eq!(
            bricks[0],
            SandBrick {
                start: Vec3D::new(1, 0, 1),
                end: Vec3D::new(1, 2, 1)
            }
        );
    }
    #[test]
    fn test_brick_range() {
        let brick = SandBrick {
            start: Vec3D::new(1, 0, 1),
            end: Vec3D::new(1, 2, 1),
        };
        let range = brick.range().collect_vec();
        assert_eq!(range.len(), 3);
        assert_eq!(
            range,
            vec![
                Vec3D::new(1, 0, 1),
                Vec3D::new(1, 1, 1),
                Vec3D::new(1, 2, 1)
            ]
        );
    }
    #[test]
    fn test_negative_brick_range() {
        // swap the start and end points of the previous test
        let brick = SandBrick {
            start: Vec3D::new(1, 2, 1),
            end: Vec3D::new(1, 0, 1),
        };
        let range = brick.range().collect_vec();
        assert_eq!(range.len(), 3);
        assert_eq!(
            range,
            vec![
                Vec3D::new(1, 0, 1),
                Vec3D::new(1, 1, 1),
                Vec3D::new(1, 2, 1),
            ]
        );
    }
    #[test]
    fn test_fall() {
        let input = read_to_string("inputs/day-22.example.txt").unwrap();
        let bricks = parse_input(&input);
        let new_bricks = brick_fall(bricks.clone());
        assert_eq!(new_bricks.len(), 7);
        // the first brick should be the one on the ground. example says this is line 1 (A)
        assert_eq!(
            new_bricks[0],
            SandBrick {
                start: Vec3D::new(1, 0, 1),
                end: Vec3D::new(1, 2, 1)
            }
        );
        // but that brick hasn't moved, we can check that bricks have properly fallen by
        // looking at the 3rd brick from example (C)
        assert_eq!(
            new_bricks[1],
            SandBrick {
                start: Vec3D::new(0, 0, 2),
                end: Vec3D::new(2, 0, 2)
            }
        );
        assert_eq!(
            new_bricks[2],
            SandBrick {
                start: Vec3D::new(0, 2, 2),
                end: Vec3D::new(2, 2, 2)
            }
        );
    }
}
