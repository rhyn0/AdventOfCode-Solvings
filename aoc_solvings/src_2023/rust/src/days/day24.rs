use itertools::Itertools;
use num_bigint::{BigInt, ToBigInt};
use num_traits::ToPrimitive;

use crate::etc::{Solution, SolutionPair};

use std::{
    fs::read_to_string,
    ops::{Div, Sub},
    str::FromStr,
};

pub fn solve(path: Option<&str>) -> SolutionPair {
    let input = read_to_string(path.unwrap_or("inputs/day-24.txt")).unwrap();
    (Solution::from(part1(&input)), Solution::from(part2(&input)))
}

fn part1(input: &str) -> usize {
    let problem: Problem = input.parse().unwrap();
    find_future_intersections(&problem)
}

/// Have to find the number of `Hailstone` pairs that do intersect each other in the future.
/// But only within a specific area. Each `Hailstone` has a position and a velocity. In the part1 problem,
/// we can ignore the Z axis. So we can build slope intercept equations for each `Hailstone` in the XY dimension and find the intersection point.
fn find_future_intersections(problem: &Problem) -> usize {
    problem
        .hailstones
        .iter()
        .enumerate()
        .flat_map(|(i, hailstone)| {
            problem
                .hailstones
                .iter()
                .skip(i + 1)
                .map(move |other_hailstone| (hailstone, other_hailstone))
        })
        .filter(|(hailstone, other_hailstone)| {
            if let Some((x, y)) = hailstone.xy_intersect_point(other_hailstone) {
                // test for positive time means that the intersection is in the future
                problem.in_bounds(x, y)
                    && hailstone.time_to_position(x, y, hailstone.position.z.to_f64().unwrap())
                        >= 0.0
                    && other_hailstone.time_to_position(
                        x,
                        y,
                        other_hailstone.position.z.to_f64().unwrap(),
                    ) >= 0.0
            } else {
                false
            }
        })
        .count()
}

#[derive(Debug, Clone)]
pub struct Problem {
    hailstones: Vec<Hailstone>,
    min_bound: i128,
    max_bound: i128,
}

impl Default for Problem {
    fn default() -> Self {
        Self {
            hailstones: Vec::new(),
            min_bound: 200_000_000_000_000,
            max_bound: 400_000_000_000_000,
        }
    }
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            hailstones: s
                .lines()
                .filter_map(|l| {
                    if l.is_empty() {
                        None
                    } else {
                        Some(l.parse().unwrap())
                    }
                })
                .collect(),
            ..Default::default()
        })
    }
}

