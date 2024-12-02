use itertools::Itertools;

advent_of_code::solution!(1);

fn parse_num_arrays(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("   ").unwrap();
            (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (nums_l, nums_r) = parse_num_arrays(input);
    Some(
        nums_l.into_iter().sorted()
            .zip(nums_r.into_iter().sorted())
            .map(|(l, r)| l.abs_diff(r)).sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (nums_l, nums_r) = parse_num_arrays(input);
    let counts = nums_r.iter().counts();
    Some(
        nums_l.iter().map(|i| i * *counts.get(&i).unwrap_or(&0) as u32).sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
