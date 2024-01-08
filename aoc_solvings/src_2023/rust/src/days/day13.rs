use crate::etc::{Solution, SolutionPair};

use std::{
    fmt::{Display, Write},
    fs::read_to_string,
    ops::Index,
    str::FromStr,
};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input_str = read_to_string(input_path.unwrap_or("./inputs/day-13.txt")).unwrap();
    (
        Solution::from(part1(&input_str)),
        Solution::from(part2(&input_str)),
    )
}

/// Part 1:
/// Find the score for the given patterns.
/// Score for each pattern is the number of columns to the left of the reflection line
/// plus 100 times the number of rows above the reflection line.
/// Each pattern will only have one reflection line - either horizontal or vertical.
fn part1(input: &str) -> u32 {
    let problem = Problem::from_str(input).unwrap();
    problem
        .patterns
        .into_iter()
        .map(|pattern| pattern.find_reflection(None).score())
        .sum()
}

#[derive(Debug, Clone)]
pub struct Problem {
    patterns: Vec<Pattern>,
}

impl FromStr for Problem {
    type Err = AshParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let patterns = s
            .split("\n\n")
            .filter(|line| !line.trim().is_empty())
            .map(Pattern::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { patterns })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Pattern {
    grid: Vec<AshRow>,
    reflection_horiz: Option<usize>,
    reflection_vert: Option<usize>,
}

impl FromStr for Pattern {
    type Err = AshParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(AshRow::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            grid,
            ..Default::default()
        })
    }
}

impl Pattern {
    fn score(&self) -> u32 {
        let mut score = 0;
        if let Some(reflection_horiz) = self.reflection_horiz {
            score += u32::try_from(reflection_horiz).unwrap() * 100;
        }
        if let Some(reflection_vert) = self.reflection_vert {
            score += u32::try_from(reflection_vert).unwrap();
        }
        score
    }
    fn find_reflection(self, diffs_option: Option<usize>) -> Self {
        Self {
            reflection_vert: Self::_find_reflection_sequence(
                &self._rotate_grid(),
                diffs_option.unwrap_or(0),
            ),
            reflection_horiz: Self::_find_reflection_sequence(
                &self.grid,
                diffs_option.unwrap_or(0),
            ),
            ..self
        }
    }
    /// Return the horizontal reflection point in the grid of this pattern.
    /// If there is no reflection point, return None.
    ///
    /// Reflection point is the row in which all pieces above this point reflect the ones below.
    /// Imagine a mirror being at this point and creating the reflection, hence the name.
    /// Since this is a horizontal reflection line, the analysis is done in each column
    fn _rotate_grid(&self) -> Vec<AshRow> {
        // convert the rows into columns
        let mut cols = vec![AshRow::default(); self.grid[0].len()];
        for row in &self.grid {
            for (col_idx, piece) in row.iter().enumerate() {
                cols[col_idx].pieces.push(*piece);
            }
        }
        cols
    }

    /// Find the reflection point in the given grid, if one exists.
    ///
    /// Reflection point is the point in the sequence where the sequence is mirrored.
    /// Reflection point is not exactly an index as it is between two indices.
    /// If one exists, return the index of the reflection point.
    fn _find_reflection_sequence(grid: &Vec<AshRow>, allowed_diffs: usize) -> Option<usize> {
        // given abccba we should return 3 since the reflection point is after the second c (third character)
        // reflection points can't be before the first character or after the last character
        // so we can skip those
        let num_cols = grid[0].len();
        (1..grid.len()).find(|&split_idx| {
            Self::_min_checkable_indices(split_idx, grid.len()).try_fold(0, |acc, (left, right)| {
                // accumulate the number of differences between the left and right sequences
                // this compares "rows" of the given grid which are on opposite sides
                // of the `split_idx` which is candidate reflection point
                let new_acc = (0..num_cols)
                    .filter(|col_idx| grid[left][*col_idx] != grid[right][*col_idx])
                    .count()
                    + acc;
                if new_acc > allowed_diffs {
                    None
                } else {
                    Some(new_acc)
                }
            }) == Some(allowed_diffs)
        })
    }

