use advent_of_code::helpers::{Point, PointDirection, PointGrid};
use parse_display::{Display, FromStr};

#[derive(Debug, Display, FromStr)]
enum Content {
    #[display("#")]
    Elf,
}

fn parse_input(_input: &str) -> PointGrid<Content> {
    let mut grid: PointGrid<Content> = PointGrid::default();

    for (y, l) in _input.lines().map(|l| l.chars()).enumerate() {
        for (x, c) in l.enumerate() {
            if c == '#' {
                grid.insert(
                    Point {
                        x: x as isize,
                        y: y as isize,
                    },
                    Content::Elf,
                );
            }
        }
    }

    grid
}

fn get_proposed_moves(
    grid: &PointGrid<Content>,
    cycle: usize,
) -> (bool, PointGrid<Vec<Point<isize>>>) {
    let mut proposed_by: PointGrid<Vec<Point<isize>>> = PointGrid::default();
    let mut anyone_moved: bool = false;
    let mut directions_order = [
        PointDirection::North,
        PointDirection::South,
        PointDirection::West,
        PointDirection::East,
    ];
    directions_order.rotate_left(cycle);

    for elf in grid.points.keys() {
        // print!("Point {} proposes: ", elf);
        if PointDirection::all_with_diagonals()
            .all(|d| grid.get(&elf.get_point_in_direction(d, 1)).is_none())
        {
            // println!("rest.");
            proposed_by.insert(*elf, vec![*elf]);
            continue;
        }

        anyone_moved = true;
        let mut movable = false;
        for d in directions_order.iter() {
            match d {
                PointDirection::North => {
                    if [
                        PointDirection::North,
                        PointDirection::NorthEast,
                        PointDirection::NorthWest,
                    ]
                    .iter()
                    .all(|d| grid.get(&elf.get_point_in_direction(d, 1)).is_none())
                    {
                        let proposed_position =
                            elf.get_point_in_direction(&PointDirection::North, 1);
                        if let Some(v) = proposed_by.get(&proposed_position) {
                            let mut new_list = v.clone();
                            new_list.push(*elf);
                            // println!("move to {}, (with {:?})", proposed_position, new_list);
                            proposed_by.insert(proposed_position, new_list);
                        } else {
                            // println!("move to {}", proposed_position);
                            proposed_by.insert(proposed_position, vec![*elf]);
                        }
                        movable = true;
                        break;
                    }
                }
                PointDirection::South => {
                    if [
                        PointDirection::South,
                        PointDirection::SouthEast,
                        PointDirection::SouthWest,
                    ]
                    .iter()
                    .all(|d| grid.get(&elf.get_point_in_direction(d, 1)).is_none())
                    {
                        let proposed_position =
                            elf.get_point_in_direction(&PointDirection::South, 1);
                        if let Some(v) = proposed_by.get(&proposed_position) {
                            let mut new_list = v.clone();
                            new_list.push(*elf);
                            // println!("move to {}, (with {:?})", proposed_position, new_list);
                            proposed_by.insert(proposed_position, new_list);
                        } else {
                            // println!("move to {}", proposed_position);
                            proposed_by.insert(proposed_position, vec![*elf]);
                        }
                        movable = true;
                        break;
                    }
                }
                PointDirection::West => {
                    if [
                        PointDirection::West,
                        PointDirection::NorthWest,
                        PointDirection::SouthWest,
                    ]
                    .iter()
                    .all(|d| grid.get(&elf.get_point_in_direction(d, 1)).is_none())
                    {
                        let proposed_position =
                            elf.get_point_in_direction(&PointDirection::West, 1);
                        if let Some(v) = proposed_by.get(&proposed_position) {
                            let mut new_list = v.clone();
                            new_list.push(*elf);
                            // println!("move to {}, (with {:?})", proposed_position, new_list);
                            proposed_by.insert(proposed_position, new_list);
                        } else {
                            // println!("move to {}", proposed_position);
                            proposed_by.insert(proposed_position, vec![*elf]);
                        }
                        movable = true;
                        break;
                    }
                }
                PointDirection::East => {
                    if [
                        PointDirection::East,
                        PointDirection::NorthEast,
                        PointDirection::SouthEast,
                    ]
                    .iter()
                    .all(|d| grid.get(&elf.get_point_in_direction(d, 1)).is_none())
                    {
                        let proposed_position =
                            elf.get_point_in_direction(&PointDirection::East, 1);
                        if let Some(v) = proposed_by.get(&proposed_position) {
                            let mut new_list = v.clone();
                            new_list.push(*elf);
                            // println!("move to {}, (with {:?})", proposed_position, new_list);
                            proposed_by.insert(proposed_position, new_list);
                        } else {
                            // println!("move to {}", proposed_position);
                            proposed_by.insert(proposed_position, vec![*elf]);
                        }
                        movable = true;
                        break;
                    }
                }
                _ => unimplemented!(),
            }
        }

        if !movable {
            // println!("stuck, stay at {}", elf);
            proposed_by.insert(*elf, vec![*elf]);
        }
    }

    (anyone_moved, proposed_by)
}

fn do_proposed_move(proposed_list: &PointGrid<Vec<Point<isize>>>) -> PointGrid<Content> {
    let mut grid: PointGrid<Content> = PointGrid::default();

    for (pos, elves) in proposed_list.points.iter() {
        if elves.len() > 1 {
            for elf in elves {
                grid.insert(*elf, Content::Elf);
            }
        } else {
            grid.insert(*pos, Content::Elf);
        }
    }

    grid
}

pub fn part_one(_input: &str) -> Option<isize> {
    let mut grid: PointGrid<Content> = parse_input(_input);

    for i in 0..10 {
        let (anyone_moved, proposed_list) = get_proposed_moves(&grid, i % 4);
        if !anyone_moved {
            break;
        }
        grid = do_proposed_move(&proposed_list);
    }

    let (min, max) = grid.dimensions();
    let tiles = ((max.x + 1) - min.x) * ((max.y + 1) - min.y);
    Some(tiles - grid.points.keys().len() as isize)
}

pub fn part_two(_input: &str) -> Option<isize> {
    let mut grid: PointGrid<Content> = parse_input(_input);
    let mut tracker: isize = 1;

    loop {
        let (anyone_moved, proposed_list) = get_proposed_moves(&grid, (tracker as usize - 1) % 4);
        if !anyone_moved {
            break;
        }
        grid = do_proposed_move(&proposed_list);
        tracker += 1;
    }

    Some(tracker)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[ignore]
    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
