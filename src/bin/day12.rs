use aoc_rs_2023::*;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part_1(input!()));
    println!("Part 2: {}", part_2(input!()));
}

fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (map_str, groups_str) = line.split_once(' ').unwrap();
            let groups_size = groups_str
                .split(',')
                .map(|n| n.parse::<u64>().unwrap())
                .collect_vec();
            let map = map_str.chars().collect_vec();

            count_arrangements(map.as_slice(), groups_size.as_slice())
        })
        .sum()
}

fn part_2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (map_str, groups_str) = line.split_once(' ').unwrap();
            let group_size = groups_str
                .split(',')
                .map(|n| n.parse::<u64>().unwrap())
                .collect_vec();

            let map = (0..5).map(|_| map_str).join("?").chars().collect_vec();
            let groups_size = (0..5)
                .flat_map(|_| group_size.iter().copied())
                .collect_vec();

            count_arrangements(map.as_slice(), groups_size.as_slice())
        })
        .sum()
}

fn count_arrangements(map: &[char], groups: &[u64]) -> u64 {
    count_arrangements_internal(map, groups, 0, &mut HashMap::new())
}

fn count_arrangements_internal(
    map: &[char],
    groups: &[u64],
    staged: u64,
    cache: &mut HashMap<(u64, u64, u64), u64>,
) -> u64 {
    if map.is_empty() {
        // No more space available on the map.
        // Return 1 if there is enough room to squeeze the last group.
        return match (staged, groups.len()) {
            (0, 0) => 1,
            (n, 1) => {
                // One last group to put...
                if n == groups[0] {
                    1 // ... and just enough room.
                } else {
                    0 // ... and not enough room.
                }
            }
            _ => 0,
        };
    }
    if staged > 0 && groups.is_empty() {
        // No more groups to put on the map.
        return 0;
    }
    assert!(
        if staged > 0 { !groups.is_empty() } else { true },
        "The group cannot be empty past this point if there are staged elements."
    );

    let cache_key = (map.len() as u64, groups.len() as u64, staged);
    if let Some(cached_result) = cache.get(&cache_key) {
        return *cached_result;
    }

    let result = match map[0] {
        // -----------------------------------------------------------------------------------------
        // (.) are empty locations. Staging will be reset as nothing can be found there.
        //
        '.' => {
            if staged == 0 {
                // Nothing staged: we can ignore and proceed.
                count_arrangements_internal(&map[1..], groups, 0, cache)
            } else if staged == groups[0] {
                // Staging (contiguous previous cell) are enough to fit the current group!
                // Reset staging and advance.
                count_arrangements_internal(&map[1..], &groups[1..], 0, cache)
            } else {
                // Staging does not fit!
                // No more answers to be found here. Backtrack.
                0
            }
        }

        // -----------------------------------------------------------------------------------------
        // (#) mark actual spot locations and must be staged.
        //
        '#' => {
            // Stage one letter and advance.
            count_arrangements_internal(&map[1..], groups, staged + 1, cache)
        }

        // -----------------------------------------------------------------------------------------
        // (?) characters can be staged or left alone.
        //
        '?' => {
            if staged == 0 {
                // Choices character with no staged items: consider both alternatives
                // (1) staging
                // (2) not staging
                count_arrangements_internal(&map[1..], groups, 1, cache) // (1)
                + count_arrangements_internal(&map[1..], groups, 0, cache) // (2)
            } else if staged == groups[0] {
                // Staging area is enough to fit a group! Consider:
                // (1) taking that element out and proceeding with an empty stage
                // (2) not taking that group out and proceeding (with an increased staging)
                count_arrangements_internal(&map[1..], &groups[1..], 0, cache) // (1)
                + count_arrangements_internal(&map[1..], groups, staged + 1, cache)
            } else {
                // Staging is not empty: we have to keep on staging.
                count_arrangements_internal(&map[1..], groups, staged + 1, cache)
            }
        }

        _ => unreachable!(),
    };

    cache.insert(cache_key, result);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample!()), 21);
        assert_eq!(part_2(sample!()), 525152);
    }
}
