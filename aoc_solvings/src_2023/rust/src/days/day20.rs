use itertools::Itertools;

use crate::etc::{Solution, SolutionPair};

use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    iter::once,
    ops::Add,
    str::FromStr,
};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("inputs/day-20.txt")).unwrap();

    (Solution::from(part1(&input)), Solution::from(part2(&input)))
}

fn part1(input: &str) -> usize {
    let mut problem = Problem::from_str(input).unwrap();
    let num_cycles = 1000usize;

    problem.cycle(num_cycles)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    comm_modules: HashMap<String, Module>,
}

impl FromStr for Problem {
    type Err = peg::error::ParseError<peg::str::LineCol>;

    #[allow(clippy::ignored_unit_patterns)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        peg::parser! {
            grammar comm_parser() for str {
                pub rule problem() -> Problem
                    = modules:module() ++ "\n" newline() ![_] {
                        let mut comm_modules = HashMap::new();
                        for module in modules {
                            comm_modules.insert(module.id.clone(), module);
                        }
                        Problem { comm_modules }
                    }

                rule module() -> Module
                    = name_and_type:module_name() __ "->" __ destinations:destinations() {
                        let (id, variant) = name_and_type;
                        Module { id, variant, destinations }
                    }

                rule module_name() -> (String, ModuleType)
                    = id:module_identifier()? name:ascii_text() {
                        let module_type = ModuleType::new(id.unwrap_or('?'));
                        (name, module_type)
                    }

                rule destinations() -> Vec<String>
                    = first:ascii_text() ids:("," __ id:ascii_text() { id.to_string()})* {
                        ids.iter().map(std::string::ToString::to_string).chain(once(first)).collect()
                    }

                rule module_identifier() -> char
                    = a:[ '%' | '&' ] {a}

                rule ascii_text() -> String
                    = a:['A'..='Z' | 'a'..='z']+ { a.iter().collect::<String>() }

                rule __()
                    = [' ' | '\t']+

                rule newline()
                    = ['\r' | '\n']+
            }
        }
        match comm_parser::problem(s) {
            Ok(mut problem) => {
                problem.initialize_conjunctions();
                Ok(problem)
            }
            Err(err) => Err(err),
        }
    }
}

impl Problem {
    /// Press the button `number_of_times` and return the score of the signals.
    fn cycle(&mut self, number_of_times: usize) -> usize {
        let result = (0..number_of_times).fold(ButtonSignalGenerated::default(), |acc, _| {
            acc + self.press_button()
        });
        result.total()
    }
    /// Initialize Conjuctions modules with their full map of input names
    fn initialize_conjunctions(&mut self) {
        let mut conjuct_modules: HashMap<String, Vec<String>> = self
            .comm_modules
            .values()
            .filter_map(|module| match module.variant {
                ModuleType::Conjunction { .. } => Some((module.id.clone(), Vec::new())),
                _ => None,
            })
            .collect();
        for (id, module) in &self.comm_modules {
            for dest in &module.destinations {
                if let Some(conjuct) = conjuct_modules.get_mut(dest) {
                    conjuct.push(id.clone());
                }
            }
        }
        for (id, inputs) in &conjuct_modules {
            if let Some(module) = self.comm_modules.get_mut(id) {
                if let ModuleType::Conjunction { ref mut memory } = module.variant {
                    *memory = inputs.iter().map(|id| (id.clone(), Signal::Low)).collect();
                }
            }
        }
    }
    /// Return the number of signals created from pushing the button this time.
    fn press_button(&mut self) -> ButtonSignalGenerated {
        let mut signals_sent = ButtonSignalGenerated::default();
        let mut signal_queue: VecDeque<InputSignal> = VecDeque::new();

        signal_queue.push_back(InputSignal::new(
            "button".to_string(),
            "broadcaster".to_string(),
            Signal::Low,
        ));
        // this button press counts
        signals_sent.low += 1;

        while let Some(InputSignal { from, to, signal }) = signal_queue.pop_front() {
            if let Some(module) = self.comm_modules.get_mut(&to) {
                let (new_module, new_signal) = module.variant.process_signal(&from, signal);
                if let Some(new_signal) = new_signal {
                    match new_signal {
                        Signal::Low => signals_sent.low += module.destinations.len(),
                        Signal::High => signals_sent.high += module.destinations.len(),
                    }
                    for dest in &module.destinations {
                        signal_queue.push_back(InputSignal::new(
                            to.clone(),
                            dest.to_string(),
                            new_signal,
                        ));
                    }
                }
                *self.comm_modules.get_mut(&to).unwrap() = module.new_with_type(new_module);
            }
        }
        signals_sent
    }
    fn find_parent(&self, id: &str) -> Option<&Module> {
        self.comm_modules
            .values()
            .find(|module| module.destinations.contains(&id.to_string()))
    }
}

