use aoc_rs_2023::*;
use itertools::Itertools;
use std::cmp;
use std::ops::Range;

fn main() {
    println!("Part 1: {}", part_1(input!()));
    println!("Part 2: {}", part_2(input!()));
}

fn part_1(input: &str) -> u64 {
    let Input { seeds, mappings } = get_input(input);

    seeds
        .iter()
        .map(|&seed| mappings.iter().fold(seed, apply_to_seed))
        .min()
        .unwrap()
}

fn part_2(input: &str) -> u64 {
    let Input { seeds, mappings } = get_input(input);
    let mut seed_ranges: Vec<Range<u64>> = seeds
        .iter()
        .tuples::<(_, _)>()
        .map(|(&from, &len)| Range {
            start: from,
            end: from + len,
        })
        .collect_vec();

    for mapping in mappings {
        seed_ranges = seed_ranges
            .iter()
            .flat_map(|r| apply_to_range(r, &mapping).into_iter())
            .collect();
    }

    seed_ranges.iter().map(|r| r.start).min().unwrap()
}

struct Input {
    seeds: Vec<u64>,
    mappings: Vec<Vec<(u64, u64, u64)>>,
}

fn get_input(input: &str) -> Input {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split(": ")
        .collect_tuple::<(_, _)>()
        .map(|(_, positions)| {
            positions
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect_vec()
        })
        .unwrap();

    let mut mappings: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            mappings.push(Vec::new());
            lines.next();
            continue;
        }

        let range = line
            .split_whitespace()
            .map(str::parse::<u64>)
            .map(Result::unwrap)
            .collect_tuple::<(_, _, _)>()
            .unwrap();

        if let Some(mapping) = mappings.last_mut() {
            mapping.push(range)
        }
    }

    Input { seeds, mappings }
}

fn apply_to_seed(seed_position: u64, mapping: &Vec<(u64, u64, u64)>) -> u64 {
    for &(to, from, len) in mapping {
        if from <= seed_position && seed_position < from + len {
            return to + (seed_position - from);
        }
    }

    seed_position
}

/// Translates a whole range of positions at once through several mappings.
fn apply_to_range(range: &Range<u64>, mappings: &Vec<(u64, u64, u64)>) -> Vec<Range<u64>> {
    let mut shifted_ranges: Vec<Range<u64>> = Vec::new();
    let mut intersections: Vec<Range<u64>> = Vec::new();

    for mapping in mappings {
        let &(_, mapping_start, mapping_len) = mapping;
        let map_range = Range {
            start: mapping_start,
            end: mapping_start + mapping_len,
        };
        if let Some(intersection) = intersect(range, &map_range) {
            let shifted_range = Range {
                start: apply_mapping(intersection.start, mapping),
                end: apply_mapping(intersection.end, mapping),
            };
            shifted_ranges.push(shifted_range);
            intersections.push(intersection);
        }
    }

    let not_shifted_ranges = remove_ranges(range, intersections);
    shifted_ranges.extend(not_shifted_ranges);

    shifted_ranges
}

fn apply_mapping(position: u64, mapping: &(u64, u64, u64)) -> u64 {
    let &(to, from, _) = mapping;
    let shift = to as i64 - from as i64;
    (position as i64 + shift) as u64
}

fn intersect(a: &Range<u64>, b: &Range<u64>) -> Option<Range<u64>> {
    let start = cmp::max(a.start, b.start);
    let end = cmp::min(a.end, b.end);
    if start <= end {
        Some(Range { start, end })
    } else {
        None
    }
}

/// Removes intersections between a range and a list of other ranges.
///
/// The initial range may be split into several smaller ranges, for instance if it encompasses one
/// of the ranges to remove.
fn remove_ranges(range: &Range<u64>, mut to_remove: Vec<Range<u64>>) -> Vec<Range<u64>> {
    to_remove.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());

    let mut output_ranges: Vec<Range<u64>> = Vec::new();
    let mut pos = range.start;
    for r in to_remove.iter() {
        if r.start > pos {
            output_ranges.push(Range {
                start: pos,
                end: r.start,
            });
            pos = r.end;
            continue;
        }
        if r.end > pos {
            pos = r.end;
        }
    }
    if pos < range.end {
        output_ranges.push(Range {
            start: pos,
            end: range.end,
        });
    }
    output_ranges
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample!()), 35);
        assert_eq!(part_2(sample!()), 46);
    }
}
