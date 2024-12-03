use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]*),([0-9]*)\)").unwrap();

    Some(
        re.captures_iter(input).map(|c| c.extract())
            .map(|(_, [a, b])| a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut enabled = true;
    let mut muls = vec![];

    let re = Regex::new(r"mul\(([0-9]*),([0-9]*)\)|(do\(\))()|(don't\(\))()").unwrap();

    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        match a {
            "do()" => { enabled = true; },
            "don't()" => { enabled = false; },
            _ => {
                if enabled {
                    muls.push(a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap());
                }
            }
        }
    }

    Some(
        muls.iter().sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