#[derive(Debug, Clone)]
struct InputSignal {
    from: String,
    to: String,
    signal: Signal,
}

impl InputSignal {
    const fn new(from: String, to: String, signal: Signal) -> Self {
        Self { from, to, signal }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ButtonSignalGenerated {
    low: usize,
    high: usize,
}

impl ButtonSignalGenerated {
    const fn total(self) -> usize {
        self.low * self.high
    }
}

impl Add<Self> for ButtonSignalGenerated {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            low: self.low + other.low,
            high: self.high + other.high,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module {
    id: String,
    variant: ModuleType,
    destinations: Vec<String>, // store the ids of the modules that can be reached from this module
}

impl Module {
    fn new_with_type(&self, variant: ModuleType) -> Self {
        Self {
            variant,
            ..self.clone()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ModuleType {
    /// toggle on state when receiving a low signal.
    /// when switching to on, it sends a high signal
    /// when switching to off, it sends a low signal
    FlipFlop { status: bool },
    /// memory based module, only sends low pulse if most recent signal for all inputs was high
    /// otherwise sends high pulse
    Conjunction { memory: HashMap<String, Signal> },
    /// broadcaster repeats the signal it receives to all its destinations
    Broadcaster,
}

impl ModuleType {
    fn new(ch: char) -> Self {
        match ch {
            '%' => Self::FlipFlop { status: false },
            '&' => Self::Conjunction {
                memory: HashMap::new(),
            },
            _ => Self::Broadcaster,
        }
    }
    fn process_signal(&self, from_module: &str, signal: Signal) -> (Self, Option<Signal>) {
        match self {
            Self::FlipFlop { status } => match signal {
                // flip flop only responds when it receives a low signal
                Signal::Low => {
                    let new_status = !status;
                    let new_signal = if new_status {
                        Signal::High
                    } else {
                        Signal::Low
                    };
                    (Self::FlipFlop { status: new_status }, Some(new_signal))
                }
                Signal::High => (Self::FlipFlop { status: *status }, None),
            },
            Self::Conjunction { memory } => {
                // conjunctions update their memory with this signal first
                // if all incoming signals are high, it sends a low signal
                // otherwise send a high signal
                let mut new_memory = memory.clone();
                new_memory
                    .entry(from_module.to_string())
                    .and_modify(|sig| *sig = signal)
                    .or_insert(signal);

                let new_signal = if new_memory.values().all(|s| s == &Signal::High) {
                    Signal::Low
                } else {
                    Signal::High
                };
                (Self::Conjunction { memory: new_memory }, Some(new_signal))
            }
            Self::Broadcaster => (Self::Broadcaster, Some(signal)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Signal {
    Low,
    High,
}

fn part2(input: &str) -> usize {
    let mut problem = Problem::from_str(input).unwrap();
    let mut button_presses = 0;
    let mut signal_queue: VecDeque<InputSignal> = VecDeque::new();

    // in our case, 'rx' only has one parent that is a conjunction.
    // so it will only send a low pulse when all its parents are high
    // so find rx parent, then use its initialized HashMap to get all the names of the parents and see if they are all high
    // if they are, then a low pulse is sent to rx and we can return it
    let parent_rx = problem.find_parent("rx").unwrap().clone();
    let parent_rx_inputs = match parent_rx.variant {
        ModuleType::Conjunction { ref memory } => memory.keys().cloned().collect_vec(),
        _ => panic!("rx parent is not a conjunction"),
    };
    let mut first_button_press_per_rx_parent: HashMap<String, usize> =
        HashMap::with_capacity(parent_rx_inputs.len());
    loop {
        signal_queue.push_back(InputSignal::new(
            "button".to_string(),
            "broadcaster".to_string(),
            Signal::Low,
        ));
        button_presses += 1;

        while let Some(InputSignal { from, to, signal }) = signal_queue.pop_front() {
            // check if this is an input to rx parent
            if to == parent_rx.id && signal == Signal::High {
                first_button_press_per_rx_parent
                    .entry(from.clone())
                    .or_insert(button_presses);
                if first_button_press_per_rx_parent.len() == parent_rx_inputs.len() {
                    // we return LCM of all the first button presses for each parent
                    // not all parents are high when the last one is finally entered into this map
                    return lcm(&first_button_press_per_rx_parent
                        .values()
                        .copied()
                        .collect_vec());
                }
            }

            if let Some(module) = problem.comm_modules.get_mut(&to) {
                let (new_module, new_signal) = module.variant.process_signal(&from, signal);
                if let Some(new_signal) = new_signal {
                    for dest in &module.destinations {
                        signal_queue.push_back(InputSignal::new(
                            to.clone(),
                            dest.to_string(),
                            new_signal,
                        ));
                    }
                }
                *problem.comm_modules.get_mut(&to).unwrap() = module.new_with_type(new_module);
            }
        }
    }
}
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
fn lcm(values: &[usize]) -> usize {
    values.iter().fold(1, |acc, &val| acc * val / gcd(acc, val))
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = read_to_string("inputs/day-20.example.txt").unwrap();
        assert_eq!(part1(&input), 32000000);
    }
    #[test]
    fn test_part1_alternatvie() {
        let input = indoc! {"
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output
        "};
        assert_eq!(part1(&input), 11687500);
    }
    #[test]
    fn test_parse_problem() {
        let input = read_to_string("inputs/day-20.example.txt").unwrap();

        let problem = Problem::from_str(&input).unwrap();
        assert_eq!(problem.comm_modules.len(), 5);
        assert_eq!(
            problem
                .comm_modules
                .values()
                .filter(|module| module.variant == ModuleType::Broadcaster)
                .count(),
            1
        );
    }
    #[test]
    fn test_conjunction_init() {
        let input = indoc! {"
            broadcaster -> a
            %a -> b, c
            &b -> broadcaster
            &c -> broadcaster
        "};
        let problem = Problem::from_str(&input).unwrap();
        let conjuct_one = problem.comm_modules.get("b").unwrap();
        let conjuct_two = problem.comm_modules.get("c").unwrap();

        let mut expected_memory = HashMap::new();
        expected_memory.insert("a".to_string(), Signal::Low);
        assert_eq!(
            conjuct_one.variant,
            ModuleType::Conjunction {
                memory: expected_memory.clone()
            }
        );
        assert_eq!(
            conjuct_two.variant,
            ModuleType::Conjunction {
                memory: expected_memory
            }
        );
    }
    #[test]
    fn test_single_button_press() {
        let input = read_to_string("inputs/day-20.example.txt").unwrap();
        let mut problem = Problem::from_str(&input).unwrap();
        let signals = problem.press_button();
        assert_eq!(signals.low, 8);
        assert_eq!(signals.high, 4);
        assert_eq!(signals.total(), 32);
    }
}
