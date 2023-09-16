use std::{collections::HashMap, env, fmt::Debug};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    let mut problem = ProblemStatement::parse_input(&input_str);
    println!("Part 1: {}", problem.clone().part1());
    println!("Part 2: {}", problem.part2());
}

fn get_input(input_file: &String) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

#[derive(Debug, Default, Clone)]
pub struct ProblemStatement {
    last_number: i64,
    prev_numbers: HashMap<i64, (i64, i64)>,
    current_turn: i64,
}

impl ProblemStatement {
    /// # Panics
    /// If file is empty
    #[must_use]
    pub fn parse_input(input: &str) -> Self {
        // input is on first line
        let input_iter = input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .enumerate()
            .map(|(idx, c)| {
                (
                    c.parse::<i64>().unwrap(),
                    (i64::try_from(idx).unwrap() + 1, 0),
                )
            });
        let (last_number, last_turn) = input_iter.clone().last().unwrap();
        Self {
            prev_numbers: input_iter.collect(),
            // current turn is plus one from number of numbers said
            current_turn: last_turn.0 + 1,
            last_number,
        }
    }
    fn count_numbers(&mut self, limit: Option<i64>) -> i64 {
        let limit = limit.unwrap_or(2020);
        let mut prev_num = self.last_number;
        for curr_turn in self.current_turn..=limit {
            let last_spoken_tup = self.prev_numbers.get(&prev_num).unwrap_or(&(0, 0));
            let new_num = match last_spoken_tup {
                (_, 0) => 0,
                (recent, before_recent) => recent - before_recent,
            };
            let new_num_last_spoken = self.prev_numbers.get(&new_num).unwrap_or(&(0, 0));
            let spoken_tup = match new_num_last_spoken {
                (0, 0) => (curr_turn, 0),
                (x, _) => (curr_turn, *x),
            };
            self.prev_numbers.insert(new_num, spoken_tup);
            prev_num = new_num;
        }
        prev_num
    }
    pub fn part1(&mut self) -> i64 {
        self.count_numbers(None)
    }

    pub fn part2(&mut self) -> i64 {
        self.count_numbers(Some(30_000_000))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "inputs/test-15.txt";

    #[test]
    fn test_steps_example() {
        let mut problem = ProblemStatement::parse_input(&get_input(&TEST_FILE.to_string()));
        assert_eq!(problem.count_numbers(Some(10)), 0);
    }

    #[test]
    fn test_extra_part1_examples() {
        assert_eq!(ProblemStatement::parse_input("1,3,2").part1(), 1);
        assert_eq!(ProblemStatement::parse_input("2,1,3").part1(), 10);
        assert_eq!(ProblemStatement::parse_input("1,2,3").part1(), 27);
        assert_eq!(ProblemStatement::parse_input("2,3,1").part1(), 78);
        assert_eq!(ProblemStatement::parse_input("3,2,1").part1(), 438);
        assert_eq!(ProblemStatement::parse_input("3,1,2").part1(), 1836);
    }

    #[test]
    fn test_part1_example() {
        let mut problem = ProblemStatement::parse_input(&get_input(&TEST_FILE.to_string()));
        assert_eq!(problem.part1(), 436);
    }

    #[test]
    fn test_extra_part2_examples() {
        // this is going to take awhile
        assert_eq!(ProblemStatement::parse_input("1,3,2").part2(), 2578);
        assert_eq!(ProblemStatement::parse_input("2,1,3").part2(), 3544142);
        assert_eq!(ProblemStatement::parse_input("1,2,3").part2(), 261214);
        assert_eq!(ProblemStatement::parse_input("2,3,1").part2(), 6895259);
        assert_eq!(ProblemStatement::parse_input("3,2,1").part2(), 18);
        assert_eq!(ProblemStatement::parse_input("3,1,2").part2(), 362);
    }

    #[test]
    fn test_part2_example() {
        let mut problem = ProblemStatement::parse_input(&get_input(&TEST_FILE.to_string()));
        assert_eq!(problem.part2(), 175594);
    }
}
