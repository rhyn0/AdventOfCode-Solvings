use std::collections::HashMap;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    let num_vec = input_string_to_vec(input_str);
    println!("Part 1: {}", part1(&num_vec));
    println!("Part 2: {}", part2(num_vec));
}

fn get_input(input_file: &String) -> String {
    return std::fs::read_to_string(input_file).unwrap();
}

fn input_string_to_vec(str: String) -> Vec<i64> {
    return str
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|x| x.parse().unwrap())
        .collect();
}

fn part1(nums: &Vec<i64>) -> i64 {
    let mut remainders = HashMap::new();
    for num in nums {
        let remain = 2020 - num;
        if remainders.contains_key(num) {
            return num * remainders.get(num).unwrap();
        }
        remainders.insert(remain, *num);
    }
    return 0;
}

fn part2(mut nums: Vec<i64>) -> i64 {
    let mut remainders = HashMap::new();
    nums.sort();
    if nums.len() < 3 {
        return 0;
    }
    for (i, num) in nums.iter().enumerate() {
        remainders.insert(num, i);
    }
    for (i, x) in nums.iter().enumerate() {
        let mut j = i + 1;
        while j < nums.len() {
            let required: i64 = (2020 - x - nums[j]).into();
            if remainders.contains_key(&required) && remainders.get(&required).unwrap() > &j {
                return x * nums[j] * required;
            }
            j += 1;
        }
    }
    return 0;
}
