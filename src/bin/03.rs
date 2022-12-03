use itertools::Itertools;

fn split_rucksack(rucksack: &str) -> (Vec<char>, Vec<char>) {
    let (left, right) = rucksack.split_at(rucksack.len() / 2);
    (left.chars().collect(), right.chars().collect())
}

fn gather_same_items(left: &[char], right: &[char]) -> Vec<char> {
    left.iter()
        .cloned()
        .unique()
        .filter(|c| right.contains(c))
        .collect()
}

fn gather_same_items_in_rucksack_group(rucksack_group: &[&str]) -> Vec<char> {
    let compare_rucksack = rucksack_group.first().unwrap().chars().into_iter().unique();
    let mut same_items = compare_rucksack.collect_vec();

    for rucksack in rucksack_group.into_iter().skip(1) {
        same_items = same_items
            .iter()
            .cloned()
            .filter(|c| rucksack.contains(*c))
            .collect();
    }

    same_items
}

fn priority_of_item(item: &char) -> u32 {
    match *item as u32 {
        65..=90 => (*item as u32) - 38,
        97..=122 => (*item as u32) - 96,
        _ => panic!("Got non character!"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(split_rucksack)
            .map(|(l, r)| gather_same_items(&l, &r))
            .map(|il| il.iter().map(priority_of_item).sum::<u32>())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let it = input.lines().chunks(3);
    Some(
        it.into_iter()
            .map(|i| gather_same_items_in_rucksack_group(&i.collect_vec()))
            .map(|il| il.iter().map(priority_of_item).sum::<u32>())
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
