use aoc_rs_2023::*;
use itertools::Itertools;

fn main() {
    let contraption = read_input(input!());
    println!("Part 1: {}", part_1(&contraption));
    println!("Part 2: {}", part_2(&contraption));
}

fn part_1(contraption: &Contraption) -> u32 {
    let mut sum = 0;

    contraption.iter().enumerate().for_each(|(n, line)| {
        let mut col = 0;
        while col < line.len() {
            let number = get_number(&line[col..]);
            if let Some((number, nb_len)) = number {
                let end_col = col + nb_len - 1;
                if has_adjacent_symbol(n, col, end_col, contraption) {
                    sum += number;
                }

                col += nb_len;
            } else {
                col += 1;
            }
        }
    });

    sum
}

fn part_2(contraption: &Contraption) -> u32 {
    let mut sum = 0;

    for line in 0..contraption.len() {
        for column in 0..contraption[0].len() {
            if contraption[line].chars().nth(column).unwrap() == '*' {
                let adjacent_numbers = get_adjacent_numbers(line, column, contraption);

                if adjacent_numbers.len() == 2 {
                    sum += adjacent_numbers.iter().product::<u32>();
                }
            }
        }
    }

    sum
}

type Contraption = Vec<String>;

fn read_input(input: &str) -> Contraption {
    input
        .lines()
        .map(|line| line.chars().collect::<String>())
        .collect_vec()
}

fn get_number(line: &str) -> Option<(u32, usize)> {
    if !line.chars().next().unwrap().is_ascii_alphanumeric() {
        return None;
    }

    let last_digit = line
        .chars()
        .find_position(|c| !c.is_ascii_alphanumeric())
        .map_or(line.len(), |(n, _)| n);

    match last_digit {
        0 => None,
        n => Some((line[..last_digit].parse().unwrap(), n)),
    }
}

fn has_adjacent_symbol(
    line: usize,
    col_start: usize,
    col_end: usize,
    contraption: &Contraption,
) -> bool {
    for l in (line as isize - 1)..=(line as isize + 1) {
        for column in (col_start as isize - 1)..=(col_end as isize + 1) {
            // Skip inside the number
            if (l == line as isize) && ((col_start as isize)..=(col_end as isize)).contains(&column)
            {
                continue;
            }

            let has_adjacent_symbol = contraption
                .get(l as usize)
                .map(|l| l.chars())
                .and_then(|mut columns| columns.nth(column as usize))
                .map_or(false, |c| c != '.');

            if has_adjacent_symbol {
                return true;
            }
        }
    }
    false
}

fn get_adjacent_numbers(line: usize, column: usize, contraption: &Contraption) -> Vec<u32> {
    let mut adjacent_numbers = Vec::new();

    let max_column = contraption[0].len();
    for number_line in (line as isize - 1)..=(line as isize + 1) {
        let mut number_start_col = 0;
        while number_start_col <= column + 1 {
            // Out of bounds
            if number_line < 0
                || number_line >= contraption.len() as isize
                || number_start_col >= max_column
            {
                continue;
            }

            let str_to_scan = &contraption[number_line as usize][number_start_col..];
            let number = get_number(str_to_scan);

            if let Some((n, len)) = number {
                if ((number_start_col as isize - 1)..=((number_start_col + len) as isize))
                    .contains(&(column as isize))
                {
                    adjacent_numbers.push(n);
                }
                number_start_col += len;
                continue;
            }

            number_start_col += 1;
        }
    }

    adjacent_numbers
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_sample() {
        let contraption = read_input(sample!());
        assert_eq!(part_1(&contraption), 4361);
        assert_eq!(part_2(&contraption), 467835);
    }
}
