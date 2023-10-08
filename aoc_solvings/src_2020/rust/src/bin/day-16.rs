use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    env,
    fmt::Debug,
    ops::RangeInclusive,
};

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

#[derive(Debug, Default, Clone)]
struct Ticket {
    values: Vec<usize>,
}

#[derive(Debug, Default, Clone)]
pub struct ProblemStatement {
    rules: HashMap<String, Vec<RangeInclusive<usize>>>,
    my_ticket: Ticket,
    other_tickets: Vec<Ticket>,
}

impl ProblemStatement {
    /// # Panics
    /// If file is empty
    #[must_use]
    pub fn parse_input(input: &str) -> Self {
        // there are 3 sections, but 2 types of section
        // a rule section parser and a ticket parser are needed
        peg::parser! {
            grammar ticket_parser() for str {
                pub rule ticket_rules(p: &mut ProblemStatement)
                    = (field_rule(p)  whitespace()*)+
                pub rule my_ticket(p: &mut ProblemStatement)
                    = "your ticket:\n" nums:ticket_nums() {
                        p.my_ticket = Ticket {values: nums};
                    }
                pub rule nearby_tickets(p: &mut ProblemStatement)
                    = "nearby tickets:\n" tickets:ticket_nums() ** "\n" ![_] {
                        p.other_tickets.extend(tickets.iter().map(|t| Ticket{values: t.clone()}));
                    }
                rule field_rule(p: &mut ProblemStatement)
                    = n:field_name() ": " e:field_range() {
                        p.rules.insert(n, e);
                    }
                rule field_range() -> Vec<RangeInclusive<usize>>
                    = n1:num_range() " or " n2:num_range() {
                        vec![n1, n2]
                    }
                rule ticket_nums() -> Vec<usize>
                    = nums:(number() ++ ",") {
                        nums
                    }
                rule num_range() -> RangeInclusive<usize>
                    = start:number() "-" end:number() {
                        RangeInclusive::new(start, end)
                    }
                rule field_name() -> String
                    = e:$(['a'..='z' | 'A'..='Z' | ' ']+) { e.to_string() }
                rule number() -> usize
                    = e:$(['0'..='9']+) { e.parse::<usize>().unwrap() }
                rule whitespace()
                    = [' ' | '\t' | '\n' | '\r']
            }
        }
        let mut problem = Self::default();
        let mut section_iter = input.split("\n\n");
        ticket_parser::ticket_rules(section_iter.next().unwrap(), &mut problem).unwrap();
        ticket_parser::my_ticket(section_iter.next().unwrap().trim_end(), &mut problem).unwrap();
        ticket_parser::nearby_tickets(section_iter.next().unwrap().trim_end(), &mut problem)
            .unwrap();

        problem
    }

    #[must_use]
    pub fn part1(&self) -> usize {
        self.other_tickets
            .iter()
            .flat_map(|t| t.values.clone())
            .filter(|num| {
                !self
                    .rules
                    .values()
                    .flatten()
                    .any(|range| range.contains(num))
            })
            .sum()
    }

    fn is_valid_ticket(&self, t: &Ticket) -> bool {
        t.values
            .iter()
            .all(|x| self.rules.values().flatten().any(|range| range.contains(x)))
    }

    /// # Panics
    /// When a ticket the number of fields in nearby tickets is not uniform.
    #[must_use]
    pub fn part2(&self) -> usize {
        let valid_tickets = self
            .other_tickets
            .iter()
            .filter(|&t| self.is_valid_ticket(t))
            .collect_vec();

        let mut rules_poss_fields: HashMap<_, HashSet<usize>> = self
            .rules
            .iter()
            .map(|(k, v)| {
                (
                    k,
                    (0..self.my_ticket.values.len())
                        .filter(|&idx| {
                            valid_tickets
                                .iter()
                                .map(|&t| *t.values.get(idx).unwrap())
                                .filter(|x| v.iter().any(|r| r.contains(x)))
                                .count()
                                == valid_tickets.len()
                        })
                        .collect(),
                )
            })
            .collect();
        let mut ticket_idx_to_field_map = vec![""; self.rules.len()];
        for _ in 0..ticket_idx_to_field_map.len() {
            let (&field_name, possible_idxs) = rules_poss_fields
                .iter()
                .find(|(_, poss)| poss.len() == 1)
                .unwrap();
            let idx = *possible_idxs.iter().next().unwrap();
            ticket_idx_to_field_map[idx] = field_name;
            for poss in rules_poss_fields.values_mut() {
                poss.remove(&idx);
            }
            rules_poss_fields.remove(field_name);
        }
        self.my_ticket
            .values
            .iter()
            .enumerate()
            .map(|(i, v)| (ticket_idx_to_field_map[i], v))
            .filter_map(|(field, v)| {
                if field.starts_with("departure") {
                    Some(v)
                } else {
                    None
                }
            })
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "inputs/test-16.txt";

    #[test]
    fn test_part1_example() {
        let problem = ProblemStatement::parse_input(&get_input(&TEST_FILE.to_string()));
        assert_eq!(problem.part1(), 71);
    }

    #[test]
    fn test_part2_example() {
        let problem = ProblemStatement::parse_input(&get_input(&TEST_FILE.to_string()));
        assert_eq!(problem.part2(), 1);
    }
}
