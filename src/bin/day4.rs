use std::{collections::BTreeSet, iter};

use aoc_rs_2023::*;
use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1(input!()));
    println!("Part 2: {}", part_2(input!()));
}

fn part_1(input: &str) -> u32 {
    get_card_scores(input)
        .map(|n| if n > 0 { 2_u32.pow(n - 1) } else { 0 })
        .sum::<u32>()
}

fn part_2(input: &str) -> u32 {
    let mut cards = get_card_scores(input)
        .zip(iter::repeat(1_u32))
        .collect_vec();

    for i in 0..cards.len() {
        let (score, copies) = cards[i];
        for extra_card in 1..=score as usize {
            if i + extra_card >= cards.len() {
                break;
            }
            cards[i + extra_card].1 += copies;
        }
    }

    cards.iter().map(|(_, nb)| nb).sum()
}

fn get_card_scores(input: &str) -> impl Iterator<Item = u32> + '_ {
    input
        .lines()
        .map(|line| line.split(": ").collect_tuple::<(_, _)>().unwrap())
        .map(|(_, numbers)| numbers.split(" | ").collect_tuple::<(_, _)>().unwrap())
        .map(|(win_nb, card_nb)| {
            let win_nb =
                BTreeSet::from_iter(win_nb.split_whitespace().map(|x| x.parse::<u32>().unwrap()));
            let card_nb = BTreeSet::from_iter(
                card_nb
                    .split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap()),
            );
            win_nb.intersection(&card_nb).count() as u32
        })
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample!()), 13);
        assert_eq!(part_2(sample!()), 30);
    }
}
