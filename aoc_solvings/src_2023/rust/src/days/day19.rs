use crate::etc::{Solution, SolutionPair};

use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    ops::RangeInclusive,
    str::FromStr,
};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("inputs/day-19.txt")).unwrap();

    (Solution::from(part1(&input)), Solution::from(part2(&input)))
}

fn part1(input: &str) -> usize {
    let problem = Problem::from_str(input).unwrap();
    let accepted_parts = problem.apply_sorting_workflow();
    accepted_parts.iter().map(Part::attribute_sum).sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl FromStr for Problem {
    type Err = peg::error::ParseError<peg::str::LineCol>;
    #[allow(clippy::ignored_unit_patterns)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        peg::parser! {
            grammar workflow_parser() for str {
                pub rule problem() -> Problem
                    = workflows:workflow() ++ "\n" newline() parts:part() ++ "\n" newline()? ![_] { Problem {
                        workflows: workflows.iter().map(std::borrow::ToOwned::to_owned).collect(),
                        parts,
                    } }

                rule part() -> Part
                    = "{x=" extreme:number() "," "m=" musical:number() "," "a=" aerodynamic:number() "," "s=" shiny:number() "}" {
                        Part { extreme, musical, aerodynamic, shiny }
                    }

                rule workflow() -> (String, Workflow)
                    = name:ascii_text() "{" rules:workflow_rule() ++ "," "}" { (name.clone(), Workflow { rules, name }) }

                rule workflow_rule() -> Rule
                    = conditional_rule() / default_rule()

                rule conditional_rule() -> Rule
                    = attribute:attribute_comparison() value:number() ":" action:part_action() { Rule { attribute: Some(attribute), value, action } }

                rule default_rule() -> Rule
                    = action:part_action() {Rule { attribute: None, value: 0, action }}

                rule part_action() -> PartAction
                    = name:ascii_text() {
                        match name.as_str() {
                            "A" => PartAction::Accept,
                            "R" => PartAction::Reject,
                            _ => PartAction::Jump(name)
                        }
                    }

                rule attribute_comparison() -> PartAttribute
                    = text:['a' | 'x' | 's' | 'm'] comp:comparison() {
                        match text {
                            'a' => PartAttribute::Aerodynamic(comp),
                            'x' => PartAttribute::Extreme(comp),
                            's' => PartAttribute::Shiny(comp),
                            'm' => PartAttribute::Musical(comp),
                            _ => unreachable!(),
                        }
                    }

                rule comparison() -> Ordering
                    = "<" { Ordering::Less }
                    / ">" { Ordering::Greater }

                rule number() -> usize
                    = n:$(['0'..='9']+) { n.parse().unwrap() }

                rule ascii_text() -> String
                    = a:['A'..='Z' | 'a'..='z']+ { a.iter().collect::<String>() }

                rule __()
                    = [' '| '\t']+

                rule newline()
                    = ['\r' | '\n']+

            }
        }
        match workflow_parser::problem(s) {
            Ok(problem) => Ok(problem),
            Err(err) => Err(err),
        }
    }
}

