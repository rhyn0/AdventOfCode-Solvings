use itertools::Itertools;
use std::collections::HashSet;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    let boarding_passes = parse_boarding_passes(&input_str);
    println!("Part 1: {}", part1(&boarding_passes));
    println!("Part 2: {}", part2_alt(&boarding_passes));
}

fn get_input(input_file: &String) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

fn seat_id((row, col): (u64, u64)) -> u128 {
    (row * 8 + col).into()
}

fn _parse_bin_section(section_str: &str, one: char) -> u64 {
    u64::from_str_radix(
        &section_str
            .chars()
            .map(|c| if c == one { '1' } else { '0' })
            .collect::<String>(),
        2,
    )
    .unwrap()
}

fn _parse_boarding_pass(pass: &str) -> (u64, u64) {
    (
        _parse_bin_section(&pass[..7], 'B'),
        _parse_bin_section(&pass[7..], 'R'),
    )
}

fn parse_boarding_passes(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(_parse_boarding_pass)
        .collect()
}

fn part1(passes: &[(u64, u64)]) -> u128 {
    passes.iter().map(|p| seat_id(*p)).max().unwrap()
}

#[allow(dead_code)]
fn part2(passes: &[(u64, u64)]) -> u128 {
    let mut seat_grid: Vec<Vec<bool>> = vec![vec![false; 8]; 128];
    for &(row, col) in passes.iter().sorted_by_key(|p| p.0).sorted_by_key(|p| p.1) {
        seat_grid[usize::try_from(row).unwrap()][usize::try_from(col).unwrap()] = true;
    }
    let row_id = seat_grid
        .iter()
        .position(|row| row.iter().filter(|seat| !**seat).count() == 1)
        .unwrap();
    let col_id = seat_grid
        .get(row_id)
        .unwrap()
        .iter()
        .position(|val| !*val)
        .unwrap();
    seat_id((row_id as u64, col_id as u64))
}

fn part2_alt(passes: &[(u64, u64)]) -> u128 {
    let seats: HashSet<u128> = passes.iter().map(|&seat| seat_id(seat)).sorted().collect();
    let (&min_seat, &max_seat) = (seats.iter().min().unwrap(), seats.iter().max().unwrap());
    for my_seat in min_seat..max_seat {
        if seats.contains(&my_seat) {
            continue;
        }
        return my_seat;
    }
    panic!("My seat is MISSING");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "inputs/test-5.txt";
    fn _input_setup() -> Vec<(u64, u64)> {
        parse_boarding_passes(&get_input(&TEST_FILE.to_string()))
    }

    #[test]
    fn correct_part1() {
        assert_eq!(part1(&_input_setup()), 820);
    }

    #[test]
    #[should_panic]
    fn correct_part2() {
        // this panics due to not having enough inputs in the test file
        // this is the shortcoming of this memory intensive method
        part2(&_input_setup());
    }

    #[test]
    fn correct_part2_alt() {
        // minimum seat id is 119 in the 3 seats given
        // and since 120 is not one of the 3 seats, it assumes this is answer
        assert_eq!(part2_alt(&_input_setup()), 120);
    }
}
