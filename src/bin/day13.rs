use aoc_rs_2023::*;
use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1(input!()));
    println!("Part 2: {}", part_2(input!()));
}

fn part_1(input: &str) -> u32 {
    get_maps(input).iter().map(|m| find_reflection(m, 0)).sum()
}

fn part_2(input: &str) -> u32 {
    get_maps(input).iter().map(|m| find_reflection(m, 1)).sum()
}

type Map = Vec<Vec<char>>;

fn find_reflection(map: &Map, smudges: usize) -> u32 {
    let nb_lines = map.len();
    let nb_columns = map[0].len();

    // 1) Vertical reflection
    for x in 0..(nb_columns - 1) {
        let nb_differences: usize = (0..=x)
            .rev()
            .zip(x + 1..nb_columns)
            .map(|(x1, x2)| {
                // Number of differences between columns x1 and x2
                (0..nb_lines).filter(|&y| map[y][x1] != map[y][x2]).count()
            })
            .sum();

        if nb_differences == smudges {
            return (x + 1) as u32;
        }
    }

    // 2) Horizontal reflection
    for y in 0..(nb_lines - 1) {
        let nb_differences: usize = (0..=y)
            .rev()
            .zip(y + 1..nb_lines)
            .map(|(y1, y2)| {
                // Number of differences between lines y1 and y2
                (0..nb_columns)
                    .filter(|&x| map[y1][x] != map[y2][x])
                    .count()
            })
            .sum();

        if nb_differences == smudges {
            return (y + 1) as u32 * 100;
        }
    }

    0
}

fn get_maps(input: &str) -> Vec<Map> {
    input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec()
        })
        .collect_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample!()), 405);
        assert_eq!(part_2(sample!()), 400);
    }
}
