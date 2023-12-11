use aoc_rs_2023::*;
use itertools::Itertools;
use std::cmp;
use std::collections::HashSet;

fn main() {
    println!("Part 1: {}", part_1(input!()));
    println!("Part 2: {}", part_2(input!()));
}

fn part_1(input: &str) -> usize {
    let universe = parse_universe(input);
    distances(&universe, 2)
}

fn part_2(input: &str) -> usize {
    let universe = parse_universe(input);
    distances(&universe, 1000000)
}

struct Universe {
    galaxies: HashSet<(usize, usize)>,
    empty_lines: Vec<usize>,
    empty_columns: Vec<usize>,
}

fn parse_universe(input: &str) -> Universe {
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect::<HashSet<_>>();

    let max_x = *galaxies.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *galaxies.iter().map(|(_, y)| y).max().unwrap();

    let empty_lines = (0..max_y)
        .filter(|&y| !galaxies.iter().any(|&(_, galaxy)| y == galaxy))
        .collect_vec();

    let empty_columns = (0..max_x)
        .filter(|&x| !galaxies.iter().any(|&(galaxy, _)| x == galaxy))
        .collect_vec();

    Universe {
        galaxies,
        empty_lines,
        empty_columns,
    }
}

fn distances(universe: &Universe, growth_factor: usize) -> usize {
    let mut distances = 0;

    for (a, b) in universe.galaxies.iter().tuple_combinations() {
        let (x_a, y_a) = *a;
        let (x_b, y_b) = *b;

        // Compute expansion between the two galaxies
        let expand_x = universe
            .empty_columns
            .iter()
            .skip_while(|&&x| x <= cmp::min(x_a, x_b))
            .take_while(|&&x| x < cmp::max(x_a, x_b))
            .count()
            * (growth_factor - 1);
        let expand_y = universe
            .empty_lines
            .iter()
            .skip_while(|&&y| y <= cmp::min(y_a, y_b))
            .take_while(|&&y| y < cmp::max(y_a, y_b))
            .count()
            * (growth_factor - 1);

        distances += x_a.abs_diff(x_b) + (expand_x) + y_a.abs_diff(y_b) + (expand_y);
    }

    distances
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample!()), 374);

        let universe = parse_universe(sample!());
        assert_eq!(distances(&universe, 10), 1030);
        assert_eq!(distances(&universe, 100), 8410);
    }
}
