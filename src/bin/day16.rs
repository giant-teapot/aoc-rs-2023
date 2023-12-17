use std::collections::HashSet;

use aoc_rs_2023::*;
use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1(input!()));
    println!("Part 2: {}", part_2(input!()));
}

fn part_1(input: &str) -> usize {
    cast_ray(&get_map(input), (0, 0, Direction::Right))
}

fn part_2(input: &str) -> usize {
    let map = get_map(input);
    let heigh = map.len() as isize;
    let width = map[0].len() as isize;

    let vertical = (0..width)
        .map(|x| {
            let from_top = cast_ray(&map, (x, 0, Direction::Down));
            let from_bottom = cast_ray(&map, (x, heigh - 1, Direction::Up));
            std::cmp::max(from_top, from_bottom)
        })
        .max()
        .unwrap();

    let horizontal = (0..heigh)
        .map(|y| {
            let from_left = cast_ray(&map, (0, y, Direction::Right));
            let from_right = cast_ray(&map, (width - 1, y, Direction::Left));
            std::cmp::max(from_left, from_right)
        })
        .max()
        .unwrap();

    std::cmp::max(vertical, horizontal)
}

type Map = Vec<Vec<char>>;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
type Ray = (isize, isize, Direction);

fn get_map(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn cast_ray(map: &Map, start: Ray) -> usize {
    let mut to_visit: Vec<Ray> = vec![start];
    let mut visited: HashSet<Ray> = HashSet::new();

    while let Some((x, y, dir)) = to_visit.pop() {
        if !in_bounds(x, y, map) || visited.contains(&(x, y, dir)) {
            continue;
        }

        visited.insert((x, y, dir));
        match map[y as usize][x as usize] {
            '.' => match dir {
                Direction::Right => to_visit.push((x + 1, y, dir)),
                Direction::Left => to_visit.push((x - 1, y, dir)),
                Direction::Down => to_visit.push((x, y + 1, dir)),
                Direction::Up => to_visit.push((x, y - 1, dir)),
            },

            '\\' => match dir {
                Direction::Right => to_visit.push((x, y + 1, Direction::Down)),
                Direction::Left => to_visit.push((x, y - 1, Direction::Up)),
                Direction::Down => to_visit.push((x + 1, y, Direction::Right)),
                Direction::Up => to_visit.push((x - 1, y, Direction::Left)),
            },

            '/' => match dir {
                Direction::Right => to_visit.push((x, y - 1, Direction::Up)),
                Direction::Left => to_visit.push((x, y + 1, Direction::Down)),
                Direction::Down => to_visit.push((x - 1, y, Direction::Left)),
                Direction::Up => to_visit.push((x + 1, y, Direction::Right)),
            },

            '-' => match dir {
                Direction::Right => to_visit.push((x + 1, y, dir)),
                Direction::Left => to_visit.push((x - 1, y, dir)),
                Direction::Down | Direction::Up => {
                    to_visit.push((x - 1, y, Direction::Left));
                    to_visit.push((x + 1, y, Direction::Right));
                }
            },

            '|' => match dir {
                Direction::Down => to_visit.push((x, y + 1, dir)),
                Direction::Up => to_visit.push((x, y - 1, dir)),
                Direction::Right | Direction::Left => {
                    to_visit.push((x, y - 1, Direction::Up));
                    to_visit.push((x, y + 1, Direction::Down));
                }
            },

            _ => unreachable!(),
        }
    }

    visited.iter().map(|&(x, y, _)| (x, y)).unique().count()
}

fn in_bounds(x: isize, y: isize, map: &Map) -> bool {
    x >= 0 && (x as usize) < map[0].len() && y >= 0 && (y as usize) < map.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample!()), 46);
        assert_eq!(part_2(sample!()), 51);
    }
}
