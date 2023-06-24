use std::collections::HashMap;
use std::env;

#[derive(Debug)]
struct Passport {
    fields: HashMap<String, String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    let passports = parse_passports(&input_str);
    println!("Part 1: {}", part1(&passports));
    println!("Part 2: {}", part2(&passports));
}

fn get_input(input_file: &String) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

fn _create_passport(passport_input: Vec<&str>) -> Passport {
    let mut p_fields = HashMap::new();
    for field in passport_input {
        let split_fields: Vec<&str> = field.split(':').collect();
        p_fields.insert(split_fields[0].to_string(), split_fields[1].to_string());
    }
    Passport { fields: p_fields }
}

fn parse_passports(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .map(|x| _create_passport(x.trim().split('\n').flat_map(|y| y.split(' ')).collect()))
        .collect()
}

fn part1(pports: &[Passport]) -> u128 {
    pports
        .iter()
        .filter(|p| {
            p.fields.contains_key("cid") && p.fields.len() == 8
                || !p.fields.contains_key("cid") && p.fields.len() == 7
        })
        .count()
        .try_into()
        .unwrap()
}

fn _valid_byr(byr_value: &str) -> bool {
    byr_value.len() == 4 && (1920..=2002).contains(&byr_value.parse::<u128>().unwrap())
}

fn _valid_iyr(iyr_value: &str) -> bool {
    iyr_value.len() == 4 && (2010..=2020).contains(&iyr_value.parse::<u128>().unwrap())
}

fn _valid_eyr(eyr_value: &str) -> bool {
    eyr_value.len() == 4 && (2020..=2030).contains(&eyr_value.parse::<u128>().unwrap())
}

fn _valid_hcl(hcl_value: &str) -> bool {
    hcl_value.len() == 7
        && hcl_value.starts_with('#')
        && hcl_value[1..]
            .chars()
            .filter(|c| c.is_ascii_hexdigit() || c.is_ascii_digit())
            .count()
            == 6
}

fn _valid_hgt(hgt_value: &str) -> bool {
    let hgt = hgt_value
        .chars()
        .take_while(|x| x.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();
    if hgt_value.ends_with("cm") {
        (150..=193).contains(&hgt)
    } else if hgt_value.ends_with("in") {
        (59..=76).contains(&hgt)
    } else {
        false
    }
}

fn _valid_ecl(ecl_value: &str) -> bool {
    vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .any(|&color| color == ecl_value)
}

fn _valid_pid(pid_value: &str) -> bool {
    pid_value.len() == 9 && pid_value.chars().filter(|c| c.is_ascii_digit()).count() == 9
}

fn part2(pports: &[Passport]) -> u128 {
    pports
        .iter()
        .filter(|p| {
            p.fields.contains_key("cid") && p.fields.len() == 8
                || !p.fields.contains_key("cid") && p.fields.len() == 7
        })
        .filter(|p| {
            _valid_byr(p.fields.get("byr").unwrap())
                && _valid_ecl(p.fields.get("ecl").unwrap())
                && _valid_eyr(p.fields.get("eyr").unwrap())
                && _valid_hcl(p.fields.get("hcl").unwrap())
                && _valid_iyr(p.fields.get("iyr").unwrap())
                && _valid_pid(p.fields.get("pid").unwrap())
                && _valid_hgt(p.fields.get("hgt").unwrap())
        })
        .count()
        .try_into()
        .unwrap()
}
