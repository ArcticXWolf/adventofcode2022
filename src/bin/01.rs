use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let it = input
        .split("\n\n")
        .map(|linegroup| linegroup.lines().map(|l| l.parse::<u32>().unwrap()));

    it.map(|ng| ng.sum()).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let it = input
        .split("\n\n")
        .map(|linegroup| linegroup.lines().map(|l| l.parse::<u32>().unwrap()));

    Some(it.map(|ng| ng.sum::<u32>()).sorted().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
