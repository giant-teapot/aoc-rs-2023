use std::collections::HashMap;

use aoc_rs_2023::*;
use itertools::{partition, Itertools};

fn main() {
    println!("Part 1: {}", part_1(input!()));
    println!("Part 2: {}", part_2(input!()));
}

fn part_1(input: &str) -> u32 {
    let (rules, ratings) = parse_input(input);

    ratings
        .iter()
        .filter(|r| accepted(r, &rules))
        .map(|&Rating { x, m, a, s }| x + m + a + s)
        .sum()
}

fn part_2(input: &str) -> u64 {
    let initial_ranges = RatingRange::new(1, 4000);
    let (rules, _) = parse_input(input);

    count_accepted_combinations(initial_ranges, "in", &rules)
}

fn accepted(rating: &Rating, rules: &RulesMap) -> bool {
    let mut next_id = "in";

    while let Some(ruleset) = rules.get(next_id) {
        for rule in ruleset {
            let (outcome, target) =
                rule.split_once(':')
                    .map_or((true, rule.as_str()), |(test, target_node)| {
                        let field_value = match &test[0..1] {
                            "x" => rating.x,
                            "m" => rating.m,
                            "a" => rating.a,
                            "s" => rating.s,
                            _ => unreachable!(),
                        };
                        let target_value = test[2..].parse::<u32>().unwrap();

                        let outcome = match &test[1..2] {
                            ">" => field_value > target_value,
                            "<" => field_value < target_value,
                            _ => unreachable!(),
                        };

                        (outcome, target_node)
                    });

            match (outcome, target) {
                (false, _) => continue,
                (true, "A") => return true,
                (true, "R") => return false,
                (true, x) => {
                    next_id = x;
                    break;
                }
            }
        }
    }

    false
}

fn count_accepted_combinations(
    current_ranges: RatingRange,
    rule_id: &str,
    rules: &RulesMap,
) -> u64 {
    match rule_id {
        "A" => current_ranges.nb_combinations(),
        "R" => 0,
        id => {
            let node_rules = rules.get(id).unwrap();
            let mut nb_combinations = 0;
            let mut remaining_ranges = current_ranges;

            let (final_rule_id, filter_rules) = node_rules.split_last().unwrap();
            for rule in filter_rules {
                let (test, next_rule_id) = rule.as_str().split_once(':').unwrap();
                let target_value = test[2..].parse::<u32>().unwrap();
                let field = &test[0..1];

                let mut filtered_range = remaining_ranges.clone();
                let partition_index = partition(remaining_ranges.get_range(field), |&x| {
                    if &test[1..2] == "<" {
                        x < target_value
                    } else {
                        x > target_value
                    }
                });

                *filtered_range.get_range(field) = remaining_ranges.get_range(field)
                    [..partition_index]
                    .iter()
                    .cloned()
                    .collect_vec();
                *remaining_ranges.get_range(field) = remaining_ranges.get_range(field)
                    [partition_index..]
                    .iter()
                    .cloned()
                    .collect_vec();

                nb_combinations += count_accepted_combinations(filtered_range, next_rule_id, rules)
            }

            nb_combinations + count_accepted_combinations(remaining_ranges, final_rule_id, rules)
        }
    }
}

struct Rating {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Clone)]
struct RatingRange {
    // Should look for a better way to implement ranges here:
    x: Vec<u32>,
    m: Vec<u32>,
    a: Vec<u32>,
    s: Vec<u32>,
}

impl RatingRange {
    fn new(min: u32, max: u32) -> Self {
        Self {
            x: (min..=max).collect_vec(),
            m: (min..=max).collect_vec(),
            a: (min..=max).collect_vec(),
            s: (min..=max).collect_vec(),
        }
    }

    fn get_range(&mut self, letter: &str) -> &mut Vec<u32> {
        match letter {
            "x" => &mut self.x,
            "m" => &mut self.m,
            "a" => &mut self.a,
            "s" => &mut self.s,
            _ => unreachable!(),
        }
    }

    fn nb_combinations(&self) -> u64 {
        self.x.len() as u64 * self.m.len() as u64 * self.a.len() as u64 * self.s.len() as u64
    }
}

type RulesMap = HashMap<String, Vec<String>>;

fn parse_input(input: &str) -> (RulesMap, Vec<Rating>) {
    let (rules, ratings) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            let (id, node_rules) = line.split_once('{').unwrap();
            let node_rules = node_rules
                .trim_end_matches('}')
                .split(',')
                .map(str::to_string)
                .collect_vec();

            (id.to_string(), node_rules)
        })
        .collect();

    let ratings = ratings
        .lines()
        .map(|line| {
            let (x, m, a, s) = line[1..line.len() - 1]
                .split(',')
                .map(|x| {
                    let (_, rating) = x.rsplit_once('=').unwrap();
                    rating.parse::<u32>().unwrap()
                })
                .collect_tuple()
                .unwrap();

            Rating { x, m, a, s }
        })
        .collect_vec();

    (rules, ratings)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample!()), 19114);
        assert_eq!(part_2(sample!()), 167409079868000);
    }
}
