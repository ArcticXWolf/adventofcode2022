use std::{collections::HashSet, fmt};

use advent_of_code::helpers::{lcm, Point, PointDirection, PointGrid};
use parse_display::{Display, FromStr};
use priority_queue::PriorityQueue;

#[derive(Debug, Display, FromStr, Clone, PartialEq, Eq)]
#[display("{dir}")]
struct Blizzard {
    dir: PointDirection,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct BlizzardList(Vec<Blizzard>);

impl fmt::Display for BlizzardList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.len() {
            0 => {
                write!(f, ".")
            }
            1 => {
                write!(f, "{}", self.0[0].dir)
            }
            _ => {
                write!(f, "{}", self.0.len())
            }
        }
    }
}

fn parse_input(input: &str) -> (PointGrid<BlizzardList>, Point<isize>, Point<isize>) {
    let mut result: PointGrid<BlizzardList> = PointGrid::default();

    for (y, l) in input.lines().enumerate().skip(1) {
        for (x, c) in l.chars().enumerate() {
            let blizzard = match c {
                '^' | '>' | 'v' | '<' => Some(c.to_string().parse::<Blizzard>().unwrap()),
                _ => None,
            };

            if let Some(b) = blizzard {
                result
                    .points
                    .entry(Point {
                        x: x as isize,
                        y: y as isize,
                    })
                    .or_default()
                    .0
                    .push(b);
            } else if c == '.' {
                result
                    .points
                    .entry(Point {
                        x: x as isize,
                        y: y as isize,
                    })
                    .or_default();
            }
        }
    }

    let (min, max) = result.dimensions();
    let start = Point {
        x: min.x,
        y: min.y - 1,
    };
    let end = Point { x: max.x, y: max.y };
    result.insert(start, BlizzardList::default());

    (result, start, end)
}

fn forward_blizzards(blizzards: &PointGrid<BlizzardList>) -> PointGrid<BlizzardList> {
    let mut result: PointGrid<BlizzardList> = PointGrid::default();
    let (min_grid, max_grid) = blizzards.dimensions();
    let (min_bliz, max_bliz) = (
        Point {
            x: min_grid.x,
            y: min_grid.y + 1,
        },
        Point {
            x: max_grid.x + 1,
            y: max_grid.y,
        },
    );
    for p in blizzards.points.keys() {
        result.points.entry(*p).or_default();
    }

    for (p, bl) in blizzards.points.iter() {
        for b in bl.0.iter() {
            let new_point = p
                .get_point_in_direction(&b.dir, 1)
                .wrap_around_in_rectangle(min_bliz, max_bliz);

            result
                .points
                .entry(new_point)
                .or_default()
                .0
                .push(b.clone());
        }
    }

    result
}

fn pathfinding(
    bl_cache: &[PointGrid<BlizzardList>],
    start: &Point<isize>,
    end: &Point<isize>,
    time_offset: usize,
    cycle_length: usize,
    max_distance: usize,
) -> Option<usize> {
    let mut closed_set: HashSet<(Point<isize>, usize)> = HashSet::new();
    let mut queue: PriorityQueue<(Point<isize>, usize), usize> = PriorityQueue::new();

    queue.push(
        (*start, time_offset),
        max_distance - start.manhattan_distance(end),
    );
    while let Some(((current_pos, current_min), _)) = queue.pop() {
        closed_set.insert((current_pos, current_min % cycle_length));

        for next_pos in
            get_possible_actions(&bl_cache[(current_min + 1) % cycle_length], &current_pos)
        {
            if closed_set.contains(&(next_pos, (current_min + 1) % cycle_length)) {
                continue;
            }
            if next_pos == *end {
                return Some(current_min + 1);
            }
            queue.push(
                (next_pos, current_min + 1),
                max_distance - (current_min + 1 + current_pos.manhattan_distance(end)),
            );
        }
    }

    None
}

fn get_possible_actions(
    blizzards_next_round: &PointGrid<BlizzardList>,
    current_position: &Point<isize>,
) -> Vec<Point<isize>> {
    let mut next_positions = vec![];

    for d in PointDirection::all() {
        let new_pos = current_position.get_point_in_direction(d, 1);
        if let Some(bl) = blizzards_next_round.points.get(&new_pos) {
            if bl.0.is_empty() {
                next_positions.push(new_pos);
            }
        }
    }

    // wait
    let new_pos = *current_position;
    if let Some(bl) = blizzards_next_round.points.get(&new_pos) {
        if bl.0.is_empty() {
            next_positions.push(new_pos);
        }
    }

    next_positions
}

fn init_valley(
    blizzards: &PointGrid<BlizzardList>,
) -> (Vec<PointGrid<BlizzardList>>, usize, usize) {
    let (min, max) = blizzards.dimensions();
    let cycle_length = lcm((max.x + 1 - min.x) as usize, (max.y - (min.y + 1)) as usize);
    let max_distance = Point {
        x: max.x + 1,
        y: max.y + 1,
    }
    .manhattan_distance(&min)
        * cycle_length;

    let mut bl_cache: Vec<PointGrid<BlizzardList>> = vec![];

    let mut current_bliz = blizzards.clone();
    for _ in 0..cycle_length {
        bl_cache.push(current_bliz.clone());
        current_bliz = forward_blizzards(&current_bliz);
    }

    (bl_cache, cycle_length, max_distance)
}

pub fn part_one(_input: &str) -> Option<usize> {
    let (grid, start, end) = parse_input(_input);
    let (bl_cache, cycle_length, max_distance) = init_valley(&grid);

    pathfinding(&bl_cache, &start, &end, 0, cycle_length, max_distance)
}

pub fn part_two(_input: &str) -> Option<usize> {
    let (grid, start, end) = parse_input(_input);
    let (bl_cache, cycle_length, max_distance) = init_valley(&grid);

    let goal1 = pathfinding(&bl_cache, &start, &end, 0, cycle_length, max_distance).unwrap();
    let goal2 = pathfinding(&bl_cache, &end, &start, goal1, cycle_length, max_distance).unwrap();
    pathfinding(&bl_cache, &start, &end, goal2, cycle_length, max_distance)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(54));
    }
}
