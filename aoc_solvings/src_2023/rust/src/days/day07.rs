use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

use crate::etc::{Solution, SolutionPair};

pub fn solve(input_path: Option<&str>) -> SolutionPair {
    let input = read_to_string(input_path.unwrap_or("inputs/day-07.txt")).unwrap();
    (Solution::from(part1(&input)), Solution::from(part2(&input)))
}

fn part1(input: &str) -> u64 {
    let problem = Problem::from_str(input, false);
    let mut hands = problem.hands;
    // sort by the type of hand, break ties by value of cards in order
    get_total_winnings(&mut hands)
}

fn get_total_winnings(hands: &mut [Hand]) -> u64 {
    hands.sort_unstable_by_key(|hand| (hand.strength, hand.cards));
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (hand.bet * (i + 1) as u64))
}

const CARD_CHARS: [&str; 14] = [
    // the one represents the JOKER when in play in part2
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    hands: Vec<Hand>,
}

impl Problem {
    fn from_str(s: &str, with_joker: bool) -> Self {
        let hands: Vec<Hand> = s
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let hand_components: (&str, &str) = line.split(' ').collect_tuple().unwrap();
                Hand::new(
                    hand_components.0,
                    hand_components.1.parse().unwrap(),
                    with_joker,
                )
            })
            .collect();
        Self { hands }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: (u8, u8, u8, u8, u8),
    bet: u64,
    strength: HandType,
}

impl Hand {
    fn new(cards: &str, bet: u64, joker: bool) -> Self {
        let card_ints = cards
            .chars()
            .map(|char| {
                if joker && char == 'J' {
                    1
                } else {
                    u8::try_from(
                        CARD_CHARS
                            .iter()
                            .position(|x| x == &char.to_string())
                            .unwrap(),
                    )
                    .unwrap()
                        + 1
                }
            })
            .collect_vec();
        let mut card_counts: HashMap<u8, usize> = HashMap::new();
        for card in &card_ints {
            card_counts
                .entry(*card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        // sort the counts of a card with some check against JOKER
        let mut scores: Vec<usize> = card_counts
            .keys()
            .map(|key| {
                // made cards be a 1-indexed version of CARD_CHARS
                if joker && *key == 1 {
                    // don't count joker as a score
                    // only increases the strength of the rest of hand
                    0
                } else {
                    card_counts.get(key).unwrap().to_owned()
                }
            })
            .sorted_by(|a, b| b.cmp(a))
            .collect();

        if joker {
            // add the number of jokers to increase power of hand
            scores[0] += *card_counts.get(&1).unwrap_or(&0);
        }

        // take the most common card count and determine what type of hand it is
        let strength = match scores.first() {
            Some(5) => HandType::FiveOfAKind,
            Some(4) => HandType::FourOfAKind,
            Some(3) => {
                if *scores.get(1).unwrap() == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            Some(2) => {
                if *scores.get(1).unwrap() == 2 {
                    HandType::TwoPairs
                } else {
                    HandType::Pair
                }
            }
            _ => HandType::HighCard,
        };
        Self {
            cards: card_ints.iter().copied().tuples().next().unwrap(),
            bet,
            strength,
        }
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq)]
enum HandType {
    HighCard = 1,
    Pair = 2,
    TwoPairs = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

fn part2(input: &str) -> u64 {
    let mut hands = Problem::from_str(input, true).hands;
    get_total_winnings(&mut hands)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_to_string("inputs/day-07.example.txt").unwrap();
        assert_eq!(part1(&input), 6440);
    }

    #[test]
    fn test_hand_parse() {
        let problem = Problem::from_str("32T3K 765", false);
        assert_eq!(
            problem,
            Problem {
                hands: vec![Hand {
                    cards: (3, 2, 10, 3, 13),
                    bet: 765,
                    strength: HandType::Pair
                }]
            }
        );
    }

    #[test]
    fn test_example_part1_parse() {
        let input = read_to_string("inputs/day-07.example.txt").unwrap();
        let problem = Problem::from_str(&input, false);
        assert_eq!(problem.hands.len(), 5);
        assert_eq!(
            problem.hands[0],
            Hand {
                cards: (3, 2, 10, 3, 13),
                bet: 765,
                strength: HandType::Pair
            }
        );
        assert_eq!(
            problem.hands[4],
            Hand {
                cards: (12, 12, 12, 11, 14),
                bet: 483,
                strength: HandType::ThreeOfAKind
            }
        );
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("inputs/day-07.example.txt").unwrap();
        assert_eq!(part2(&input), 5905);
    }
}
