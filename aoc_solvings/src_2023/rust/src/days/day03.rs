use itertools::Itertools;

use crate::etc::{Solution, SolutionPair};
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};
pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input_str = read_to_string(input_path.unwrap_or("./inputs/day-03.txt")).unwrap();
    (
        Solution::from(part1(&input_str)),
        Solution::from(part2(&input_str)),
    )
}

fn part1(input: &str) -> u32 {
    // find the locations of the symbols
    let symbols = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| is_symbol(*c))
                .map(|(x, _)| (x, y))
                .collect_vec()
        })
        .collect::<HashSet<(usize, usize)>>();
    // build the numbers and their locations (start, end)
    let numbers = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| find_all_numbers_in_schematic(y, line))
        .filter_map(|part_number| {
            let mut part_neighbor_symbol = (part_number.x_start..=part_number.x_end)
                .map(move |x| (x, part_number.y))
                .flat_map(|(x, y)| all_neighbors(x, y).into_iter().map(move |(x, y)| (x, y)));
            if part_neighbor_symbol.any(|(x, y)| symbols.contains(&(x, y))) {
                Some(part_number.value)
            } else {
                None
            }
        });
    numbers.sum()
}

fn find_all_numbers_in_schematic(y: usize, line: &str) -> Vec<PartNumber> {
    // need to find the numbers in the line, concatenate adjacent digits
    // then parse them into numbers
    let mut numbers = vec![];
    let mut x_start = 0;
    let mut x_end = 0;
    let mut value = 0;
    for (x, c) in line.chars().enumerate() {
        if c.is_ascii_digit() {
            if x_start == 0 {
                x_start = x;
            }
            x_end = x;
            value = value * 10 + c.to_digit(10).unwrap();
        } else if value != 0 {
            numbers.push(PartNumber {
                x_start,
                x_end,
                y,
                value,
            });
            x_start = 0;
            x_end = 0;
            value = 0;
        }
    }
    if value != 0 {
        numbers.push(PartNumber {
            x_start,
            x_end,
            y,
            value,
        });
    }
    numbers
}

/**
 * Return all neighbors of a given point
 */
fn all_neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    // x and y can be 0, so we need to use checked_sub
    vec![
        (x.checked_sub(1).unwrap_or(x), y.checked_sub(1).unwrap_or(y)),
        (x, y.checked_sub(1).unwrap_or(y)),
        (x + 1, y.checked_sub(1).unwrap_or(y)),
        (x.checked_sub(1).unwrap_or(x), y),
        // (x, y), // self
        (x + 1, y),
        (x.checked_sub(1).unwrap_or(x), y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

fn is_symbol(c: char) -> bool {
    (!c.is_numeric()) && c != '.'
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PartNumber {
    x_start: usize,
    x_end: usize,
    y: usize,
    value: u32,
}

fn part2(input: &str) -> u32 {
    // find all the symbols
    // store the location of a symbol against a previously found numbers next to it
    let mut symbols = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| is_symbol(*c))
                .map(|(x, _)| ((x, y), vec![]))
                .collect_vec()
        })
        .collect::<HashMap<(usize, usize), Vec<u32>>>();
    // build the numbers and their locations (start, end)
    for part_number in input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| find_all_numbers_in_schematic(y, line))
    {
        // now numbers that are next to a symbol and store in the symbols map
        // the numbers that are next to it
        // not short circuiting on each number
        if let Some((x, y)) = (part_number.x_start..=part_number.x_end)
            .map(move |x| (x, part_number.y))
            .flat_map(|(x, y)| all_neighbors(x, y).into_iter().map(move |(x, y)| (x, y)))
            .find(|&(x, y)| symbols.contains_key(&(x, y)))
        {
            symbols.get_mut(&(x, y)).unwrap().push(part_number.value);
        }
    }
    let gear_ratios = symbols
        .values()
        .filter_map(|v| {
            if v.len() == 2 {
                Some(v.iter().product::<u32>())
            } else {
                None
            }
        })
        .collect_vec();
    gear_ratios.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let input_str = read_to_string("./inputs/day-03.example.txt").unwrap();
        assert_eq!(part1(&input_str), 4361);
    }

    #[test]
    fn test_find_numbers() {
        let input_str = "10...20";
        let numbers = find_all_numbers_in_schematic(0, input_str);
        assert_eq!(numbers.len(), 2);
        assert_eq!(numbers[0].value, 10);
        assert_eq!(numbers[1].value, 20);
    }

    #[test]
    fn test_part2() {
        let input_str = read_to_string("./inputs/day-03.example.txt").unwrap();
        assert_eq!(part2(&input_str), 467835);
    }
}
