use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;

use advent_of_code::helpers::Point3;
use itertools::Itertools;

fn parse_input(input: &str) -> (HashSet<Point3<i32>>, Point3<i32>, Point3<i32>) {
    let points = input.trim().lines().map(|l| {
        let split = l.split(',').collect_tuple::<(&str, &str, &str)>().unwrap();
        Point3 {
            x: split.0.parse::<i32>().unwrap(),
            y: split.1.parse::<i32>().unwrap(),
            z: split.2.parse::<i32>().unwrap(),
        }
    });
    let minpoint = Point3 {
        x: points.clone().map(|p| p.x).min().unwrap(),
        y: points.clone().map(|p| p.y).min().unwrap(),
        z: points.clone().map(|p| p.z).min().unwrap(),
    };
    let maxpoint = Point3 {
        x: points.clone().map(|p| p.x).max().unwrap(),
        y: points.clone().map(|p| p.y).max().unwrap(),
        z: points.clone().map(|p| p.z).max().unwrap(),
    };
    (HashSet::from_iter(points), minpoint, maxpoint)
}

fn count_open_sides(points: &HashSet<Point3<i32>>) -> u32 {
    let mut tracker = 0;

    for point in points.iter() {
        for d in Point3::directions() {
            let neighbor = *point + d;

            if points.contains(&neighbor) {
                continue;
            }

            tracker += 1;
        }
    }

    tracker
}

fn find_connected_points(
    points: &HashSet<Point3<i32>>,
    start: &Point3<i32>,
) -> HashSet<Point3<i32>> {
    let mut closed: HashSet<Point3<i32>> = HashSet::new();
    let mut queue: VecDeque<Point3<i32>> = VecDeque::new();

    queue.push_back(*start);
    while !queue.is_empty() {
        if let Some(point) = queue.pop_front() {
            for d in Point3::directions() {
                let neighbor = point + d;

                if closed.contains(&neighbor) {
                    continue;
                }

                if points.contains(&neighbor) {
                    queue.push_back(neighbor);
                    closed.insert(neighbor);
                    continue;
                }
            }
        }
    }

    closed
}

fn create_cube(minpoint: &Point3<i32>, maxpoint: &Point3<i32>) -> HashSet<Point3<i32>> {
    let mut closed: HashSet<Point3<i32>> = HashSet::new();
    for x in minpoint.x..maxpoint.x {
        for y in minpoint.y..maxpoint.y {
            for z in minpoint.z..maxpoint.z {
                closed.insert(Point3 { x, y, z });
            }
        }
    }
    closed
}

pub fn part_one(_input: &str) -> Option<u32> {
    let (points, _, _) = parse_input(_input);
    Some(count_open_sides(&points))
}

pub fn part_two(_input: &str) -> Option<u32> {
    let (points, min, max) = parse_input(_input);

    let cube = create_cube(
        &(min - Point3 { x: 1, y: 1, z: 1 }),
        &(max + Point3 { x: 2, y: 2, z: 2 }),
    );
    let difference = cube.difference(&points).cloned().collect();
    let outside = find_connected_points(&difference, &(min - Point3 { x: 1, y: 1, z: 1 }));
    let inside: HashSet<Point3<i32>> = difference.difference(&outside).cloned().collect();

    Some(count_open_sides(&points) - count_open_sides(&inside))
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
        assert_eq!(part_two(&input), Some(58));
    }
}
