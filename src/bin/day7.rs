use aoc_rs_2023::*;
use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1(input!()));
    println!("Part 2: {}", part_2(input!()));
}

fn part_1(input: &str) -> u32 {
    get_score(get_input(input, Ruleset::Part1))
}

fn part_2(input: &str) -> u32 {
    get_score(get_input(input, Ruleset::Part2))
}

fn get_score(hands_and_bids: Vec<(Hand, u32)>) -> u32 {
    hands_and_bids
        .into_iter()
        .map(|(hand, bid)| {
            let hand_type = get_hand_type(&hand);
            let key = (hand_type, hand[0], hand[1], hand[2], hand[3], hand[4]);

            (key, bid, hand)
        })
        .sorted_by_key(|&(key, _, _)| key)
        .enumerate()
        .map(|(i, (_, bid, _))| (i + 1) as u32 * bid)
        .sum()
}

fn get_hand_type(hand: &Hand) -> u32 {
    let (jokers, other_cards): (Vec<_>, Vec<_>) = hand.iter().partition(|&&x| x == 0);

    let mut card_groups = other_cards
        .into_iter()
        .filter(|&&x| x != 0)
        .sorted()
        .dedup_with_count()
        .map(|(count, _)| count)
        .sorted()
        .rev()
        .take(2)
        .collect_tuple::<(_, _)>()
        .unwrap();

    card_groups.0 += jokers.len();
    match card_groups {
        (5, _) => 7,
        (4, _) => 6,
        (3, 2) => 5,
        (3, 1) => 4,
        (2, 2) => 3,
        (2, 1) => 2,
        (1, _) => 1,
        _ => unreachable!(),
    }
}

type Hand = [u32; 5];

fn get_input(input: &str, ruleset: Ruleset) -> Vec<(Hand, u32)> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect_tuple::<(_, _)>().unwrap())
        .map(|(hand, bid)| (parse_hand(hand, ruleset), bid.parse::<u32>().unwrap()))
        .collect_vec()
}

#[derive(PartialEq, Copy, Clone)]
enum Ruleset {
    Part1,
    Part2,
}

fn parse_hand(cards: &str, ruleset: Ruleset) -> Hand {
    cards
        .chars()
        .take(5)
        .map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => {
                if ruleset == Ruleset::Part1 {
                    11
                } else {
                    0
                }
            }
            'T' => 10,
            _ => c.to_string().parse::<u32>().unwrap(),
        })
        .collect_vec()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample!()), 6440);
        assert_eq!(part_2(sample!()), 5905);
    }
}
