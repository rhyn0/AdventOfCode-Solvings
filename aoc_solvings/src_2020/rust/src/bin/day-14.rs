use itertools::Itertools;
use std::{collections::HashMap, env, fmt::Debug};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    let problem = ProblemStatement::parse_input(&input_str);
    println!("Part 1: {}", &problem.part1());
    println!("Part 2: {}", problem.part2());
}

fn get_input(input_file: &String) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

#[derive(Debug, Default)]
pub struct ProblemStatement {
    instructions: Vec<Instruction>,
}

#[derive(PartialEq)]
enum Instruction {
    SetMask(Mask),
    RegisterAssign { addr: usize, value: usize },
}

#[derive(Copy, Clone, PartialEq, Default)]
struct Mask {
    set: usize,
    clear: usize,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SetMask(m) => write!(f, "mask = {m:?}"),
            Self::RegisterAssign { addr, value } => write!(f, "mem[{addr}] = {value}"),
        }
    }
}

impl Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..Self::MASK_LEN {
            let pos = 1 << (Self::MASK_LEN - i);
            write!(
                f,
                "{}",
                if self.set & pos != 0 {
                    "1"
                } else if self.clear & pos != 0 {
                    "0"
                } else {
                    "X"
                }
            )?;
        }
        Ok(())
    }
}

impl Mask {
    const MASK_LEN: usize = 36;
    pub const fn apply(&self, value: usize) -> usize {
        (value | self.set) & !self.clear
    }
    pub fn x_positions(&self) -> impl Iterator<Item = usize> + '_ {
        // bitmasks are always 36 bits
        // if either set or clear is "set" then there can't be an X
        (0..36_usize).filter(move |i| ((1 << i) & (self.set | self.clear)) == 0)
    }
    pub fn apply_x(&self, x_pos: &[usize]) -> Self {
        // mutate existing mask to follow Floating X rule
        let mut new_self = *self;
        for pos in self.x_positions() {
            let addl_mask = 1_usize << pos;
            if x_pos.contains(&pos) {
                // if this mutation calls for setting this position
                new_self.set |= addl_mask;
            } else {
                // otherwise clear it
                new_self.clear |= addl_mask;
            }
        }
        new_self
    }
    pub fn or(&self, addr: usize) -> Self {
        let mut new_self = *self;
        let set_positions = self.set | self.clear;

        for i in 0..Self::MASK_LEN {
            let pos_check = 1 << i;
            if pos_check & set_positions == 0 {
                // can only happen if this value is X
                // so leave it alone
            } else if pos_check & addr != 0 {
                // incoming value is set at this position
                // existing mask is already 0 or 1, so just preserve this position
                new_self.set |= pos_check;
                new_self.clear &= !pos_check;
            }
        }
        new_self
    }
    pub fn each_combination(&self) -> impl Iterator<Item = Self> + '_ {
        self.x_positions()
            .powerset()
            .map(move |xes| self.apply_x(&xes))
    }
    pub fn each_binary_set_mask(&self) -> impl Iterator<Item = usize> + '_ {
        self.each_combination().map(|m| m.set)
    }
}

impl ProblemStatement {
    fn parse_input(input: &str) -> Self {
        peg::parser! {
            grammar mem_parser() for str{
                pub rule root(p: &mut ProblemStatement)
                    = (line(p) whitespace()*)* ![_]
                rule line(p: &mut ProblemStatement)
                    = i:instruction() {p.instructions.push(i)}
                rule instruction() -> Instruction
                    = set_mask() / assign()

                rule set_mask() -> Instruction
                    = "mask = " e:$(['X' | '1' | '0']+) {
                        let mut curr_mask: Mask = Mask::default();
                        for (idx, c) in e.as_bytes().iter().rev().enumerate() {
                            match c {
                                b'1' => curr_mask.set |= 2_usize.pow(idx.try_into().unwrap()),
                                b'0' => curr_mask.clear |= 2_usize.pow(idx.try_into().unwrap()),
                                _ => {},
                            }
                        }
                        Instruction::SetMask(curr_mask)
                    }

                rule assign() -> Instruction
                    = "mem[" addr:number() "] = " value:number() {
                        Instruction::RegisterAssign { addr, value }
                    }


                rule number() -> usize
                    = e:$(['0'..='9']+) { e.parse::<usize>().unwrap() }
                rule whitespace()
                    = [' '| '\t' | '\n' | '\r']
            }
        }
        let mut problem: Self = Self::default();
        mem_parser::root(input, &mut problem).unwrap();
        problem
    }
    fn part1(&self) -> usize {
        let mut registers: HashMap<usize, usize> = HashMap::new();
        let mut mask = Mask::default();

        for instr in &self.instructions {
            match *instr {
                Instruction::RegisterAssign { addr, value } => {
                    registers.insert(addr, mask.apply(value));
                }
                Instruction::SetMask(m) => mask = m,
            }
        }
        registers.values().sum()
    }

    fn part2(&self) -> usize {
        let mut registers: HashMap<usize, usize> = HashMap::new();
        let mut mask = Mask::default();
        for instr in &self.instructions {
            match *instr {
                Instruction::RegisterAssign { addr, value } => {
                    for register in mask.or(addr).each_binary_set_mask() {
                        registers.insert(register, value);
                    }
                }
                Instruction::SetMask(m) => mask = m,
            }
        }
        registers.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "inputs/test-14.txt";

    #[test]
    fn test_part1_example() {
        let problem = ProblemStatement::parse_input(&get_input(&TEST_FILE.to_string()));
        assert_eq!(problem.part1(), 165);
    }

    #[test]
    fn test_part2_example() {
        let problem = ProblemStatement::parse_input(
            "mask = 000000000000000000000000000000X1001X
        mem[42] = 100
        mask = 00000000000000000000000000000000X0XX
        mem[26] = 1",
        );
        assert_eq!(problem.part2(), 208);
    }

    #[test]
    fn test_mask_or() {
        let mask =
            match ProblemStatement::parse_input("mask = 000000000000000000000000000000X1001X")
                .instructions[0]
            {
                Instruction::SetMask(m) => m,
                Instruction::RegisterAssign { .. } => {
                    panic!("Mask instruction got parsed as RegisterAssign.")
                }
            };
        let or_mask = mask.or(42);
        assert_eq!(or_mask.set, 26);
        assert_eq!(or_mask.x_positions().collect_vec().len(), 2);
    }

    #[test]
    fn test_mask_floating_x() {
        // this mask needs to yield the addresses to set
        let mask =
            match ProblemStatement::parse_input("mask = 000000000000000000000000000000X1101X")
                .instructions[0]
            {
                Instruction::SetMask(m) => m,
                Instruction::RegisterAssign { .. } => {
                    panic!("Mask instruction got parsed as RegisterAssign.")
                }
            };
        let expanded_addrs = mask.each_binary_set_mask().collect_vec();
        assert!(expanded_addrs.contains(&26_usize));
        assert!(expanded_addrs.contains(&27_usize));
        assert!(expanded_addrs.contains(&58_usize));
        assert!(expanded_addrs.contains(&59_usize));
    }
}
