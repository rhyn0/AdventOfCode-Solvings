use crate::etc::{Solution, SolutionPair};

use itertools::Itertools;
use std::{fs::read_to_string, str::FromStr};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("inputs/day-09.txt")).unwrap();
    (Solution::from(part1(&input)), Solution::from(part2(&input)))
}

fn part1(input: &str) -> i64 {
    // find the sum of the next numbers in the readings
    let problem = Problem::from_str(input).unwrap();
    problem
        .oasis_readings
        .iter()
        .map(|x| x.clone().find_next_element())
        .sum()
}

#[derive(Debug, Clone)]
pub struct Problem {
    oasis_readings: Vec<OasisReading>,
}

impl FromStr for Problem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let oasis_readings = s
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let values = line
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect();
                OasisReading { values }
            })
            .collect();
        Ok(Self { oasis_readings })
    }
}

#[derive(Debug, Clone)]
struct OasisReading {
    values: Vec<i64>,
}

impl OasisReading {
    /// `OasisReading` is a line of numbers that represent a sequence of numbers following some pattern
    /// We want to find the next number in the sequence
    /// Which can be found by first finding the differences between adjacent numbers
    /// until the differences are all the same
    /// then sum up that difference value with the
    /// last number of all the difference sequences and the last number of the original array
    fn differences(&self) -> Self {
        // return the differences between adjacent elements in self.values
        Self {
            values: self
                .values
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect(),
        }
    }

    fn is_constant(&self) -> bool {
        // return true if all the elements in self.values are the same
        self.values.iter().all(|&x| x == self.values[0])
    }

    fn last(&self) -> Option<i64> {
        self.values.last().copied()
    }

    fn first(&self) -> Option<i64> {
        self.values.first().copied()
    }

    fn find_next_element(self) -> i64 {
        // find the next element in the sequence
        // by finding the differences between adjacent elements
        // until the differences are all the same
        // then sum up that difference value with the
        // last number of all the difference sequences and the last number of the original array
        let mut differences = self.differences();
        let mut next_element = self.last().unwrap() + differences.last().unwrap();

        while !differences.is_constant() {
            differences = differences.differences();
            next_element += differences.last().unwrap();
        }
        next_element
    }

    fn find_prev_element(&self) -> i64 {
        // find the previous element in the sequence
        // by finding the differences between adjacent elements
        // until the differences are all the same
        // then subtract up that difference value with the
        // first number of all the difference sequences and the first number of the original array
        let mut differences = self.differences();
        let mut prev_element = self.first().unwrap() - differences.first().unwrap();
        // since the operation has a negative distributive property (a - b = a + (-b))
        // we need to count the number of subtractions done and whether it is actually a subtracted subtraction
        // or an added subtraction. Meaning we need to multiply by -1 on every other subtraction
        let mut steps = 1;
        while !differences.is_constant() {
            differences = differences.differences();
            prev_element -= differences.first().unwrap() * (-1_i64).pow(steps);
            steps += 1;
        }
        prev_element
    }
}

fn part2(input: &str) -> i64 {
    // find the previous element of each sequence of readings and return sum
    let problem = Problem::from_str(input).unwrap();
    problem
        .oasis_readings
        .iter()
        .map(OasisReading::find_prev_element)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = read_to_string("inputs/day-09.example.txt").unwrap();
        assert_eq!(part1(&input), 114);
    }

    #[test]
    fn test_parse_from_str() {
        let input = indoc!(
            "
            0 0 0 0 0
            1 2 3 4 5



        "
        );
        let problem = Problem::from_str(input).unwrap();
        assert_eq!(problem.oasis_readings.len(), 2);
    }
    #[test]
    fn test_part2() {
        let input = read_to_string("inputs/day-09.example.txt").unwrap();
        assert_eq!(part2(&input), 2);
    }
    #[test]
    fn test_backwards_element() {
        let values = vec![1, 2, 3, 4, 5];
        // the difference in each element is 1, so we should receive a 0
        let oasis_reading = OasisReading { values };
        assert_eq!(oasis_reading.find_prev_element(), 0);
    }
    #[test]
    fn test_backwards_element2() {
        // pattern becomes more complex
        //  1 1 2 4 7 11
        //   0 1 2 3 4
        //    1 1 1 1
        // so predict the previous is 1 - 1, then 1 - 0 to get 1
        let values = vec![1, 2, 4, 7, 11];
        // the difference increases by one each element, so we should get a difference of 0
        // which means the prev element is 1
        let oasis_reading = OasisReading { values };
        assert_eq!(oasis_reading.find_prev_element(), 1);
    }
}
