use std::collections::{HashMap, HashSet};

use aoc_rs_2023::*;
use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1(input!()));
    println!("Part 2: {}", part_2(input!()));
}

fn part_1(input: &str) -> usize {
    let (map, start, destination) = parse_input(input);
    let adjacency = to_adjacency_map(&map, false);

    find_longest_hike(&adjacency, &start, &destination, &HashSet::new()).unwrap()
}

fn part_2(input: &str) -> usize {
    let (map, start, destination) = parse_input(input);
    let adjacency = to_adjacency_map(&map, true);

    find_longest_hike(&adjacency, &start, &destination, &HashSet::new()).unwrap()
}

fn find_longest_hike(
    adjacency: &AdjacencyMap,
    start: &(usize, usize),
    destination: &(usize, usize),
    visited: &HashSet<(usize, usize)>,
) -> Option<usize> {
    if *start == *destination {
        return Some(0);
    }

    let Some(neighbours) = adjacency.get(start) else {
        return None;
    };

    neighbours
        .iter()
        .filter(|&&(next_x, next_y, _)| !visited.contains(&(next_x, next_y)))
        .filter_map(|&(next_x, next_y, dist_to_next)| {
            let next = (next_x, next_y);
            let mut next_visited = visited.clone();
            next_visited.insert(next);

            let hike = find_longest_hike(adjacency, &next, destination, &next_visited);
            hike.map(|d| d + dist_to_next)
        })
        .max()
}

type Map = Vec<Vec<char>>;
type Position = (usize, usize);

fn get_tile(map: &Map, x: usize, y: usize) -> Option<char> {
    if x < map[0].len() && y < map.len() {
        Some(map[y][x])
    } else {
        None
    }
}

fn parse_input(input: &str) -> (Map, Position, Position) {
    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let start = (map[0].iter().position(|&c| c == '.').unwrap(), 0);
    let destination = (
        map.last().unwrap().iter().position(|&c| c == '.').unwrap(),
        map.len() - 1,
    );

    (map, start, destination)
}

type AdjacencyMap = HashMap<Position, Vec<(usize, usize, usize)>>;

fn to_adjacency_map(map: &Map, can_hike: bool) -> AdjacencyMap {
    let mut adjacency = HashMap::new();
    let all_directions = [(-1, 0), (1, 0), (0, -1), (0, 1)].as_slice();

    // Step 1: build the adjacency lists
    for (x, y) in (0..map[0].len()).cartesian_product(0..map.len()) {
        let directions = match map[y][x] {
            '#' => continue,
            _ if can_hike => all_directions,
            '.' => all_directions,
            '^' => [(0, -1)].as_slice(),
            '>' => [(1, 0)].as_slice(),
            'v' => [(0, 1)].as_slice(),
            '<' => [(-1, 0)].as_slice(),
            _ => unreachable!(),
        };

        let neighbours: &mut Vec<(usize, usize, usize)> = adjacency.entry((x, y)).or_default();
        for (dx, dy) in directions {
            let x2 = (x as isize + dx) as usize;
            let y2 = (y as isize + dy) as usize;

            let Some(tile) = get_tile(map, x2, y2) else {
                continue;
            };
            if tile != '#' {
                // Note: all edges have a length of 1 (immediate neighbours)
                neighbours.push((x2, y2, 1));
            }
        }
    }

    // Step 2: collapse corridors
    let tiles_in_corridors = adjacency
        .iter()
        .filter_map(|(&tile, neighbours)| {
            if neighbours.len() == 2 {
                Some(tile)
            } else {
                None
            }
        })
        .collect_vec();

    for (x, y) in tiles_in_corridors {
        let Some(neighbours) = adjacency.remove(&(x, y)) else {
            continue;
        };
        assert!(neighbours.len() == 2);

        let (left_x, left_y, left_dist) = neighbours[0];
        let (right_x, right_y, right_dist) = neighbours[1];

        // Link tiles "left -> right"
        let left_neighbours = adjacency.get_mut(&(left_x, left_y)).unwrap();
        if let Some(to_replace) = left_neighbours
            .iter_mut()
            .find(|&&mut (neighbour_x, neighbour_y, _)| (neighbour_x, neighbour_y) == (x, y))
        {
            *to_replace = (right_x, right_y, left_dist + right_dist);
        }

        // Link tiles "right -> left"
        let right_tile = adjacency.get_mut(&(right_x, right_y)).unwrap();
        if let Some(to_replace) = right_tile
            .iter_mut()
            .find(|&&mut (neighbour_x, neighbour_y, _)| (neighbour_x, neighbour_y) == (x, y))
        {
            *to_replace = (left_x, left_y, left_dist + right_dist);
        }
    }

    adjacency
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample!()), 94);
        assert_eq!(part_2(sample!()), 154);
    }
}
