use aoc_rs_2023::*;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part_1(input!()));
    println!("Part 2: {}", part_2(input!()));
}

fn part_1(input: &str) -> u32 {
    let (directions, map) = parse_input(input);
    let mut nb_steps = 0_u32;
    let mut position = "AAA";

    for direction in directions.iter().cycle() {
        let &(left, right) = map.get(position).unwrap();
        position = match direction {
            Direction::Left => left,
            Direction::Right => right,
        };
        nb_steps += 1;

        if position == "ZZZ" {
            return nb_steps;
        }
    }
    unreachable!()
}

fn part_2(input: &str) -> u64 {
    let (directions, map) = parse_input(input);
    let positions = map
        .keys()
        .filter(|&k| k.ends_with('A'))
        .copied()
        .collect_vec();

    positions
        .iter()
        .map(|&start| {
            let mut position = start;
            let mut nb_steps = 0_u64;

            for direction in directions.iter().cycle() {
                let &(left, right) = map.get(position).unwrap();

                position = match direction {
                    Direction::Left => left,
                    Direction::Right => right,
                };
                nb_steps += 1;

                if position.ends_with('Z') {
                    return nb_steps;
                }
            }
            unreachable!()
        })
        .reduce(lcm)
        .unwrap()
}

fn lcm(a: u64, b: u64) -> u64 {
    fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            std::mem::swap(&mut a, &mut b);
            b %= a;
        }
        a
    }
    a * b / gcd(a, b)
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<&str, (&str, &str)>) {
    let directions = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect_vec();

    let map = input
        .lines()
        .skip(2)
        .map(|line| {
            let start = &line[0..3];
            let left_dest = &line[7..10];
            let right_dest = &line[12..15];
            (start, (left_dest, right_dest))
        })
        .collect::<HashMap<_, _>>();

    (directions, map)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample1!()), 2);
        assert_eq!(part_1(sample2!()), 6);
        assert_eq!(part_2(sample3!()), 6);
    }
}
