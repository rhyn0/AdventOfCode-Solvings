use hashbrown::HashMap;
use itertools::Itertools;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    let inputs = parse_input(&input_str);
    let sorted_jolts: Vec<usize> = get_adapters(&inputs);
    println!("Part 1: {}", part1(&sorted_jolts));
    println!("Part 2: {}", part2(&sorted_jolts));
}

fn get_input(input_file: &String) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec()
}

fn part1(nums: &[usize]) -> usize {
    let diff_3: usize = diff_occurrences(nums, 3);
    let diff_1: usize = diff_occurrences(nums, 1);
    diff_3 * diff_1
}

fn part2(nums: &[usize]) -> usize {
    // use hashmap to store number of ways from current node to end
    // previous nodes (less than) use sum of all reachable nodes after it
    let mut num_paths = HashMap::new();
    let n = nums.len();
    // last node (our device) has only one path to end
    num_paths.insert(nums.last().copied().unwrap(), 1);
    for idx in (0..(n - 1)).rev() {
        let adapter_val = nums[idx];
        // each adapter can reach up to 3 greater than its value
        // bound that to make sure we dont out of bounds the array
        let check_range = (idx + 1)..=std::cmp::min(idx + 3, n - 1);
        let num_neighbors: usize = check_range
            .filter_map(|j| {
                let j_val = nums[j];
                let gap = j_val - adapter_val;
                if (1..=3).contains(&gap) {
                    Some(num_paths.get(&j_val).unwrap())
                } else {
                    None
                }
            })
            .sum();
        num_paths.insert(adapter_val, num_neighbors);
    }
    // 0 is the wall value, so always the real start point
    num_paths.get(&0).unwrap().to_owned()
}

fn diff_occurrences(sorted_nums: &[usize], target: usize) -> usize {
    (sorted_nums)
        .iter()
        .tuple_windows()
        .filter_map(|(&a, &b)| if b - a == target { Some(1) } else { None })
        .count()
}

fn get_adapters(nums: &[usize]) -> Vec<usize> {
    let owned_nums = nums.to_owned();
    let mut adapters = std::iter::once(0)
        .chain(owned_nums.iter().copied())
        .collect_vec();
    adapters.sort_unstable();
    adapters.push(adapters.iter().max().unwrap() + 3);
    adapters
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "inputs/test-10.txt";

    #[test]
    fn correct_part1() {
        let input = parse_input(&get_input(&TEST_FILE.to_string()));
        let jolts = get_adapters(&input);

        assert_eq!(part1(&jolts), 220);
    }

    #[test]
    fn correct_part2() {
        let input = parse_input(&get_input(&TEST_FILE.to_string()));
        let jolts = get_adapters(&input);

        assert_eq!(part2(&jolts), 19208);
    }
}