    fn _min_checkable_indices(idx: usize, length: usize) -> impl Iterator<Item = (usize, usize)> {
        let max_offset = (length.saturating_sub(idx)).min(idx);
        (0..max_offset).map(move |offset| (idx - offset - 1, idx + offset))
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
struct AshRow {
    pieces: Vec<AshPiece>,
}

impl AshRow {
    fn len(&self) -> usize {
        self.pieces.len()
    }
    const fn iter(&self) -> AshRowIter<'_> {
        AshRowIter {
            values: self,
            curr_idx: 0,
        }
    }
}

impl FromStr for AshRow {
    type Err = AshParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pieces = s
            .chars()
            .map(AshPiece::from_char)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { pieces })
    }
}

impl Index<usize> for AshRow {
    type Output = AshPiece;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pieces[index]
    }
}

impl<'a> IntoIterator for &'a AshRow {
    type Item = &'a AshPiece;

    type IntoIter = AshRowIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

struct AshRowIter<'a> {
    values: &'a AshRow,
    curr_idx: usize,
}

impl ExactSizeIterator for AshRowIter<'_> {
    fn len(&self) -> usize {
        self.values.pieces.len()
    }
}

impl<'a> Iterator for AshRowIter<'a> {
    type Item = &'a AshPiece;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_idx < self.values.pieces.len() {
            let curr_idx = self.curr_idx;
            self.curr_idx += 1;
            Some(&self.values.pieces[curr_idx])
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AshPiece {
    Ash,
    Rock,
}

#[derive(Debug, Clone, Copy)]
pub enum AshParseError {
    InvalidChar(char),
}

impl Display for AshParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::InvalidChar(c) => write!(f, "Invalid character: {c}"),
        }
    }
}

impl Display for AshRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write each piece character by character to buffer, let AshPiece decide how it is displayed per piece
        // we want to write everything in the same row.
        let pieces = self.pieces.iter().fold(String::new(), |mut acc, piece| {
            let _ = write!(acc, "{piece}");
            acc
        });
        write!(f, "{pieces}")
    }
}

impl Display for AshPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::Ash => write!(f, "."),
            Self::Rock => write!(f, "#"),
        }
    }
}

impl AshPiece {
    const fn from_char(s: char) -> Result<Self, AshParseError> {
        match s {
            '.' => Ok(Self::Ash),
            '#' => Ok(Self::Rock),
            _ => Err(AshParseError::InvalidChar(s)),
        }
    }
}

fn part2(input: &str) -> u32 {
    let problem = Problem::from_str(input).unwrap();
    problem
        .patterns
        .into_iter()
        .map(|pattern| {
            // there is allowed to be one difference in the reflection point
            pattern.find_reflection(Some(1)).score()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_to_string("./inputs/day-13.example.txt").unwrap();
        assert_eq!(part1(&input), 405);
    }

    #[test]
    fn test_problem_parse() {
        let input = read_to_string("./inputs/day-13.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        assert_eq!(problem.patterns.len(), 2);
    }
    #[test]
    fn test_horizontal_reflection_find() {
        let input = read_to_string("./inputs/day-13.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        let pattern = problem.patterns[1].clone();
        assert_eq!(
            pattern.clone().find_reflection(None),
            Pattern {
                reflection_horiz: Some(4),
                ..pattern
            }
        );
    }
    #[test]
    fn test_vertical_reflection_find() {
        let input = read_to_string("./inputs/day-13.example.txt").unwrap();
        let problem = Problem::from_str(&input).unwrap();
        let pattern = problem.patterns[0].clone();
        assert_eq!(
            pattern.clone().find_reflection(None),
            Pattern {
                reflection_vert: Some(5),
                ..pattern
            }
        );
    }
    #[test]
    fn test_reflection_find_unit() {
        let row_input = indoc::indoc!(
            "
            #.##..#
            .#..##.
            "
        );
        let row = Pattern::from_str(row_input).unwrap();
        // reflection point should be between the two periods.
        assert_eq!(
            Pattern::_find_reflection_sequence(&row._rotate_grid(), 0),
            Option::Some(5)
        );
    }
    #[test]
    fn test_reflection_on_first() {
        let input = indoc::indoc!(
            "
            #.#...
            #.#...
            ....#.
        "
        );
        let pattern = Pattern::from_str(input).unwrap();
        assert_eq!(
            pattern.clone().find_reflection(None),
            // should be a match between the visible rows
            Pattern {
                reflection_horiz: Some(1),
                ..pattern
            }
        );
    }
}
