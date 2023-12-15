use aoc_rs_2023::*;

fn main() {
    println!("Part 1: {}", part_1(input!()));
    println!("Part 2: {}", part_2(input!()));
}

fn part_1(input: &str) -> u32 {
    input.split(',').map(|s| hash(s) as u32).sum()
}

fn part_2(input: &str) -> u32 {
    let mut boxes = vec![vec![]; 256];
    input.split(',').for_each(|s| store_lens(&mut boxes, s));

    boxes
        .iter()
        .enumerate()
        .flat_map(|(id, lens_box)| {
            lens_box
                .iter()
                .enumerate()
                .map(move |(n, &(_, focal))| (id as u32 + 1) * (n as u32 + 1) * focal)
        })
        .sum()
}

fn hash(string: &str) -> u8 {
    string
        .chars()
        .map(|c| c as u32)
        .fold(0_u32, |acc, x| ((acc + x) * 17) % 256) as u8
}

fn store_lens(boxes: &mut [Vec<(String, u32)>], instruction: &str) {
    let (label, focal) = if instruction.ends_with('-') {
        (instruction.trim_end_matches('-'), None)
    } else {
        instruction
            .split_once('=')
            .map(|(label, f)| (label, f.parse::<u32>().ok()))
            .unwrap()
    };

    let lens_box = &mut boxes[hash(label) as usize];
    if let Some(focal_value) = focal {
        if let Some(lens) = lens_box.iter_mut().find(|(l, _)| l == label) {
            lens.1 = focal_value;
        } else {
            lens_box.push((label.to_string(), focal_value));
        }
    } else {
        lens_box.retain(|(l, _)| l != label);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample!()), 1320);
        assert_eq!(part_2(sample!()), 145);
    }
}
