use itertools::Itertools;
use multimap::MultiMap;
use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let my_bag: Bag = ("shiny", "gold");
    let input_str = get_input(&args[1]);
    let bag_rules = parse_input_to_rules(&input_str);
    println!("Part 1: {}", part1(&bag_rules, &my_bag));
    println!("Part 2: {}", part2(&bag_rules, &my_bag));
}

fn get_input(input_file: &String) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

type Bag<'a> = (&'a str, &'a str);

type Rules<'a> = MultiMap<Bag<'a>, (usize, Bag<'a>)>;

fn parse_input_to_rules(input: &str) -> Rules<'_> {
    // match each line for an occurrence of `"color" bags contain`
    // then parse the line for repeats of `(?:([\d]+) ([\s\w]+))`
    // for each match, we get 2 groups - count and color
    // if there were no matches, like in 'no other bags' - we do nothing
    let valid_line_re = Regex::new(
        r"^([\w\s]+) bags contain (?:([\d]+) ([\s\w]+)|no other) bags?(?:, ([\d]+) ([\sa-z]+) bags?)*\.$",
    )
    .unwrap();
    let bag_re = Regex::new(r"[\d]+ [\s\w]+ bags?").unwrap();
    let mut rules: Rules = MultiMap::default();
    for line in input.lines().filter(|line| valid_line_re.is_match(line)) {
        let capture = valid_line_re.captures(line).unwrap();
        let top_bag: Bag = capture
            .get(1)
            .unwrap()
            .as_str()
            .split_whitespace()
            .next_tuple()
            .unwrap();
        for re_match in bag_re.find_iter(line) {
            let match_iter = re_match.as_str().splitn(2, ' ');
            let (num_bags_s, bag_color_combined): (&str, &str) =
                match_iter.into_iter().next_tuple().unwrap();
            let num_bags = num_bags_s.parse::<usize>().unwrap();
            let bag_color: Bag = bag_color_combined
                .split_ascii_whitespace()
                .take_while(|&word| word != "bags")
                .next_tuple()
                .unwrap();
            rules.insert(top_bag, (num_bags, bag_color));
        }
    }
    rules
}

fn subgraph_contains(graph: &Rules<'_>, root: &Bag, needle: &Bag) -> bool {
    graph
        .get_vec(root)
        .map(|v| {
            v.iter().any(|(_, neighbor_bag)| {
                neighbor_bag == needle || subgraph_contains(graph, neighbor_bag, needle)
            })
        })
        .unwrap_or_default()
}

fn part1(rules: &Rules<'_>, goal: &Bag) -> u128 {
    rules
        .keys()
        .filter(|&k| k != goal)
        .filter(|&k| subgraph_contains(rules, k, goal))
        .count() as u128
}

fn bag_count_iter<'iter, 'elems: 'iter>(
    graph: &'iter Rules<'elems>,
    start: &(&'iter str, &'iter str),
) -> Box<dyn Iterator<Item = usize> + 'iter> {
    Box::new(
        graph
            .get_vec(start)
            .into_iter()
            .flatten()
            .flat_map(move |&(qt, neighbor)| {
                std::iter::once(qt).chain(bag_count_iter(graph, &neighbor).map(move |x| x * qt))
            }),
    )
}

fn part2(rules: &Rules<'_>, start: &Bag) -> u128 {
    bag_count_iter(rules, start).sum::<usize>() as u128
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "inputs/test-7.txt";
    const GOAL_BAG: Bag = ("shiny", "gold");

    #[test]
    fn correct_part1() {
        let input = get_input(&TEST_FILE.to_string());

        assert_eq!(part1(&parse_input_to_rules(&input), &GOAL_BAG), 4);
    }

    #[test]
    fn correct_part2() {
        let input = get_input(&TEST_FILE.to_string());

        assert_eq!(part2(&parse_input_to_rules(&input), &GOAL_BAG), 32);
    }
}
