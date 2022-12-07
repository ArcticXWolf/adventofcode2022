use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut found_index: i32 = -1;
    for (i, cl) in input.chars().collect_vec().windows(4).enumerate() {
        if cl.iter().sorted().dedup().count() == 4 {
            found_index = i as i32;
            break;
        }
    }

    if found_index > 0 {
        return Some(found_index as u32 + 4);
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut found_index: i32 = -1;
    for (i, cl) in input.chars().collect_vec().windows(14).enumerate() {
        if cl.iter().sorted().dedup().count() == 14 {
            found_index = i as i32;
            break;
        }
    }

    if found_index > 0 {
        return Some(found_index as u32 + 14);
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
