use aoc_rs_2023::*;
use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1(input!()));
    println!("Part 2: {}", part_2(input!()));
}

fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .collect_vec()
        })
        .map(predict_future_value)
        .sum()
}

fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .collect_vec()
        })
        .map(predict_past_value)
        .sum()
}

fn predict_future_value(history: Vec<i32>) -> i32 {
    let derivatives = get_derivatives(history);
    derivatives
        .iter()
        .map(|series| series.last().unwrap())
        .sum()
}

fn predict_past_value(history: Vec<i32>) -> i32 {
    let derivatives = get_derivatives(history);
    derivatives
        .iter()
        .map(|series| series.first().unwrap())
        .rev()
        .fold(0, |acc, &n| -acc + n)
}

fn get_derivatives(history: Vec<i32>) -> Vec<Vec<i32>> {
    let mut derivatives = vec![history];
    let mut last = derivatives.last().unwrap();

    while !last.iter().all(|&x| x == 0) {
        let delta = last
            .iter()
            .tuple_windows::<(_, _)>()
            .map(|(&x, &y)| y - x)
            .collect_vec();

        derivatives.push(delta);
        last = derivatives.last().unwrap();
    }
    derivatives
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample!()), 114);
        assert_eq!(part_2(sample!()), 2);
    }
}
