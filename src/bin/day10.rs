use aoc_rs_2023::*;
use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1(input!()));
}

fn part_1(input: &str) -> u32 {
    let map = get_map(input);
    let start = find_start(&map).unwrap();

    let mut circumference = 1;
    let mut previous = start;
    let mut current = next(start, previous, &map).unwrap();

    while current != start {
        let tmp = next(current, previous, &map);
        previous = current;
        current = tmp.unwrap();
        circumference += 1;
    }

    circumference / 2
}

type Map = Vec<Vec<char>>;

fn get_map(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn find_start(map: &Map) -> Option<(usize, usize)> {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'S' {
                return Some((x, y));
            }
        }
    }
    None
}

fn next(current: (usize, usize), previous: (usize, usize), map: &Map) -> Option<(usize, usize)> {
    let (x, y) = current;
    let pipe = &map[y][x];

    let directions = match pipe {
        '|' => [(0, -1), (0, 1)],
        '-' => [(-1, 0), (1, 0)],
        'L' => [(0, -1), (1, 0)],
        'J' => [(0, -1), (-1, 0)],
        '7' => [(-1, 0), (0, 1)],
        'F' => [(1, 0), (0, 1)],

        'S' => return get_valid_start_directions(current, map),
        _ => unreachable!(),
    };

    let xs = directions
        .iter()
        .map(|&(dx, dy)| (x as i32 + dx, y as i32 + dy))
        .filter(|coord| in_bounds(coord, map))
        .map(|(x, y)| (x as usize, y as usize))
        .filter(|&coord| coord != previous)
        .collect_vec();

    println!("@({},{})::{} -> possible = {:?}", x, y, pipe, xs);
    xs.into_iter().next()
}

fn get_valid_start_directions(current: (usize, usize), map: &Map) -> Option<(usize, usize)> {
    #[derive(Copy, Clone)]
    enum Dir {
        North,
        South,
        West,
        East,
    }

    let (x, y) = current;
    [
        (Dir::North, (0, -1)),
        (Dir::South, (0, 1)),
        (Dir::West, (-1, 0)),
        (Dir::East, (1, 0)),
    ]
    .iter()
    .map(|(dir, (dx, dy))| (dir, (dx + x as i32, dy + y as i32)))
    .filter(|(_, coord)| in_bounds(coord, map))
    .map(|(&dir, (x, y))| (dir, (x as usize, y as usize)))
    .filter(|&(dir, (x, y))| {
        let pipe = map[y][x];
        match dir {
            Dir::North => ['F', '|', '7'].contains(&pipe),
            Dir::South => ['L', '|', 'J'].contains(&pipe),
            Dir::West => ['F', '-', 'L'].contains(&pipe),
            Dir::East => ['7', '-', 'J'].contains(&pipe),
        }
    })
    .map(|(_, coord)| coord)
    .next()
}

fn in_bounds(coordinates: &(i32, i32), map: &Map) -> bool {
    let &(x, y) = coordinates;

    x >= 0 && (x as usize) < map[0].len() && y >= 0 && (y as usize) < map.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample1!()), 4);
        assert_eq!(part_1(sample2!()), 8);
    }
}
