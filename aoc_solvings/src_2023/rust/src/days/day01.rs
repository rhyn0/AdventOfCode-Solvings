use crate::etc::{Solution, SolutionPair};
use itertools::Itertools;
use std::fs::read_to_string;

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("./inputs/day-01.txt")).unwrap();
    (
        Solution::from(part01(&input)),
        Solution::from(part02(&input)),
    )
}

/**
 * Return the sum of all number in the input.
 * Numbers are specially formatted to be the first digit and the last digit on a line.
 */
fn part01(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut chars = l.chars().filter_map(|c| c.to_digit(10));
            // unwrap or 0, if a line doesnt contain any digits, the sum is 0
            let first = chars.next().unwrap_or(0);
            // if there is only one digit, the last digit is the same as the first digit
            let last = chars.last().unwrap_or(first);
            // first digit is the tens digit, last digit is the ones digit
            10 * first + last
        })
        .sum::<u32>()
}

/**
 * Return the sum of all number in the input.
 * Numbers are specially formatted to be the first digit and the last digit on a line.
 * But the digits can be the spelled out english words for the digit
 * E.g. "one", "two" "three" "four" etc
 * Lines of input can have any english characters, and any number of digits
 * There is no whitespace on a line besides the newline character
 */
fn part02(input: &str) -> u32 {
    let digits: [(&str, u32); 9] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let line_numbers = input.lines().map(|l| {
        let digit_locs = digits
            .into_iter()
            .flat_map(|(digit_str, d)| l.match_indices(digit_str).map(move |(idx, _)| (idx, d)));
        let chars = l
            .chars()
            .enumerate()
            .filter_map(|(idx, c)| c.to_digit(10).map(|d| (idx, d)))
            .chain(digit_locs)
            .sorted_by_key(|(idx, _d)| *idx)
            .collect_vec();
        let first = chars.first().unwrap().1;
        // if there is only one digit, the last digit is the same as the first digit
        let last = chars.last().unwrap_or(&(0, first)).1;
        // first digit is the tens digit, last digit is the ones digit
        10 * first + last
    });
    line_numbers.sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_day01_p1() {
        // ignore part 2 here, original input does not have part 2 like inputs
        let (p1, _) = solve(Some("./inputs/day-01.example.txt"));
        assert_eq!(p1, Solution::from(142_u32));
    }
    #[test]
    fn test_day01_p2() {
        let (_, p2) = solve(Some("./inputs/day-01.example2.txt"));
        assert_eq!(p2, Solution::from(281_u32));
    }
    #[test]
    fn test_day01_p2_repeat_digits() {
        let result = part02("five6five");
        assert_eq!(result, 55);
    }
}
