use crate::etc::{Solution, SolutionPair};

use std::{fs::read_to_string, iter::once, str::FromStr};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("inputs/day-15.txt")).unwrap();

    (Solution::from(part1(&input)), Solution::from(part2(&input)))
}

fn part1(input: &str) -> u32 {
    // given input string has comma seperate values
    input
        .trim_end()
        .split(',')
        .map(|x| {
            HashString::from_str(x)
                .map(HashString::apply_algorithm)
                .unwrap()
        })
        .sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HashString {
    value: String,
}

impl FromStr for HashString {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.to_string();
        if value.is_ascii() {
            Ok(Self { value })
        } else {
            Err(once("String is not ascii: ".to_string())
                .chain(once(
                    value.chars().filter(|c| !c.is_ascii()).collect::<String>(),
                ))
                .collect())
        }
    }
}

impl HashString {
    /// Hash the string of `value` according to algorithm
    ///
    /// Algorithm is to go character by character
    /// For each character, take ASCII value, add to running total, multiply by 17, then take modulo of 256
    fn apply_algorithm(self) -> u32 {
        self.value
            .chars()
            .map(|c| c as u32)
            .fold(0, |acc, c| (acc + c) * 17 % 256)
    }
}

fn part2(input: &str) -> u32 {
    let mut map = HashHashMap::new();
    let operations = input
        .trim_end()
        .split(',')
        .map(|x| InitOperations::from_str(x).unwrap())
        .collect::<Vec<_>>();

    map = operations.iter().fold(map, HashHashMap::apply_operation);

    map.boxes.iter().map(LightBox::focusing_power).sum::<u32>()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HashHashMap {
    boxes: Vec<LightBox>,
}
impl Default for HashHashMap {
    fn default() -> Self {
        Self::new()
    }
}
impl HashHashMap {
    fn new() -> Self {
        Self {
            boxes: (0..256).map(LightBox::new).collect(),
        }
    }
    fn apply_operation(self, operation: &InitOperations) -> Self {
        match operation {
            InitOperations::Insert(lens) => {
                let mut new = self;
                let box_ = new
                    .boxes
                    .get_mut(lens.label.clone().apply_algorithm() as usize)
                    .unwrap();
                box_.insert(lens.clone());
                new
            }
            InitOperations::Remove(label) => {
                let mut new = self;
                let lightbox = new
                    .boxes
                    .get_mut(label.clone().apply_algorithm() as usize)
                    .unwrap();
                lightbox.remove(label);
                new
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LightBox {
    id: u32,
    lenses: Vec<LightLens>,
}

impl LightBox {
    const fn new(id: u32) -> Self {
        Self {
            id,
            lenses: Vec::new(),
        }
    }
    fn focusing_power(&self) -> u32 {
        self.lenses
            .iter()
            .enumerate()
            .map(|(idx, lens)| (self.id + 1) * u32::try_from(idx + 1).unwrap() * lens.focal_power)
            .sum()
    }
    fn insert(&mut self, lens: LightLens) {
        if let Some(idx) = self.lenses.iter().position(|x| x.label == lens.label) {
            self.lenses[idx] = lens;
        } else {
            self.lenses.push(lens);
        }
    }
    fn remove(&mut self, label: &HashString) {
        if let Some(idx) = self.lenses.iter().position(|x| x.label == *label) {
            self.lenses.remove(idx);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LightLens {
    label: HashString,
    focal_power: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum InitOperations {
    Insert(LightLens),
    Remove(HashString),
}

impl FromStr for InitOperations {
    type Err = String;

    /// Parse a string into an `InitOperations`
    ///
    /// Format is:
    ///     insert: `LABEL=FOCAL_POWER`
    ///     remove: `LABEL-`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('=');
        let label = HashString::from_str(split.next().unwrap()).unwrap();
        if let Some(focal_power) = split.next().map(|x| x.parse::<u32>().unwrap()) {
            Ok(Self::Insert(LightLens { label, focal_power }))
        } else {
            match s.chars().last() {
                // remove trailing '-'
                Some('-') => Ok(Self::Remove(HashString::from_str(&s[..s.len() - 1])?)),
                _ => Err(format!("Invalid InitOperations string: {s}")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_to_string("./inputs/day-15.example.txt").unwrap();
        assert_eq!(part1(&input), 1320)
    }
    #[test]
    fn test_part2() {
        let input = read_to_string("./inputs/day-15.example.txt").unwrap();
        assert_eq!(part2(&input), 145)
    }
    #[test]
    fn test_operation_str_parse() {
        let input_1 = "cm-".to_owned();
        let input_2 = "cm=1".to_owned();

        assert_eq!(
            InitOperations::from_str(&input_1).unwrap(),
            InitOperations::Remove(HashString::from_str("cm").unwrap())
        );
        assert_eq!(
            InitOperations::from_str(&input_2).unwrap(),
            InitOperations::Insert(LightLens {
                label: HashString::from_str("cm").unwrap(),
                focal_power: 1
            })
        );
    }
    #[test]
    fn test_lightbox_init() {
        let lb = LightBox::new(1);
        assert_eq!(lb.id, 1);
        assert_eq!(lb.lenses.len(), 0);
    }
    #[test]
    fn test_emtpy_lightbox_power() {
        let lb = LightBox::new(1);
        assert_eq!(lb.focusing_power(), 0);
    }
    #[test]
    fn test_lightbox_insert() {
        let mut lb = LightBox::new(1);
        let lens = LightLens {
            label: HashString::from_str("cm").unwrap(),
            focal_power: 1,
        };
        lb.insert(lens);

        assert_eq!(lb.lenses.len(), 1);
        assert_eq!(lb.lenses[0].label, HashString::from_str("cm").unwrap());
        assert_eq!(lb.lenses[0].focal_power, 1);
        assert_eq!(lb.focusing_power(), 2);
    }
    #[test]
    fn test_lightbox_remove() {
        let mut lb = LightBox::new(1);
        let lens = LightLens {
            label: HashString::from_str("cm").unwrap(),
            focal_power: 1,
        };
        lb.insert(lens.clone());
        lb.remove(&lens.label);

        assert_eq!(lb.lenses.len(), 0);
        assert_eq!(lb.focusing_power(), 0);
    }
}
