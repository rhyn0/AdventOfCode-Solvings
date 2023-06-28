use std::collections::HashSet;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    let groups = parse_input_to_groups(&input_str);
    println!("Part 1: {}", part1(&groups));
    println!("Part 2: {}", part2(&groups));
}

fn get_input(input_file: &String) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

fn parse_input_to_groups(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .map(|group| group.trim().to_string())
        .collect()
}

fn parse_customs_anyone(input: &[String]) -> Vec<HashSet<char>> {
    // One person of the group needs to answer YES to a question for
    // it to count in the end. Make a HashSet out of the valid characters
    // [a-z] and return those
    input
        .iter()
        .map(|group| group.chars().filter(char::is_ascii_lowercase).collect())
        .collect()
}

fn get_total_length<T>(set: &[HashSet<T>]) -> u128 {
    set.iter().flatten().count() as u128
}

fn part1(groups: &[String]) -> u128 {
    let group_answers = parse_customs_anyone(groups);
    get_total_length(&group_answers)
}

fn parse_customs_everyone(input: &[String]) -> Vec<HashSet<char>> {
    // Each group needs to answer YES to same questions for it to count
    // Take HashSet of each person's answer in a group then find
    // intersection with the rest of the group
    input
        .iter()
        .map(|group| {
            group
                .split('\n')
                .map(|person| person.chars().collect())
                .reduce(|acc, set| &acc & &set)
                .unwrap()
        })
        .collect()
}

fn part2(groups: &[String]) -> u128 {
    let group_answers = parse_customs_everyone(groups);
    get_total_length(&group_answers)
}
