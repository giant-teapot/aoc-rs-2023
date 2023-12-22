use std::collections::HashSet;

use aoc_rs_2023::*;
use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1(input!()));
}

fn part_1(input: &str) -> usize {
    let map = parse_input(input);
    let start = find_start(&map).unwrap();
    positions_at_range(&map, start, 64)
}

fn positions_at_range(map: &Map, start: (isize, isize), range: usize) -> usize {
    let mut positions = HashSet::from([start]);
    let mut to_visit = HashSet::new();

    let height = map.len();
    let width = map[0].len();

    for _ in 0..range {
        to_visit.clear();

        for &(x, y) in &positions {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next_x = x + dx;
                let next_y = y + dy;

                match map[next_y.unsigned_abs() % height][next_x.unsigned_abs() % width] {
                    '#' => continue,
                    '.' | 'S' => {
                        to_visit.insert((next_x, next_y));
                    }
                    _ => unreachable!(),
                }
            }
        }
        (positions, to_visit) = (to_visit, positions);
    }
    positions.len()
}

type Map = Vec<Vec<char>>;

fn parse_input(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn find_start(map: &Map) -> Option<(isize, isize)> {
    for (y, line) in map.iter().enumerate() {
        if let Some(x) = line.iter().position(|&c| c == 'S') {
            return Some((x as isize, y as isize));
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        let map = parse_input(sample!());
        let start = find_start(&map).unwrap();

        assert_eq!(positions_at_range(&map, start, 6), 16);
    }
}
