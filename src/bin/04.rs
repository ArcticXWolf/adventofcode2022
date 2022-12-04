use std::ops::RangeInclusive;

use regex::Regex;

// TODO: rewrite the range checks with functional programming

fn are_ranges_containing_each_other(
    range1: &RangeInclusive<u32>,
    range2: &RangeInclusive<u32>,
) -> bool {
    // just check the starts and ends for containment
    (range1.start() <= range2.start() && range1.end() >= range2.end())
        || (range2.start() <= range1.start() && range2.end() >= range1.end())
}

fn are_ranges_overlapping(range1: &RangeInclusive<u32>, range2: &RangeInclusive<u32>) -> bool {
    // if start and end is before the start of the other, then they are NOT overlapping
    !((range1.start() < range2.start() && range1.end() < range2.start())
        || (range2.start() < range1.start() && range2.end() < range1.start()))
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut count = 0;
    for pair in re.captures_iter(input) {
        let parsed_pair: Vec<u32> = pair
            .iter()
            .skip(1)
            .map(|s| s.unwrap().as_str().parse::<u32>().unwrap())
            .collect();
        let elf1 = parsed_pair[0]..=parsed_pair[1];
        let elf2 = parsed_pair[2]..=parsed_pair[3];
        if are_ranges_containing_each_other(&elf1, &elf2) {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut count = 0;
    for pair in re.captures_iter(input) {
        let parsed_pair: Vec<u32> = pair
            .iter()
            .skip(1)
            .map(|s| s.unwrap().as_str().parse::<u32>().unwrap())
            .collect();
        let elf1 = parsed_pair[0]..=parsed_pair[1];
        let elf2 = parsed_pair[2]..=parsed_pair[3];
        if are_ranges_overlapping(&elf1, &elf2) {
            count += 1;
        }
    }

    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
