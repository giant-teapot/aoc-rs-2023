use aoc_rs_2023::*;
use itertools::Itertools;

fn main() {
    let input = input!();
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u32 {
    input.lines().map(get_number).sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(replace_all_letter_digits)
        .map(|s| get_number(s.as_str()))
        .sum()
}

fn get_number(line: &str) -> u32 {
    let mut digits = line.chars().filter(|x| x.is_numeric());

    let a = digits.next().unwrap().to_digit(10).unwrap();
    let b = digits.next_back().map_or(a, |d| d.to_digit(10).unwrap());

    a * 10 + b
}

fn replace_all_letter_digits(line: &str) -> String {
    // Hey, who said it had to be efficient? ¯\_(ツ)_/¯
    let digits_strings = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let get_matches = |line: &str| {
        digits_strings
            .iter()
            .flat_map(|&(s, d)| line.match_indices(s).map(move |(pos, _)| (d, pos)))
            .sorted_by_key(|&(_, pos)| pos)
            .collect::<Vec<_>>()
    };

    let mut line_mut = String::from(line);
    let matches = get_matches(line);
    let mut it_matches = matches.iter();

    if let Some(&(d1, pos1)) = it_matches.next() {
        line_mut.insert_str(pos1, d1);

        if let Some(&(d2, pos2)) = it_matches.next_back() {
            line_mut.insert_str(pos2 + d1.len(), d2);
        }
    }

    line_mut
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample1!()), 142);
        assert_eq!(part_2(sample2!()), 281);
    }
}