impl Problem {
    fn in_bounds(&self, x: f64, y: f64) -> bool {
        x >= self.min_bound.to_f64().unwrap()
            && x <= self.max_bound.to_f64().unwrap()
            && y >= self.min_bound.to_f64().unwrap()
            && y <= self.max_bound.to_f64().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hailstone {
    position: Vec3D,
    /// Velocity is the change in the position for each dimension per nanosecond.
    velocity: Vec3D,
    acceleration: Vec3D,
}

impl Default for Hailstone {
    fn default() -> Self {
        Self {
            position: Vec3D { x: 0, y: 0, z: 0 },
            velocity: Vec3D { x: 0, y: 0, z: 0 },
            acceleration: Vec3D { x: 0, y: 0, z: 0 },
        }
    }
}

impl FromStr for Hailstone {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s.split_once('@').unwrap();
        let (pos_x, pos_y, pos_z) = position.trim().split(',').collect_tuple().unwrap();
        let (vel_x, vel_y, vel_z) = velocity.trim().split(',').collect_tuple().unwrap();
        Ok(Self {
            position: Vec3D {
                x: pos_x.trim().parse().unwrap(),
                y: pos_y.trim().parse().unwrap(),
                z: pos_z.trim().parse().unwrap(),
            },
            velocity: Vec3D {
                x: vel_x.trim().parse().unwrap(),
                y: vel_y.trim().parse().unwrap(),
                z: vel_z.trim().parse().unwrap(),
            },
            ..Default::default()
        })
    }
}

impl Hailstone {
    fn build_slope_intercept(&self) -> (f64, f64) {
        let slope = self.velocity.y.to_f64().unwrap() / self.velocity.x.to_f64().unwrap();
        let intercept = slope.mul_add(
            self.position.x.to_f64().unwrap(),
            self.position.y.to_f64().unwrap(),
        );
        (slope, intercept)
    }
    fn time_to_position(&self, x: f64, y: f64, z: f64) -> f64 {
        let x_time = (x - self.position.x.to_f64().unwrap()) / self.velocity.x.to_f64().unwrap();
        let y_time = (y - self.position.y.to_f64().unwrap()) / self.velocity.y.to_f64().unwrap();
        let z_time = (z - self.position.z.to_f64().unwrap()) / self.velocity.z.to_f64().unwrap();
        // do absolute value comparisons but return original sign
        let max = if y_time.abs() > x_time.abs() {
            y_time
        } else {
            x_time
        };
        if z_time.abs() > max.abs() {
            z_time
        } else {
            max
        }
    }
    fn xy_intersect_point(&self, other: &Self) -> Option<(f64, f64)> {
        let (slope, intercept) = self.build_slope_intercept();
        let (other_slope, other_intercept) = other.build_slope_intercept();
        if (slope - other_slope).abs() < 0.0001 {
            return None;
        }
        let x = (other_intercept - intercept) / (slope - other_slope);
        let y = slope.mul_add(x, intercept);
        Some((x, y))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec3D {
    x: i128,
    y: i128,
    z: i128,
}

impl Vec3D {
    const fn cross_product(&self, other: &Self) -> Self {
        Self {
            x: ((self.y * other.z) - (self.z * other.y)),
            y: ((self.z * other.x) - (self.x * other.z)),
            z: ((self.x * other.y) - (self.y * other.x)),
        }
    }
    fn dot_product(&self, other: &Self) -> BigInt {
        (self.x.to_bigint().unwrap() * other.x.to_bigint().unwrap())
            + (self.y.to_bigint().unwrap() * other.y.to_bigint().unwrap())
            + (self.z.to_bigint().unwrap() * other.z.to_bigint().unwrap())
    }
    const fn is_linear_independent(&self) -> bool {
        self.x != 0 || self.y != 0 || self.z != 0
    }
    #[allow(dead_code)]
    const fn from_linear(
        a_scalar: i128,
        a_vec: &Self,
        b_scalar: i128,
        b_vec: &Self,
        c_scalar: i128,
        c_vec: &Self,
    ) -> Self {
        let x = (a_scalar * a_vec.x) + (b_scalar * b_vec.x) + (c_scalar * c_vec.x);
        let y = (a_scalar * a_vec.y) + (b_scalar * b_vec.y) + (c_scalar * c_vec.y);
        let z = (a_scalar * a_vec.z) + (b_scalar * b_vec.z) + (c_scalar * c_vec.z);
        Self { x, y, z }
    }
    #[allow(dead_code)]
    const fn sum(&self) -> i128 {
        self.x + self.y + self.z
    }
}

impl Sub for Vec3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Div for Vec3D {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
impl Div<i128> for Vec3D {
    type Output = Self;

    #[allow(clippy::cast_possible_truncation)]
    fn div(self, rhs: i128) -> Self::Output {
        let x = (self.x.to_f64().unwrap() / rhs.to_f64().unwrap()).round() as i128;
        let y = (self.y.to_f64().unwrap() / rhs.to_f64().unwrap()).round() as i128;
        let z = (self.z.to_f64().unwrap() / rhs.to_f64().unwrap()).round() as i128;
        // let x = self.x / rhs;
        // let y = self.y / rhs;
        // let z = self.z / rhs;
        Self { x, y, z }
    }
}
impl Div<BigInt> for Vec3D {
    type Output = Self;
    #[allow(clippy::cast_possible_truncation)]
    fn div(self, rhs: BigInt) -> Self::Output {
        let x = (self.x.to_f64().unwrap() / rhs.to_f64().unwrap()).round() as i128;
        let y = (self.y.to_f64().unwrap() / rhs.to_f64().unwrap()).round() as i128;
        let z = (self.z.to_f64().unwrap() / rhs.to_f64().unwrap()).round() as i128;
        Self { x, y, z }
    }
}

impl Sub<BigIntVec3D> for Vec3D {
    type Output = Self;

    fn sub(self, rhs: BigIntVec3D) -> Self::Output {
        Self {
            x: self.x - rhs.x.to_i128().unwrap(),
            y: self.y - rhs.y.to_i128().unwrap(),
            z: self.z - rhs.z.to_i128().unwrap(),
        }
    }
}
// TODO: merge `Vec3D` and `BigIntVec3D` into a single struct, use generics to handle the different types.
#[derive(Debug, Clone)]
struct BigIntVec3D {
    x: BigInt,
    y: BigInt,
    z: BigInt,
}

impl BigIntVec3D {
    fn from_linear(
        a_scalar: &BigInt,
        a_vec: &Vec3D,
        b_scalar: &BigInt,
        b_vec: &Vec3D,
        c_scalar: &BigInt,
        c_vec: &Vec3D,
    ) -> Self {
        let x = (a_scalar.clone() * a_vec.x)
            + (b_scalar.clone() * b_vec.x)
            + (c_scalar.clone() * c_vec.x);
        let y = (a_scalar.clone() * a_vec.y)
            + (b_scalar.clone() * b_vec.y)
            + (c_scalar.clone() * c_vec.y);
        let z = (a_scalar * a_vec.z) + (b_scalar * b_vec.z) + (c_scalar * c_vec.z);
        Self { x, y, z }
    }
    fn sum(self) -> BigInt {
        &self.x + &self.y + &self.z
    }
}

impl Div<BigInt> for BigIntVec3D {
    type Output = Self;

    fn div(self, rhs: BigInt) -> Self::Output {
        let x = &self.x / rhs.clone();
        let y = &self.y / rhs.clone();
        let z = &self.z / rhs;
        Self { x, y, z }
    }
}

fn part2(input: &str) -> usize {
    let problem: Problem = input.parse().unwrap();
    let hailstone = problem.hailstones.first().unwrap();
    let (second_pos, hailstone_second) = problem
        .hailstones
        .iter()
        .find_position(|h| {
            h.velocity
                .cross_product(&hailstone.velocity)
                .is_linear_independent()
        })
        .unwrap();
    let hailstone_third = problem
        .hailstones
        .iter()
        .skip(second_pos + 1)
        .find(|h| {
            h.velocity
                .cross_product(&hailstone.velocity)
                .is_linear_independent()
                && h.velocity
                    .cross_product(&hailstone_second.velocity)
                    .is_linear_independent()
        })
        .unwrap();
    // find the intersecting planes for the pairs
    let (a_vec, a_dot) = find_plane(hailstone, hailstone_second);
    let (b_vec, b_dot) = find_plane(hailstone, hailstone_third);
    let (c_vec, c_dot) = find_plane(hailstone_second, hailstone_third);

    let w_vec = BigIntVec3D::from_linear(
        &a_dot,
        &b_vec.cross_product(&c_vec),
        &b_dot,
        &c_vec.cross_product(&a_vec),
        &c_dot,
        &a_vec.cross_product(&b_vec),
    );
    let time = a_vec.dot_product(&b_vec.cross_product(&c_vec));
    let w_vec = w_vec / time;

    let w_with_1 = hailstone.velocity - w_vec.clone();
    let w_with_2 = hailstone_second.velocity - w_vec;
    let w_cross = w_with_1.cross_product(&w_with_2);

    let e = w_cross.dot_product(&hailstone_second.position.cross_product(&w_with_2));
    let f = w_cross.dot_product(&hailstone.position.cross_product(&w_with_1));
    let g = hailstone.position.dot_product(&w_cross);
    let s = w_cross.dot_product(&w_cross);

    let rock = BigIntVec3D::from_linear(&e, &w_with_1, &-f, &w_with_2, &g, &w_cross);
    (rock.sum() / s).to_usize().unwrap()
}

fn find_plane(hailstone: &Hailstone, other: &Hailstone) -> (Vec3D, BigInt) {
    let center = hailstone.position - other.position;
    let new_vel = hailstone.velocity - other.velocity;
    let velocity_vector = hailstone.velocity.cross_product(&other.velocity);
    let normal = center.cross_product(&new_vel);
    let dot = center.dot_product(&velocity_vector);
    (normal, dot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_to_string("inputs/day-24.example.txt").unwrap();
        let problem: Problem = input.parse().unwrap();
        // this specific example has different bounds than the regular problem
        let problem = Problem {
            min_bound: 7,
            max_bound: 27,
            ..problem
        };
        assert_eq!(find_future_intersections(&problem), 2);
    }
    #[test]
    fn test_part2() {
        let input = read_to_string("inputs/day-24.example.txt").unwrap();
        assert_eq!(part2(&input), 47);
    }
    #[test]
    fn test_parallel_hailstones() {
        let problem = Problem {
            min_bound: 7,
            max_bound: 27,
            hailstones: vec![
                // these two `Hailstone` objects are parallel to each other and will never intersect.
                Hailstone {
                    position: Vec3D {
                        x: 18,
                        y: 19,
                        z: 22,
                    },
                    velocity: Vec3D {
                        x: -1,
                        y: -1,
                        z: -2,
                    },
                    ..Default::default()
                },
                Hailstone {
                    position: Vec3D {
                        x: 20,
                        y: 25,
                        z: 34,
                    },
                    velocity: Vec3D {
                        x: -2,
                        y: -2,
                        z: -4,
                    },
                    ..Default::default()
                },
            ],
        };
        assert_eq!(find_future_intersections(&problem), 0);
    }
    #[test]
    fn test_problem_parse() {
        let input = read_to_string("inputs/day-24.example.txt").unwrap();
        let problem: Problem = input.parse().unwrap();
        assert_eq!(problem.hailstones.len(), 5);
        assert_eq!(
            problem.hailstones,
            [
                Hailstone {
                    position: Vec3D {
                        x: 19,
                        y: 13,
                        z: 30
                    },
                    velocity: Vec3D { x: -2, y: 1, z: -2 },
                    ..Default::default()
                },
                Hailstone {
                    position: Vec3D {
                        x: 18,
                        y: 19,
                        z: 22
                    },
                    velocity: Vec3D {
                        x: -1,
                        y: -1,
                        z: -2
                    },
                    ..Default::default()
                },
                Hailstone {
                    position: Vec3D {
                        x: 20,
                        y: 25,
                        z: 34
                    },
                    velocity: Vec3D {
                        x: -2,
                        y: -2,
                        z: -4
                    },
                    ..Default::default()
                },
                Hailstone {
                    position: Vec3D {
                        x: 12,
                        y: 31,
                        z: 28
                    },
                    velocity: Vec3D {
                        x: -1,
                        y: -2,
                        z: -1
                    },
                    ..Default::default()
                },
                Hailstone {
                    position: Vec3D {
                        x: 20,
                        y: 19,
                        z: 15
                    },
                    velocity: Vec3D { x: 1, y: -5, z: -3 },
                    ..Default::default()
                }
            ]
        )
    }
}
