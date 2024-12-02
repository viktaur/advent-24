use std::{borrow::BorrowMut, slice::Iter};

use itertools::Itertools;

advent_of_code::solution!(2);

fn diffs_within_bounds(nums: &[i32]) -> bool {
    let diffs = nums.iter().tuple_windows().map(|(a, b)| a-b);
    diffs.to_owned().all(|d| d >= 1 && d <= 3) || diffs.to_owned().all(|d| d >= -3 && d <= -1)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect_vec()
            )
            .filter(|nums| diffs_within_bounds(nums))
            .count() as u32
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect_vec()
            )
            .filter(|nums| {
                (0..nums.len()).any(|c| {
                    let new_nums = nums.iter().enumerate().filter(|(i, _)| *i != c).map(|(_, v)| *v).collect_vec();
                    diffs_within_bounds(&new_nums)
                })
            })
            .count() as u32
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diffs_within_bounds() {
        assert_eq!(diffs_within_bounds(&[7, 6, 4, 2, 1]), true);
        assert_eq!(diffs_within_bounds(&[1, 2, 7, 8, 9]), false);
        assert_eq!(diffs_within_bounds(&[9, 7, 6, 2, 1]), false);
        assert_eq!(diffs_within_bounds(&[1, 3, 2, 4, 5]), false);
        assert_eq!(diffs_within_bounds(&[8, 6, 4, 4, 1]), false);
        assert_eq!(diffs_within_bounds(&[1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
