use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    let boarding_passes = parse_boarding_passes(&input_str);
    println!("Part 1: {}", part1(&boarding_passes));
    println!("Part 2: {}", part2(boarding_passes));
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

fn part2(mut passes: Vec<(u64, u64)>) -> u128 {
    let mut seat_grid: Vec<Vec<bool>> = vec![vec![false; 8]; 128];
    passes.sort_by_key(|p| p.0);
    passes.sort_by_key(|p| p.1);
    for (row, col) in passes {
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
