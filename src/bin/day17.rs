use std::collections::{HashMap, VecDeque};

use aoc_rs_2023::*;
use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1(input!()));
    println!("Part 2: {}", part_2(input!()));
}

fn part_1(input: &str) -> u32 {
    get_shortest_path(&get_map(input), 1, 3)
}

fn part_2(input: &str) -> u32 {
    get_shortest_path(&get_map(input), 4, 10)
}

type Map = Vec<Vec<u8>>;
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn forward(&self, position: (usize, usize), map: &Map) -> Option<(usize, usize)> {
        let (x, y) = (position.0 as isize, position.1 as isize);
        let (next_x, next_y) = match &self {
            Self::Up => (x, y - 1),
            Self::Down => (x, y + 1),
            Self::Left => (x - 1, y),
            Self::Right => (x + 1, y),
        };

        if in_bounds(next_x, next_y, map) {
            Some((next_x as usize, next_y as usize))
        } else {
            None
        }
    }

    fn left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    fn right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Crucible {
    position: (usize, usize),
    direction: Direction,
    since_last_turn: usize,
}

impl Crucible {
    fn new(position: (usize, usize), direction: Direction, since_last_turn: usize) -> Self {
        Self {
            position,
            direction,
            since_last_turn,
        }
    }
}

fn get_map(input: &str) -> Map {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

// FIXME: Complete and painfully slow
fn get_shortest_path(map: &Map, min_dist_forward: usize, max_before_turn: usize) -> u32 {
    assert!(min_dist_forward < max_before_turn);
    let mut to_visit = VecDeque::new();
    let mut visited = HashMap::new();
    let mut total_cost = u32::MAX;

    // Start: top-left corner (either right or down)
    to_visit.extend([
        (Crucible::new((0, 0), Direction::Right, 1), 0),
        (Crucible::new((0, 0), Direction::Down, 1), 0),
    ]);
    visited.extend(to_visit.iter().cloned());

    // End: bottom-right corner
    let end = (map[0].len() - 1, map.len() - 1);

    while let Some((current, cost)) = to_visit.pop_front() {
        let mut push_next_step = |facing: Direction, turn_distance: usize| {
            if let Some(pos) = facing.forward(current.position, map) {
                let state = Crucible::new(pos, facing, turn_distance);

                let (x, y) = pos;
                let cost = cost + map[y][x] as u32;

                if !visited.contains_key(&state) || visited.get(&state).unwrap() > &cost {
                    to_visit.push_back((state, cost));
                    visited.insert(state, cost);
                }
            }
        };

        if current.position == end && current.since_last_turn >= min_dist_forward {
            total_cost = std::cmp::min(total_cost, cost);
            continue;
        }

        if current.since_last_turn < max_before_turn {
            // Keep going forward
            push_next_step(current.direction, current.since_last_turn + 1);
        }

        if current.since_last_turn >= min_dist_forward {
            // Can't keep straight for too long, must turn
            push_next_step(current.direction.left(), 1);
            push_next_step(current.direction.right(), 1);
        }
    }

    total_cost
}

fn in_bounds(x: isize, y: isize, map: &Map) -> bool {
    x >= 0 && (x as usize) < map[0].len() && y >= 0 && (y as usize) < map.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample!()), 102);
        assert_eq!(part_2(sample!()), 94);
    }
}
