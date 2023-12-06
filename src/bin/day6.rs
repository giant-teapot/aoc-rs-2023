use aoc_rs_2023::*;
use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1(input!()));
    println!("Part 2: {}", part_2(input!()));
}

fn part_1(input: &str) -> u64 {
    get_input1(input)
        .iter()
        .map(|&(t, d)| get_winning_solutions(t, d))
        .map(|r| r.len())
        .product()
}

fn part_2(input: &str) -> u64 {
    let (time, distance) = get_input2(input);
    get_winning_solutions(time, distance).len()
}

fn get_input1(input: &str) -> Vec<(u64, u64)> {
    let (lines, distances) = input
        .lines()
        .map(|line| line.split(": ").collect_tuple::<(_, _)>().unwrap())
        .map(|(_, numbers)| {
            numbers
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_tuple::<(_, _)>()
        .unwrap();

    lines.into_iter().zip(distances).collect()
}

fn get_input2(input: &str) -> (u64, u64) {
    input
        .lines()
        .take(2)
        .map(|line| line.split(": ").collect_tuple::<(_, _)>().unwrap())
        .map(|(_, numbers)| numbers.split_whitespace().collect::<String>())
        .map(|number| number.parse::<u64>().unwrap())
        .collect_tuple::<(_, _)>()
        .unwrap()
}

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn len(&self) -> u64 {
        self.end - self.start
    }
}

fn get_winning_solutions(time: u64, distance: u64) -> Range {
    let delta_sqrt = f64::sqrt((time.pow(2) - 4 * distance) as f64);
    let (x1, x2) = (
        (-0.5) * (-(time as f64) + delta_sqrt),
        (-0.5) * (-(time as f64) - delta_sqrt),
    );

    Range {
        start: if x1.trunc() == x1 {
            x1 as u64 + 1
        } else {
            x1.ceil() as u64
        },
        end: if x2.trunc() == x2 {
            x2 as u64
        } else {
            x2.floor() as u64 + 1
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample!()), 288);
        assert_eq!(part_2(sample!()), 71503);
    }
}
