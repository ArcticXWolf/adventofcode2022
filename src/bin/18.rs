use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;

use advent_of_code::helpers::Point3;
use itertools::Itertools;

fn parse_input(input: &str) -> HashSet<Point3<i32>> {
    let points = input.trim().lines().map(|l| {
        let split = l.split(',').collect_tuple::<(&str, &str, &str)>().unwrap();
        Point3 {
            x: split.0.parse::<i32>().unwrap(),
            y: split.1.parse::<i32>().unwrap(),
            z: split.2.parse::<i32>().unwrap(),
        }
    });
    HashSet::from_iter(points)
}

fn count_open_sides(points: HashSet<Point3<i32>>) -> u32 {
    let mut closed: HashSet<Point3<i32>> = HashSet::with_capacity(points.len());
    let mut queue: VecDeque<Point3<i32>> = VecDeque::new();
    let mut tracker = 0;

    while points.intersection(&closed).count() != points.len() {
        queue.push_back(points.difference(&closed).next().cloned().unwrap());
        while !queue.is_empty() {
            if let Some(point) = queue.pop_front() {
                // print!("Cube: {:?}", point);
                let tracker_old = tracker;
                closed.insert(point);

                for d in Point3::directions() {
                    let neighbor = point + d;

                    if closed.contains(&neighbor) {
                        continue;
                    }

                    if points.contains(&neighbor) {
                        queue.push_back(neighbor);
                        continue;
                    }

                    tracker += 1;
                }
                // println!(", open {}", tracker - tracker_old);
            }
        }
    }

    tracker
}

pub fn part_one(_input: &str) -> Option<u32> {
    let points = parse_input(_input);
    Some(count_open_sides(points))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), None);
    }
}
