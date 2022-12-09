use std::collections::HashSet;

use itertools::Itertools;

fn parse_input(_input: &str) -> Vec<(char, u32)> {
    _input
        .lines()
        .map(|x| {
            let (c, n) = x.split_once(' ').unwrap();
            (c.chars().last().unwrap(), n.parse().unwrap())
        })
        .collect_vec()
}

type Coord2D = [i32; 2];

fn add(coord: Coord2D, other: Coord2D) -> Coord2D {
    [coord[0] + other[0], coord[1] + other[1]]
}

fn substract(coord: Coord2D, other: Coord2D) -> Coord2D {
    [coord[0] - other[0], coord[1] - other[1]]
}

fn get_direction_from_char(c: char) -> Coord2D {
    match c {
        'U' => [0, 1],
        'R' => [1, 0],
        'D' => [0, -1],
        'L' => [-1, 0],
        _ => unreachable!(),
    }
}

fn move_towards(h_pos: Coord2D, t_pos: Coord2D) -> Coord2D {
    match substract(h_pos, t_pos) {
        [x, y] if x.abs() > 1 || y.abs() > 1 => [t_pos[0] + x.signum(), t_pos[1] + y.signum()],
        _ => t_pos,
    }
}

pub fn part_one(_input: &str) -> Option<u32> {
    let movements = parse_input(_input);
    let mut h_pos: Coord2D = [0, 0];
    let mut t_pos: Coord2D = [0, 0];
    let mut tracker: HashSet<Coord2D> = HashSet::new();

    for (c, amount) in movements {
        for _ in 0..amount {
            h_pos = add(h_pos, get_direction_from_char(c));
            t_pos = move_towards(h_pos, t_pos);
            tracker.insert(t_pos);
        }
    }

    Some(tracker.len() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let movements = parse_input(_input);
    let mut knots: Vec<Coord2D> = vec![[0, 0]; 10];
    let mut tracker: HashSet<Coord2D> = HashSet::new();

    for (c, amount) in movements {
        for _ in 0..amount {
            knots[0] = add(knots[0], get_direction_from_char(c));
            for k in 1..knots.len() {
                knots[k] = move_towards(knots[k - 1], knots[k]);
            }
            tracker.insert(knots[knots.len() - 1]);
        }
    }

    Some(tracker.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
