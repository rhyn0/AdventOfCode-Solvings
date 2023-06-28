use std::{env, vec};
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    println!("Part 1: {}", part1(&input_str));
    println!("Part 2: {}", part2(&input_str));
}

fn get_input(input_file: &String) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

fn row_based_triangles(str: &str) -> Vec<Vec<u128>> {
    // Each line is formatted like the following
    //      x  y  z
    // two spaces seperating each value, each line is an input
    // two leading spaces on the line
    return str
        .lines()
        .map(|s| {
            s.trim()
                .split("  ")
                .filter_map(|x| x.trim().parse::<u128>().ok())
                .collect::<Vec<u128>>()
        })
        .collect();
}

fn part1(input_str: &str) -> u64 {
    // count number of valid triangles
    // assuming that each row of input is a triangle
    let row_triangles = row_based_triangles(input_str);
    count_valid_triangles(&row_triangles)
}

fn count_valid_triangles(triangles: &[Vec<u128>]) -> u64 {
    let mut num_valid_triangles: u64 = 0;
    for mut triangle in triangles.iter().cloned() {
        triangle.sort_unstable();
        if triangle[..2].iter().sum::<u128>() > *triangle.get(2).unwrap() {
            num_valid_triangles += 1;
        }
    }
    num_valid_triangles
}

fn col_based_triangles(str: &str) -> Vec<Vec<u128>> {
    let (mut v0, mut v1, mut v2): (Vec<u128>, Vec<u128>, Vec<u128>) = (vec![], vec![], vec![]);
    let mut ret_vec: Vec<Vec<u128>> = vec![];
    str.lines()
        .map(|s| {
            s.trim()
                .split("  ")
                .filter_map(|x| x.trim().parse::<u128>().ok())
                .collect::<Vec<u128>>()
        })
        .for_each(|row| {
            let &[zero, one, two] = &row[..] else {unreachable!()};
            v0.push(zero);
            v1.push(one);
            v2.push(two);
            if v0.len() == 3 {
                ret_vec.push(v0.clone());
                ret_vec.push(v1.clone());
                ret_vec.push(v2.clone());
                (v0, v1, v2) = (vec![], vec![], vec![]);
            }
        });
    ret_vec
}

fn part2(input_str: &str) -> u64 {
    // count the number of valid triangles when grouped in their column
    // Each column is not related to any other column
    // Still groups of 3 but within a group, Example:
    // 101 301 501
    // 102 302 502
    // 103 303 503
    // 201 401 601
    // 202 402 602
    // 203 403 603
    // where each hundred is grouped together
    let col_triangles = col_based_triangles(input_str);
    count_valid_triangles(&col_triangles)
}
