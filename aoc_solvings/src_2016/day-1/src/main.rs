use std::collections::HashSet;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    let turns = input_string_to_vec(&input_str);
    println!("Part 1: {}", part1(&turns));
    println!("Part 2: {}", part2(&turns));
}

fn get_input(input_file: &String) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

fn input_string_to_vec(str: &str) -> Vec<String> {
    // first line is commands
    return str
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.trim().to_string())
        .collect();
}

const NORTH: i8 = 0;

fn part1(commands: &Vec<String>) -> u64 {
    // start orientation is north
    // so for every turn we are one of cardinal directions
    // N and S cancel, E and W cancel
    // EAST 1, WEST 3, SOUTH 2
    let mut direction = NORTH;
    let (mut north, mut east): (i64, i64) = (0, 0);
    for command in commands {
        if command.starts_with('R') {
            direction += 1;
        } else {
            direction += 3; // same as negative one
        }
        direction = direction.abs() % 4;
        match direction {
            0 => north += command[1..].parse::<i64>().unwrap(),
            2 => north -= command[1..].parse::<i64>().unwrap(),
            1 => east += command[1..].parse::<i64>().unwrap(),
            3 => east -= command[1..].parse::<i64>().unwrap(),
            _ => panic!(),
        }
    }
    (north.abs() + east.abs()).try_into().unwrap()
}

fn part2(commands: &Vec<String>) -> u64 {
    // start orientation is north
    // so for every turn we are one of cardinal directions
    // N and S cancel, E and W cancel
    // EAST 1, WEST 3, SOUTH 2
    let mut direction = NORTH;
    let (mut north, mut east): (i64, i64) = (0, 0);
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    for command in commands {
        if command.starts_with('R') {
            direction += 1;
        } else {
            direction += 3; // same as negative one
        }
        direction = direction.abs() % 4;
        let modifier = if direction == 2 || direction == 3 {
            -1
        } else {
            1
        };
        let dist = command[1..].parse::<i64>().unwrap();
        for _ in 1..=dist {
            if direction % 2 == 0 {
                north += modifier;
            } else {
                east += modifier;
            }
            if visited.contains(&(north, east)) {
                return (north.abs() + east.abs()).try_into().unwrap();
            }
            visited.insert((north, east));
        }
    }
    panic!();
}
