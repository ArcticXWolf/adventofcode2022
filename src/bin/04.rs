fn are_ranges_containing_each_other(range1: &(u32, u32), range2: &(u32, u32)) -> bool {
    // just check the starts and ends for containment
    (range1.0 <= range2.0 && range1.1 >= range2.1) || (range2.0 <= range1.0 && range2.1 >= range1.1)
}

fn are_ranges_overlapping(range1: &(u32, u32), range2: &(u32, u32)) -> bool {
    // if end is before the start of the other, then they are NOT overlapping
    !((range1.0 < range2.0 && range1.1 < range2.0) || (range2.0 < range1.0 && range2.1 < range1.0))
}

fn extract_ranges_from_line(line: &str) -> ((u32, u32), (u32, u32)) {
    let (l, r) = line.split_once(',').unwrap();
    let lrange = l.split_once('-').unwrap();
    let rrange = r.split_once('-').unwrap();
    (
        (
            lrange.0.parse::<u32>().unwrap(),
            lrange.1.parse::<u32>().unwrap(),
        ),
        (
            rrange.0.parse::<u32>().unwrap(),
            rrange.1.parse::<u32>().unwrap(),
        ),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(extract_ranges_from_line)
            .filter(|(elf1, elf2)| are_ranges_containing_each_other(elf1, elf2))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(extract_ranges_from_line)
            .filter(|(elf1, elf2)| are_ranges_overlapping(elf1, elf2))
            .count() as u32,
    )
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
