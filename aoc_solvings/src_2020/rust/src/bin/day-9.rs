use std::env;

use itertools::Itertools;
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    let inputs = parse_input(&input_str);
    let part1_ans: usize = part1(&inputs, None);
    println!("Part 1: {part1_ans}");
    println!("Part 2: {}", part2(&inputs, part1_ans));
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

fn part1(nums: &[usize], preamble: Option<usize>) -> usize {
    let preamble_len: usize = preamble.unwrap_or(25);
    nums.windows(preamble_len + 1)
        .find_map(|window| {
            if (window[..preamble_len])
                .iter()
                .tuple_combinations()
                .any(|(a, b)| a + b == window[preamble_len])
            {
                None
            } else {
                Some(window[preamble_len])
            }
        })
        .unwrap()
}

fn part2(nums: &[usize], target: usize) -> usize {
    let (idx, length, _) = find_contiguous_set(nums, target);
    // encryption weakness is min number in this contiguous set plus max number
    let set_nums = &nums[idx..][..length];
    set_nums.iter().min().unwrap() + set_nums.iter().max().unwrap()
}

#[allow(dead_code)]
fn find_contiguous_set_brute(nums: &[usize], target: usize) -> (usize, usize, usize) {
    (2..=nums.len())
        .flat_map(|win_size| {
            nums.windows(win_size)
                .enumerate()
                .map(move |(i, win)| (i, win_size, win.iter().sum::<usize>()))
        })
        .find(|&(_, _, value)| value == target)
        .unwrap()
}

fn find_contiguous_set(nums: &[usize], target: usize) -> (usize, usize, usize) {
    let (start, end) = nums
        .iter()
        .scan(0, |sum, n| {
            *sum += n;
            Some(*sum)
        })
        .enumerate()
        .tuple_combinations()
        .find_map(|((idx1, p1), (idx2, p2))| {
            if p2 - p1 == target {
                Some((idx1, idx2))
            } else {
                None
            }
        })
        .unwrap();
    // prefix sum is remove all numbers at this index and prior
    // so actual start idx is +1
    (start + 1, end - start, target)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "inputs/test-9.txt";
    const PART1_ANS: usize = 127;

    #[test]
    fn correct_part1() {
        let input = get_input(&TEST_FILE.to_string());

        assert_eq!(part1(&parse_input(&input), Some(5)), PART1_ANS);
    }

    #[test]
    fn correct_part2() {
        let input = get_input(&TEST_FILE.to_string());

        assert_eq!(part2(&parse_input(&input), PART1_ANS), 62);
    }

    #[test]
    fn brute_equal_prefix() {
        let input = parse_input(&get_input(&TEST_FILE.to_string()));

        assert_eq!(
            find_contiguous_set(&input, PART1_ANS),
            find_contiguous_set_brute(&input, PART1_ANS)
        );
    }
}
