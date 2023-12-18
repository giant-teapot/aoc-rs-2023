use aoc_rs_2023::*;
use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1(input!()));
    println!("Part 2: {}", part_2(input!()));
}

fn part_1(input: &str) -> i64 {
    let instructions = input
        .lines()
        .map(|line| {
            let (dir, distance_str) = line.split_whitespace().take(2).collect_tuple().unwrap();
            let distance = distance_str.parse::<i64>().unwrap();
            let (dx, dy) = match dir {
                "U" => (0, -1),
                "D" => (0, 1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => unreachable!(),
            };

            (dx, dy, distance)
        })
        .collect_vec();
    get_surface(&instructions)
}

fn part_2(input: &str) -> i64 {
    let instructions = input
        .lines()
        .map(|line| {
            let instruction = &line[line.find('#').unwrap() + 1..line.len() - 1];

            let distance = i64::from_str_radix(&instruction[0..5], 16).unwrap();
            let (dx, dy) = match &instruction[5..] {
                "0" => (1, 0),  // right
                "1" => (0, 1),  // down
                "2" => (-1, 0), // left
                "3" => (0, -1), // up
                _ => unreachable!(),
            };

            (dx, dy, distance)
        })
        .collect_vec();
    get_surface(&instructions)
}

fn get_surface(instructions: &[(i64, i64, i64)]) -> i64 {
    let (mut x, mut y) = (0_i64, 0_i64);
    let mut vertices = vec![(x, y)];
    let mut border_length = 0;

    instructions.iter().for_each(|&(dx, dy, distance)| {
        x += dx * distance;
        y += dy * distance;
        border_length += distance; // manhattan_distance(previous, (x, y));
        vertices.push((x, y));
    });

    // As per Pick's theorem
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    let area_inside = shoelace_area(&vertices) - (border_length / 2) + 1;
    border_length + area_inside
}

fn shoelace_area(vertices: &[(i64, i64)]) -> i64 {
    // As per the Gauss's "shoelace" area formula:
    // https://en.wikipedia.org/wiki/Shoelace_formula
    vertices
        .iter()
        .circular_tuple_windows()
        .map(|(a, b)| (a.0) * (b.1) - (b.0) * (a.1))
        .sum::<i64>()
        .abs()
        / 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample!()), 62);
        assert_eq!(part_2(sample!()), 952408144115);
    }
}
