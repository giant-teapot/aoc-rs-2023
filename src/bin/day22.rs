use aoc_rs_2023::*;
use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1(input!()));
    println!("Part 2: {}", part_2(input!()));
}

fn part_1(input: &str) -> usize {
    let mut bricks = parse_input(input);
    let (bricks, _) = let_them_fall(&mut bricks);

    bricks
        .iter()
        .map(|b| get_nb_falling_bricks(&bricks, b))
        .filter(|&nb_falling| nb_falling == 0)
        .count()
}

fn part_2(input: &str) -> usize {
    let mut bricks = parse_input(input);
    let (bricks, _) = let_them_fall(&mut bricks);

    (0..bricks.len())
        .map(|i| {
            let mut one_removed = Vec::from_iter(bricks.clone());
            one_removed.remove(i);

            let (_, nb_fallen) = let_them_fall(&mut one_removed);
            nb_fallen
        })
        .sum()
}

#[derive(Debug, Clone)]
struct Brick {
    x: (u32, u32),
    y: (u32, u32),
    z: (u32, u32),
}

fn let_them_fall(bricks: &mut Vec<Brick>) -> (Vec<Brick>, usize) {
    bricks.sort_by_key(|brick| brick.z.0);
    let mut fallen_bricks = Vec::with_capacity(bricks.len());
    let mut nb_fallen = 0;

    for b in bricks.iter() {
        let z_min = get_fall_altitude(&fallen_bricks, b);
        let z_max = z_min + (b.z.1 - b.z.0);
        if z_min != b.z.0 {
            nb_fallen += 1;
        }

        let fallen = Brick {
            x: b.x,
            y: b.y,
            z: (z_min, z_max),
        };
        fallen_bricks.push(fallen);
    }

    (fallen_bricks, nb_fallen)
}

fn get_fall_altitude(list: &[Brick], to_fall: &Brick) -> u32 {
    // Fall altitude = max of z_max for all bricks below +1.
    // Requires the list to be sorted, so tha all below/with a lower altitude
    // would already have fallen.
    list.iter()
        .filter(|&other| {
            let is_below = other.z.1 < to_fall.z.0;
            let overlap_x = overlap(&other.x, &to_fall.x);
            let overlap_y = overlap(&other.y, &to_fall.y);

            is_below && overlap_x && overlap_y
        })
        .map(|other| other.z.1 + 1)
        .max()
        .unwrap_or(1) // 1 = ground altitude
}

fn get_nb_falling_bricks(bricks: &[Brick], removed: &Brick) -> usize {
    let falling = bricks
        .iter()
        .filter(|&other| {
            let is_atop = other.z.0 == removed.z.1 + 1;
            let overlap_x = overlap(&other.x, &removed.x);
            let overlap_y = overlap(&other.y, &removed.y);

            is_atop && overlap_x && overlap_y
        })
        .filter(|&other| count_supporting_bricks(bricks, other) < 2)
        .collect_vec();
    //.count()

    println!("{:?} --> Falling = {:?}", removed, falling);
    falling.len()
}

fn count_supporting_bricks(brick_set: &[Brick], brick: &Brick) -> usize {
    brick_set
        .iter()
        .filter(|&other| {
            let is_under = other.z.1 + 1 == brick.z.0;
            let overlap_x = overlap(&other.x, &brick.x);
            let overlap_y = overlap(&other.y, &brick.y);

            is_under && overlap_x && overlap_y
        })
        .count()
}

fn overlap(a: &(u32, u32), b: &(u32, u32)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let (min, max) = line.split_once('~').unwrap();
            let (x_min, y_min, z_min) = min
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();
            let (x_max, y_max, z_max) = max
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();

            Brick {
                x: (x_min, x_max),
                y: (y_min, y_max),
                z: (z_min, z_max),
            }
        })
        .collect_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample!()), 5);
        assert_eq!(part_2(sample!()), 7);
    }
}
