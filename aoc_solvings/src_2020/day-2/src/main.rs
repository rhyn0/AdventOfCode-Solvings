use regex::{Captures, Regex};
use std::env;

#[derive(Debug)]
struct PasswordAttempt {
    min_amount: u32,
    max_amount: u32,
    character: char,
    password: String,
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    let passwords = parse_password_attempts(&input_str);
    println!("Part 1: {}", part1(&passwords));
    println!("Part 2: {}", part2(&passwords));
}

fn get_input(input_file: &String) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

fn parse_password_attempt_line(cap: Captures) -> PasswordAttempt {
    let p = PasswordAttempt {
        min_amount: cap[1].parse().unwrap(),
        max_amount: cap[2].parse().unwrap(),
        character: cap[3].chars().last().unwrap(),
        password: String::from(&cap[4]),
    };
    p
}

fn parse_password_attempts(input: &str) -> Vec<PasswordAttempt> {
    let re: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    return input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|x| re.captures(x).unwrap())
        .map(parse_password_attempt_line)
        .collect();
}

fn part1(pswds: &Vec<PasswordAttempt>) -> u64 {
    let mut count = 0;
    for pswd in pswds {
        let appearances = pswd
            .password
            .matches(pswd.character)
            .count()
            .try_into()
            .unwrap();
        if pswd.min_amount <= appearances && appearances <= pswd.max_amount {
            count += 1;
        }
    }
    count
}

fn part2(pswds: &Vec<PasswordAttempt>) -> u64 {
    // Reminder that now min_amount means first index (1 based)
    // and that max_amount is second index (1 based)
    let mut count = 0;
    for pswd in pswds {
        let first_match = pswd
            .password
            .chars()
            .nth((pswd.min_amount - 1).try_into().unwrap())
            .unwrap()
            == pswd.character;
        let second_match = pswd
            .password
            .chars()
            .nth((pswd.max_amount - 1).try_into().unwrap())
            .unwrap()
            == pswd.character;
        if first_match ^ second_match {
            count += 1;
        }
    }
    count
}
