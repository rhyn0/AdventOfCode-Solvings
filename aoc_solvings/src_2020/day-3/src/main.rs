use std::env;

const TREE: char = '#';

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    let tree_matrix = parse_trees(&input_str);
    println!("Part 1: {}", part1(&tree_matrix));
    println!("Part 2: {}", part2(&tree_matrix));
}

fn get_input(input_file: &String) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

fn parse_trees(input: &str) -> Vec<Vec<char>> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|x| x.chars().collect())
        .collect()
}

fn simulate_trees_hit(tree_matrix: &Vec<Vec<char>>, delta_rows: usize, delta_cols: usize) -> u128 {
    // n rows, m col
    let n = tree_matrix.len();
    let m = tree_matrix[0].len();
    let (mut row, mut col): (usize, usize) = (delta_rows, delta_cols);
    let mut trees_hit: u128 = 0;
    while row < n {
        if col >= m {
            col %= m;
        }
        if tree_matrix[row][col] == TREE {
            trees_hit += 1;
        }
        row += delta_rows;
        col += delta_cols;
    }
    trees_hit
}

fn part1(trees: &Vec<Vec<char>>) -> u128 {
    simulate_trees_hit(trees, 1, 3)
}

fn part2(trees: &Vec<Vec<char>>) -> u128 {
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let all_slopes: Vec<u128> = slopes
        .iter()
        .map(|(x, y)| simulate_trees_hit(trees, *y, *x))
        .collect();
    all_slopes.iter().product()
}
