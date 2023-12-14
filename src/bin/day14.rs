use std::collections::HashMap;

use aoc_rs_2023::*;
use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1(input!()));
    println!("Part 2: {}", part_2(input!()));
}

fn part_1(input: &str) -> usize {
    let mut map = get_map(input);
    slide_boulders_north(&mut map);
    get_load(&map)
}

fn part_2(input: &str) -> usize {
    let mut map = get_map(input);
    let mut cache = HashMap::new();

    for cycle_nb in 1..=1_000_000_000 {
        map = cycle(map);

        if let Some(last_seen) = cache.insert(map.clone(), cycle_nb) {
            if (1_000_000_000 - cycle_nb) % (cycle_nb - last_seen) == 0 {
                return get_load(&map);
            }
        }
    }

    get_load(&map)
}

type Map = Vec<Vec<char>>;

fn get_map(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn slide_boulders_north(map: &mut Map) {
    for x in 0..map[0].len() {
        let mut last_free_spot = 0;
        for y in 0..map.len() {
            match map[y][x] {
                'O' => {
                    (map[y][x], map[last_free_spot][x]) = (map[last_free_spot][x], map[y][x]);
                    last_free_spot += 1;
                }
                '#' => {
                    last_free_spot = y + 1;
                }
                _ => {}
            }
        }
    }
}

fn get_load(map: &Map) -> usize {
    map.iter()
        .rev()
        .enumerate()
        .map(|(n, line)| line.iter().filter(|&&c| c == 'O').count() * (n + 1))
        .sum()
}

fn cycle(mut map: Map) -> Map {
    for _ in 0..4 {
        slide_boulders_north(&mut map);
        map = rotate_right(&map);
    }
    map
}

fn rotate_right(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (original_height, original_width) = (map.len(), map[0].len());
    let mut rotated_map = vec![vec!['.'; original_width]; original_height];

    for y in 0..original_height {
        for x in 0..original_width {
            rotated_map[x][original_height - 1 - y] = map[y][x];
        }
    }
    rotated_map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample!()), 136);
        assert_eq!(part_2(sample!()), 64);
    }
}
