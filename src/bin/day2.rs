use aoc_rs_2023::*;
use itertools::Itertools;
use std::{cmp, iter::zip};

fn main() {
    let input = input!();
    println!("Part 1: {}", part_1(input, &[12, 13, 14]));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str, max_rgb: &[u32; 3]) -> u32 {
    input
        .lines()
        .enumerate()
        .map(|(n, line)| (n as u32, get_max_balls(line)))
        .filter(|(_, rgb)| is_lower(rgb, max_rgb))
        .map(|(n, _)| n + 1)
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(get_max_balls)
        .map(|rgb| rgb.iter().product::<u32>())
        .sum()
}

fn get_max_balls(line: &str) -> [u32; 3] {
    let (_, game) = line.split(": ").collect_tuple().unwrap();
    let draws = game.split("; ").collect_vec();

    let (mut r, mut g, mut b) = (0_u32, 0_u32, 0_u32);
    for draw in draws.iter() {
        for (n, color_str) in draw.split_whitespace().tuples::<(_, _)>() {
            let n = n.parse().unwrap();
            if color_str.contains("red") {
                r = cmp::max(r, n);
            } else if color_str.contains("green") {
                g = cmp::max(g, n);
            } else if color_str.contains("blue") {
                b = cmp::max(b, n);
            }
        }
    }

    [r, g, b]
}

fn is_lower(rgb: &[u32; 3], max_rbg: &[u32; 3]) -> bool {
    zip(rgb, max_rbg).all(|(&a, &b)| a <= b)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample!(), &[12, 13, 14]), 8);
        assert_eq!(part_2(sample!()), 2286);
    }
}
