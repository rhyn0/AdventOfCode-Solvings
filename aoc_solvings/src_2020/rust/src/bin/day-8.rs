use core::panic;
use std::{collections::HashSet, env};

use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
enum OpAction {
    NoOp,
    Accumulate,
    Jump,
}

#[derive(Debug, Copy, Clone)]
struct Op {
    op_type: OpAction,
    value: isize,
}

#[derive(Debug, Copy, Clone, Default)]
struct ProgramState {
    pc: usize,
    accumulator: isize,
}

impl ProgramState {
    fn next(self, instructions: &Vec<Op>) -> Option<Self> {
        if self.pc >= instructions.len() {
            return None;
        }
        let curr_op = instructions[self.pc];
        Some(match curr_op.op_type {
            OpAction::Accumulate => Self {
                pc: self.pc + 1,
                accumulator: self.accumulator + curr_op.value,
            },
            OpAction::Jump => Self {
                // Jumps can be negative, so have to convert
                #[allow(clippy::cast_possible_wrap)]
                pc: (self.pc as isize + curr_op.value).try_into().unwrap(),
                ..self
            },
            OpAction::NoOp => Self {
                pc: self.pc + 1,
                ..self
            },
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    let mut game_ops = parse_input_to_ops(&input_str);
    println!("Part 1: {}", part1(&mut game_ops.clone()));
    println!("Part 2: {}", part2(&mut game_ops, None));
}

fn get_input(input_file: &String) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

fn parse_input_to_ops(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|l| {
            let mut tokens = l.split(' ');
            Op {
                op_type: tokens.next().map_or_else(
                    || panic!("Invalid line to be parsed"),
                    |tok| match tok {
                        "jmp" => OpAction::Jump,
                        "acc" => OpAction::Accumulate,
                        "nop" => OpAction::NoOp,
                        _ => panic!("Unknown Operation Type"),
                    },
                ),
                value: tokens.next().map_or_else(
                    || panic!("Invalid line to be parsed"),
                    |tok| tok.parse().unwrap(),
                ),
            }
        })
        .collect_vec()
}

/*
 * Returns value of accumulator prior to loop.
 */
fn part1(instructions: &mut Vec<Op>) -> isize {
    let mut iter = itertools::iterate(Some(ProgramState::default()), |s| {
        s.unwrap().next(instructions)
    });
    let mut set: HashSet<usize> = HashSet::default();
    let repeated_op = iter
        .find(|op| !set.insert(op.unwrap().pc))
        .unwrap()
        .unwrap();
    repeated_op.accumulator
}

fn flip_op_type(instr: &mut OpAction) {
    *instr = match *instr {
        OpAction::Jump => OpAction::NoOp,
        OpAction::NoOp => OpAction::Jump,
        x @ OpAction::Accumulate => x,
    }
}

#[allow(dead_code)]
fn find_variant(instructions: &[Op]) {
    // all possible switches
    let mut variants: Vec<_> = instructions
        .iter()
        .enumerate()
        .filter_map(|(index, op)| match op.op_type {
            OpAction::Jump | OpAction::NoOp => Some(index),
            OpAction::Accumulate => None,
        })
        .map(|index| {
            let mut variant = instructions.to_owned();
            flip_op_type(&mut variant[index].op_type);
            (index, variant)
        })
        .map(|(index, variant)| {
            itertools::iterate(Some(ProgramState::default()), move |state| {
                state
                    .unwrap_or_else(|| panic!("Failed on {}", index))
                    .next(&variant)
            })
        })
        .collect();
    loop {
        for v in &mut variants {
            v.next();
        }
    }
}

fn eval_program(ops: &Vec<Op>) -> Option<isize> {
    // return final state accumulator
    itertools::iterate(Some(ProgramState::default()), |state| {
        state.and_then(|state| state.next(ops))
    })
    .while_some()
    .last()
    .map(|s| s.accumulator)
}

fn part2(instructions: &mut Vec<Op>, flip_idx: Option<usize>) -> isize {
    let flip_idx = flip_idx.unwrap_or(198); // via running find_variant
    flip_op_type(&mut instructions[flip_idx].op_type);
    eval_program(instructions).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "inputs/test-8.txt";

    #[test]
    fn correct_part1() {
        let input = get_input(&TEST_FILE.to_string());

        assert_eq!(part1(&mut parse_input_to_ops(&input)), 5);
    }

    #[test]
    fn correct_part2() {
        let input = get_input(&TEST_FILE.to_string());

        assert_eq!(part2(&mut parse_input_to_ops(&input), Some(7)), 8);
    }
}