impl Problem {
    /// Apply the workflows to sort the parts
    ///
    /// Return only the accepted parts
    fn apply_sorting_workflow(&self) -> Vec<Part> {
        self.parts
            .iter()
            .filter_map(|part| {
                let start_workflow_name = "in".to_string();
                let mut current_workflow = self.workflows.get(&start_workflow_name).unwrap();
                loop {
                    // once we make decision (accept or reject) we break
                    match current_workflow.sort_part(part) {
                        PartAction::Accept => return Some(part.clone()),
                        PartAction::Reject => return None,
                        PartAction::Jump(name) => {
                            current_workflow = self.workflows.get(&name).unwrap();
                        }
                    };
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Workflow {
    rules: Vec<Rule>,
    name: String,
}

impl Workflow {
    fn sort_part(&self, part: &Part) -> PartAction {
        self.rules
            .iter()
            .find_map(|rule| {
                // only the default rule can have no attribute
                if rule.attribute.is_none() {
                    return Some(rule.action.clone());
                }
                let attribute = rule.attribute.as_ref().unwrap();

                match attribute {
                    PartAttribute::Extreme(comp) => {
                        if part.extreme.cmp(&rule.value) == *comp {
                            return Some(rule.action.clone());
                        }
                    }
                    PartAttribute::Musical(comp) => {
                        if part.musical.cmp(&rule.value) == *comp {
                            return Some(rule.action.clone());
                        }
                    }
                    PartAttribute::Aerodynamic(comp) => {
                        if part.aerodynamic.cmp(&rule.value) == *comp {
                            return Some(rule.action.clone());
                        }
                    }
                    PartAttribute::Shiny(comp) => {
                        if part.shiny.cmp(&rule.value) == *comp {
                            return Some(rule.action.clone());
                        }
                    }
                }
                None
            })
            .unwrap()
    }
    fn sort_part_range(&self, range: &PartRange) -> Vec<(PartAction, PartRange)> {
        let mut branches = Vec::new();
        self.rules.iter().fold(range.clone(), |acc, rule| {
            if let Some((affected, remaining)) = rule.split_part_range(&acc) {
                branches.push((rule.action.clone(), affected));
                remaining
            } else {
                branches.push((rule.action.clone(), acc.clone()));
                acc
            }
        });
        branches
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
    attribute: Option<PartAttribute>,
    value: usize,
    action: PartAction,
}

impl Rule {
    /// Split a range according to `Rule` attribute and value
    ///
    /// If a rule doesn't have a attribute, it is because it is a default rule and we don't split the range
    /// The .0 of the tuple is the one affected by this rule, the .1 would continue on the workflow
    fn split_part_range(&self, part_range: &PartRange) -> Option<(PartRange, PartRange)> {
        self.attribute
            .map(|attribute| part_range.split(attribute, self.value))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PartAttribute {
    Extreme(Ordering),     // x
    Musical(Ordering),     // m
    Aerodynamic(Ordering), // a
    Shiny(Ordering),       // s
}

impl PartAttribute {
    const fn unwrap(self) -> Ordering {
        match self {
            Self::Extreme(ord) | Self::Musical(ord) | Self::Aerodynamic(ord) | Self::Shiny(ord) => {
                ord
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PartAction {
    Accept,
    Reject,
    Jump(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Part {
    extreme: usize,
    musical: usize,
    aerodynamic: usize,
    shiny: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PartRange {
    extreme: RangeInclusive<usize>,
    musical: RangeInclusive<usize>,
    aerodynamic: RangeInclusive<usize>,
    shiny: RangeInclusive<usize>,
}

impl Default for PartRange {
    fn default() -> Self {
        let default_range = 1..=4000;
        Self {
            extreme: default_range.clone(),
            musical: default_range.clone(),
            aerodynamic: default_range.clone(),
            shiny: default_range,
        }
    }
}

impl PartRange {
    /// Split a range according to `PartAttribute` and value
    ///
    /// The .0 of the tuple is the one affected by this rule, the .1 would continue on the workflow
    #[allow(clippy::range_minus_one)]
    fn split(&self, attribute: PartAttribute, value: usize) -> (Self, Self) {
        // TODO: don't hardcode this
        let max_range_value = 4000;
        let (matched_range, leftover_range) = match attribute.unwrap() {
            Ordering::Less => (1..=value - 1, value..=max_range_value),
            Ordering::Greater => (value + 1..=max_range_value, 1..=value),
            Ordering::Equal => unreachable!(),
        };
        match attribute {
            // if ordering condition is less than, than the value and all greater are not affected
            PartAttribute::Extreme(_) => (
                Self {
                    extreme: intersect(&self.extreme, &matched_range).unwrap(),
                    ..self.clone()
                },
                Self {
                    extreme: intersect(&self.extreme, &leftover_range).unwrap(),
                    ..self.clone()
                },
            ),
            PartAttribute::Musical(_) => (
                Self {
                    musical: intersect(&self.musical, &matched_range).unwrap(),
                    ..self.clone()
                },
                Self {
                    musical: intersect(&self.musical, &leftover_range).unwrap(),
                    ..self.clone()
                },
            ),
            PartAttribute::Aerodynamic(_) => (
                Self {
                    aerodynamic: intersect(&self.aerodynamic, &matched_range).unwrap(),
                    ..self.clone()
                },
                Self {
                    aerodynamic: intersect(&self.aerodynamic, &leftover_range).unwrap(),
                    ..self.clone()
                },
            ),
            PartAttribute::Shiny(_) => (
                Self {
                    shiny: intersect(&self.shiny, &matched_range).unwrap(),
                    ..self.clone()
                },
                Self {
                    shiny: intersect(&self.shiny, &leftover_range).unwrap(),
                    ..self.clone()
                },
            ),
        }
    }
    fn range_len(range: &RangeInclusive<usize>) -> usize {
        range.end().to_owned() - range.start().to_owned() + 1
    }
    fn num_combinations(&self) -> usize {
        Self::range_len(&self.extreme)
            * Self::range_len(&self.musical)
            * Self::range_len(&self.aerodynamic)
            * Self::range_len(&self.shiny)
    }
}

fn intersect(
    a: &RangeInclusive<usize>,
    b: &RangeInclusive<usize>,
) -> Option<RangeInclusive<usize>> {
    let start = a.start().to_owned().max(b.start().to_owned());
    let end = a.end().to_owned().min(b.end().to_owned());
    if start > end {
        None
    } else {
        Some(start..=end)
    }
}

impl Part {
    const fn attribute_sum(&self) -> usize {
        self.extreme + self.musical + self.aerodynamic + self.shiny
    }
}

fn part2(input: &str) -> usize {
    let problem = Problem::from_str(input).unwrap();

    let part = PartRange::default();

    let mut accepted_part_ranges = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back((PartAction::Jump("in".to_string()), part));

    while let Some((action, range)) = queue.pop_front() {
        match action {
            PartAction::Accept => accepted_part_ranges.push(range),
            PartAction::Reject => {}
            PartAction::Jump(name) => {
                let workflow = problem.workflows.get(&name).unwrap();
                let branches = workflow.sort_part_range(&range);
                queue.extend(branches);
            }
        }
    }
    accepted_part_ranges
        .iter()
        .map(PartRange::num_combinations)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_to_string("inputs/day-19.example.txt").unwrap();
        assert_eq!(part1(&input), 19114);
    }
    #[test]
    fn test_parse_input() {
        let input = read_to_string("inputs/day-19.example.txt").unwrap();
        let problem = Problem::from_str(&input);

        assert!(problem.is_ok());
        let unwrapped = problem.unwrap();
        assert_eq!(unwrapped.workflows.len(), 11);
        assert_eq!(unwrapped.parts.len(), 5);
    }
    #[test]
    fn test_part2() {
        let input = read_to_string("inputs/day-19.example.txt").unwrap();

        assert_eq!(part2(&input), 167409079868000);
    }
}
