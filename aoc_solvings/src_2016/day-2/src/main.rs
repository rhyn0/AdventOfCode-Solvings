use std::collections::HashMap;
use std::{env, vec};
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    let patterns = input_string_to_vec(&input_str);
    println!("Part 1: {}", part1(&patterns));
    println!("Part 2: {}", part2(&patterns));
}

fn get_input(input_file: &String) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

fn input_string_to_vec(str: &str) -> Vec<String> {
    return str
        .lines()
        .filter(|line| !line.is_empty())
        .map(std::string::ToString::to_string)
        .collect();
}

fn part1(commands: &Vec<String>) -> u128 {
    // original start point is at 5, center of grid
    let (mut row, mut col): (u128, u128) = (1, 1);
    let mut bathroom_code: u128 = 0;
    for cmd_str in commands {
        bathroom_code *= 10;
        (row, col) = _parse_bathroom_digit_pattern(cmd_str, row, col);
        bathroom_code += row * 3 + col + 1;
    }
    bathroom_code
}

fn _parse_bathroom_digit_pattern(pattern: &str, mut row: u128, mut col: u128) -> (u128, u128) {
    for character in pattern.chars() {
        match character {
            'U' => row = if row == 0 { 0 } else { row - 1 },
            'L' => col = if col == 0 { 0 } else { col - 1 },
            'D' => row = if row == 2 { 2 } else { row + 1 },
            'R' => col = if col == 2 { 2 } else { col + 1 },
            _ => (),
        }
    }
    (row, col)
}

fn direction_to_index(dir: char) -> usize {
    match dir {
        'U' => 0,
        'L' => 1,
        'D' => 2,
        'R' => 3,
        _ => panic!(),
    }
}

fn part2(commands: &Vec<String>) -> String {
    let avail_moves: HashMap<char, [char; 4]> = HashMap::from_iter([
        ('1', ['1', '1', '3', '1']),
        ('2', ['2', '2', '6', '3']),
        ('3', ['1', '2', '7', '4']),
        ('4', ['4', '3', '8', '4']),
        ('5', ['5', '5', '5', '6']),
        ('6', ['2', '5', 'A', '7']),
        ('7', ['3', '6', 'B', '8']),
        ('8', ['4', '7', 'C', '9']),
        ('9', ['9', '8', '9', '9']),
        ('A', ['6', 'A', 'A', 'B']),
        ('B', ['7', 'A', 'D', 'C']),
        ('C', ['8', 'B', 'C', 'C']),
        ('D', ['B', 'D', 'D', 'D']),
    ]);
    let mut curr_code_vec: Vec<char> = vec![];
    let mut position = '5';
    for command in commands {
        for c in command.chars() {
            position = *avail_moves
                .get(&position)
                .unwrap()
                .get(direction_to_index(c))
                .unwrap();
        }
        curr_code_vec.push(position);
    }
    curr_code_vec.iter().collect::<String>()
}
