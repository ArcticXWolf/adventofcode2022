use std::{
    collections::HashMap,
    io::{self, Read},
};

use advent_of_code::helpers::{Point, PointDirection, PointGrid};
use itertools::Itertools;
use parse_display::Display;

// Urgh, this day is even worse programming from my part. I made the decision
// early to not bother with 3D coordinates and 3D rotations and instead list and
// check every target for a wraparound (see wrap_around_cube). Probably still
// missing a few possibilities but this was enough for the example and my input.
// Also here are some debug output lines left.

#[derive(Debug, Clone, Display, PartialEq)]
enum Content {
    #[display(".")]
    Empty,
    #[display("#")]
    Wall,
}

#[derive(Debug, Clone, PartialEq)]
enum Action {
    Forward(isize),
    TurnLeft,
    TurnRight,
}

fn parse_input(_input: &str) -> (PointGrid<Content>, Vec<Action>) {
    let mut grid: PointGrid<Content> = PointGrid::default();
    let mut actions: Vec<Action> = vec![];

    let (gridstr, actionstr) = _input.split_once("\n\n").unwrap();
    for (y, l) in gridstr.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '.' => grid.insert(
                    Point {
                        x: x as isize,
                        y: y as isize,
                    },
                    Content::Empty,
                ),
                '#' => grid.insert(
                    Point {
                        x: x as isize,
                        y: y as isize,
                    },
                    Content::Wall,
                ),
                _ => {}
            }
        }
    }

    let re = regex::Regex::new(r"(\d+|[LR]{1})").unwrap();
    for caps in re.captures_iter(actionstr) {
        if caps.len() == 2 {
            let action = match &caps[1] {
                "L" => Action::TurnLeft,
                "R" => Action::TurnRight,
                x => Action::Forward(x.parse::<isize>().unwrap()),
            };
            actions.push(action);
        }
    }

    (grid, actions)
}

fn convert_into_cube_grids(
    input: &PointGrid<Content>,
) -> (Vec<PointGrid<Content>>, PointGrid<usize>, isize) {
    let dimensions = input.dimensions();
    let face_size = match (dimensions.1.x + 1) * 60 / (dimensions.1.y + 1) {
        // 4 : 3
        80 => (dimensions.1.x + 1) / 4,
        // 3 : 4
        45 => (dimensions.1.x + 1) / 3,
        // 5 : 2
        600 => (dimensions.1.x + 1) / 5,
        // 2 : 5
        96 => (dimensions.1.x + 1) / 2,
        _ => unimplemented!(),
    };
    let mut faces = vec![];
    let mut face_grid: PointGrid<usize> = PointGrid::default();

    for face_y in 0..((dimensions.1.y + 1) / face_size) {
        for face_x in 0..((dimensions.1.x + 1) / face_size) {
            if input
                .get(&Point {
                    x: face_x * face_size,
                    y: face_y * face_size,
                })
                .is_none()
            {
                continue;
            }

            let mut face: PointGrid<Content> = PointGrid::default();
            for (p, c) in input.points.iter().filter(|(p, _)| {
                p.x >= face_x * face_size
                    && p.x < (face_x + 1) * face_size
                    && p.y >= face_y * face_size
                    && p.y < (face_y + 1) * face_size
            }) {
                face.insert(
                    Point {
                        x: p.x % face_size,
                        y: p.y % face_size,
                    },
                    c.clone(),
                );
            }
            let id = faces.len();
            faces.push(face);
            face_grid.insert(
                Point {
                    x: face_x,
                    y: face_y,
                },
                id,
            );
        }
    }

    (faces, face_grid, face_size)
}

fn get_start_position(grid: &PointGrid<Content>) -> (Point<isize>, PointDirection) {
    (
        *grid
            .points
            .keys()
            .filter(|p| p.y == 0)
            .min_by_key(|p| p.x)
            .unwrap(),
        PointDirection::East,
    )
}

fn do_action(
    grid: &PointGrid<Content>,
    pos: Point<isize>,
    facing: PointDirection,
    action: &Action,
    path: &mut HashMap<Point<isize>, PointDirection>,
) -> (Point<isize>, PointDirection) {
    match action {
        Action::TurnLeft => {
            path.insert(pos, facing.direction_left());
            (pos, facing.direction_left())
        }
        Action::TurnRight => {
            path.insert(pos, facing.direction_right());
            (pos, facing.direction_right())
        }
        Action::Forward(steps) => {
            let mut current = pos;
            for _ in 0..*steps {
                let mut new_pos = current.get_point_in_direction(&facing, 1);
                if let Some(c) = grid.get(&new_pos) {
                    if *c == Content::Empty {
                        current = new_pos;
                        path.insert(current, facing.clone());
                    }
                } else {
                    new_pos = grid.wrap_around(&current, &facing);
                    if *grid.get(&new_pos).unwrap() == Content::Empty {
                        current = new_pos;
                        path.insert(current, facing.clone());
                    }
                }
            }
            (current, facing)
        }
    }
}

fn do_action_cube(
    face_grid: &PointGrid<usize>,
    faces: &[PointGrid<Content>],
    face_size: &isize,
    face: usize,
    pos: Point<isize>,
    facing: PointDirection,
    action: &Action,
    path: &mut HashMap<(usize, Point<isize>), PointDirection>,
) -> (usize, Point<isize>, PointDirection) {
    match action {
        Action::TurnLeft => {
            path.insert((face, pos), facing.direction_left());
            (face, pos, facing.direction_left())
        }
        Action::TurnRight => {
            path.insert((face, pos), facing.direction_right());
            (face, pos, facing.direction_right())
        }
        Action::Forward(steps) => {
            let (mut current_face, mut current_pos, mut current_facing) = (face, pos, facing);
            for _ in 0..*steps {
                let new_pos = current_pos.get_point_in_direction(&current_facing.clone(), 1);
                if let Some(c) = faces[current_face].get(&new_pos) {
                    if *c == Content::Empty {
                        current_pos = new_pos;
                        path.insert((current_face, current_pos), current_facing.clone());
                    }
                } else {
                    let (new_face, new_pos, new_facing) = wrap_around_cube(
                        face_grid,
                        face_size,
                        current_face,
                        current_pos,
                        current_facing.clone(),
                    );
                    if *faces[new_face].get(&new_pos).unwrap() == Content::Empty {
                        current_face = new_face;
                        current_pos = new_pos;
                        current_facing = new_facing;
                        path.insert((current_face, current_pos), current_facing.clone());
                    }
                }
            }
            (current_face, current_pos, current_facing)
        }
    }
}

fn wrap_around_cube(
    face_grid: &PointGrid<usize>,
    face_size: &isize,
    face: usize,
    pos: Point<isize>,
    facing: PointDirection,
) -> (usize, Point<isize>, PointDirection) {
    let face_pos = face_grid
        .points
        .iter()
        .find(|(_, i)| **i == face)
        .unwrap()
        .0;

    // X
    // O
    if let Some(new_face) = face_grid.get(&face_pos.get_point_in_direction(&facing, 1)) {
        let new_pos = calculate_flipped_wrap(&pos, &facing, face_size, 0);
        return (*new_face, new_pos, facing);
    }
    // X.
    // -O
    if let Some(new_face) = face_grid.get(
        &face_pos
            .get_point_in_direction(&facing, 1)
            .get_point_in_direction(&facing.direction_left(), 1),
    ) {
        let new_pos = calculate_flipped_wrap(&pos, &facing, face_size, 3);
        return (*new_face, new_pos, facing.direction_left());
    }
    // .X
    // O-
    if let Some(new_face) = face_grid.get(
        &face_pos
            .get_point_in_direction(&facing, 1)
            .get_point_in_direction(&facing.direction_right(), 1),
    ) {
        let new_pos = calculate_flipped_wrap(&pos, &facing, face_size, 1);
        return (*new_face, new_pos, facing.direction_right());
    }

    // X..
    // --O
    if let Some(new_face) = face_grid.get(
        &face_pos
            .get_point_in_direction(&facing, 1)
            .get_point_in_direction(&facing.direction_left(), 2),
    ) {
        let new_pos = calculate_flipped_wrap(&pos, &facing, face_size, 2);
        return (*new_face, new_pos, facing.direction_left().direction_left());
    }

    // ..X
    // O--
    if let Some(new_face) = face_grid.get(
        &face_pos
            .get_point_in_direction(&facing, 1)
            .get_point_in_direction(&facing.direction_right(), 2),
    ) {
        let new_pos = calculate_flipped_wrap(&pos, &facing, face_size, 2);
        return (
            *new_face,
            new_pos,
            facing.direction_right().direction_right(),
        );
    }

    // X...
    // ---O
    if let Some(new_face) = face_grid.get(
        &face_pos
            .get_point_in_direction(&facing, 1)
            .get_point_in_direction(&facing.direction_left(), 3),
    ) {
        let new_pos = calculate_flipped_wrap(&pos, &facing, face_size, 1);
        return (
            *new_face,
            new_pos,
            facing.direction_left().direction_left().direction_left(),
        );
    }

    // ...X
    // O---
    if let Some(new_face) = face_grid.get(
        &face_pos
            .get_point_in_direction(&facing, 1)
            .get_point_in_direction(&facing.direction_right(), 3),
    ) {
        let new_pos = calculate_flipped_wrap(&pos, &facing, face_size, 3);
        return (
            *new_face,
            new_pos,
            facing.direction_right().direction_right().direction_right(),
        );
    }

    // O
    // |
    // |
    // X
    if let Some(new_face) = face_grid
        .get(&face_pos.get_point_in_direction(&facing.direction_left().direction_left(), 3))
    {
        let new_pos = calculate_flipped_wrap(&pos, &facing, face_size, 0);
        return (*new_face, new_pos, facing);
    }

    //  O
    //  |
    // ||
    // X
    if let Some(new_face) = face_grid.get(
        &face_pos
            .get_point_in_direction(&facing.direction_left(), 1)
            .get_point_in_direction(&facing.direction_left().direction_left(), 3),
    ) {
        if face_grid
            .get(&face_pos.get_point_in_direction(&facing.direction_left().direction_left(), 2))
            .is_some()
        {
            let new_pos = calculate_flipped_wrap(&pos, &facing, face_size, 1);
            return (
                *new_face,
                new_pos,
                facing.direction_left().direction_left().direction_left(),
            );
        }
    }
    //  O
    //  |
    //  ||
    //   X
    if let Some(new_face) = face_grid.get(
        &face_pos
            .get_point_in_direction(&facing.direction_right(), 1)
            .get_point_in_direction(&facing.direction_right().direction_right(), 3),
    ) {
        if face_grid
            .get(&face_pos.get_point_in_direction(&facing.direction_left().direction_left(), 2))
            .is_some()
        {
            let new_pos = calculate_flipped_wrap(&pos, &facing, face_size, 3);
            return (
                *new_face,
                new_pos,
                facing.direction_right().direction_right().direction_right(),
            );
        }
    }

    //   O
    // X--
    if let Some(new_face) = face_grid.get(
        &face_pos
            .get_point_in_direction(&facing.direction_left(), 2)
            .get_point_in_direction(&facing.direction_left().direction_left(), 1),
    ) {
        if face_grid
            .get(&face_pos.get_point_in_direction(&facing.direction_left().direction_left(), 1))
            .is_some()
        {
            let new_pos = calculate_flipped_wrap(&pos, &facing, face_size, 2);
            return (*new_face, new_pos, facing.direction_left().direction_left());
        }
    }
    // O
    // --X
    if let Some(new_face) = face_grid.get(
        &face_pos
            .get_point_in_direction(&facing.direction_right(), 2)
            .get_point_in_direction(&facing.direction_right().direction_right(), 1),
    ) {
        if face_grid
            .get(&face_pos.get_point_in_direction(&facing.direction_right().direction_right(), 1))
            .is_some()
        {
            let new_pos = calculate_flipped_wrap(&pos, &facing, face_size, 2);
            return (
                *new_face,
                new_pos,
                facing.direction_right().direction_right(),
            );
        }
    }
    //    O
    // X---
    if let Some(new_face) = face_grid.get(
        &face_pos
            .get_point_in_direction(&facing.direction_left(), 3)
            .get_point_in_direction(&facing.direction_left().direction_left(), 1),
    ) {
        let new_pos = calculate_flipped_wrap(&pos, &facing, face_size, 1);
        return (
            *new_face,
            new_pos,
            facing.direction_left().direction_left().direction_left(),
        );
    }
    // O
    // ---X
    if let Some(new_face) = face_grid.get(
        &face_pos
            .get_point_in_direction(&facing.direction_right(), 3)
            .get_point_in_direction(&facing.direction_right().direction_right(), 1),
    ) {
        let new_pos = calculate_flipped_wrap(&pos, &facing, face_size, 3);
        return (
            *new_face,
            new_pos,
            facing.direction_right().direction_right().direction_right(),
        );
    }

    //  |O
    //  |
    // ||
    // X
    if let Some(new_face) = face_grid.get(
        &face_pos
            .get_point_in_direction(&facing.direction_left(), 2)
            .get_point_in_direction(&facing.direction_left().direction_left(), 3),
    ) {
        let new_pos = calculate_flipped_wrap(&pos, &facing, face_size, 0);
        return (*new_face, new_pos, facing);
    }
    // O|
    //  |
    //  ||
    //   X
    if let Some(new_face) = face_grid.get(
        &face_pos
            .get_point_in_direction(&facing.direction_right(), 2)
            .get_point_in_direction(&facing.direction_right().direction_right(), 3),
    ) {
        let new_pos = calculate_flipped_wrap(&pos, &facing, face_size, 0);
        return (*new_face, new_pos, facing);
    }

    println!("Not implemented, for {} in direction {}", face_pos, facing);
    unimplemented!()
}

fn calculate_flipped_wrap(
    pos: &Point<isize>,
    direction: &PointDirection,
    face_size: &isize,
    turn_clockwise_amount: usize,
) -> Point<isize> {
    match turn_clockwise_amount % 4 {
        0 => match direction {
            PointDirection::North => Point {
                x: pos.x,
                y: face_size - 1,
            },
            PointDirection::East => Point { x: 0, y: pos.y },
            PointDirection::South => Point { x: pos.x, y: 0 },
            PointDirection::West => Point {
                x: face_size - 1,
                y: pos.y,
            },
            _ => unimplemented!(),
        },
        1 => match direction.direction_right() {
            PointDirection::North => Point {
                x: face_size - 1 - pos.y,
                y: face_size - 1,
            },
            PointDirection::East => Point { x: 0, y: pos.x },
            PointDirection::South => Point {
                x: face_size - 1 - pos.y,
                y: 0,
            },
            PointDirection::West => Point {
                x: face_size - 1,
                y: pos.x,
            },
            _ => unimplemented!(),
        },
        2 => match direction.direction_right().direction_right() {
            PointDirection::North => Point {
                x: face_size - 1 - pos.x,
                y: face_size - 1,
            },
            PointDirection::East => Point {
                x: 0,
                y: face_size - 1 - pos.y,
            },
            PointDirection::South => Point {
                x: face_size - 1 - pos.x,
                y: 0,
            },
            PointDirection::West => Point {
                x: face_size - 1,
                y: face_size - 1 - pos.y,
            },
            _ => unimplemented!(),
        },
        3 => match direction
            .direction_right()
            .direction_right()
            .direction_right()
        {
            PointDirection::North => Point {
                x: pos.y,
                y: face_size - 1,
            },
            PointDirection::East => Point {
                x: 0,
                y: face_size - 1 - pos.x,
            },
            PointDirection::South => Point { x: pos.y, y: 0 },
            PointDirection::West => Point {
                x: face_size - 1,
                y: face_size - 1 - pos.x,
            },
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

fn calc_password(pos: &Point<isize>, facing: &PointDirection) -> isize {
    let facing_points = match facing {
        PointDirection::North => 3,
        PointDirection::East => 0,
        PointDirection::South => 1,
        PointDirection::West => 2,
        _ => unimplemented!(),
    };
    1000 * (pos.y + 1) + 4 * (pos.x + 1) + facing_points
}

fn print_grid_with_path(
    grid: &PointGrid<Content>,
    path: &[(Point<isize>, PointDirection)],
) -> String {
    let (min, max) = grid.dimensions();
    let mut result = String::new();
    result = format!("Grid ({}, {}): \n", min, max);
    for y in min.y..(max.y + 1) {
        for x in min.x..(max.x + 1) {
            if let Some((_, direction)) = path.iter().find(|(p, _)| *p == Point { x, y }) {
                result = format!("{}{}", result, direction);
            } else if let Some(u) = grid.get(&Point { x, y }) {
                result = format!("{}{}", result, u);
            } else {
                result = format!("{} ", result);
            }
        }
        result = format!("{}\n", result);
    }

    result
}

fn print_grid_with_path_cubed(
    grid: &PointGrid<Content>,
    face_grid: &PointGrid<usize>,
    face_size: isize,
    path: &HashMap<(usize, Point<isize>), PointDirection>,
) {
    let (min, max) = grid.dimensions();
    println!("Grid ({}, {}):", min, max);
    for y in min.y..(max.y + 1) {
        for x in min.x..(max.x + 1) {
            if let Some(face) = face_grid.get(&Point {
                x: x / face_size,
                y: y / face_size,
            }) {
                if let Some(direction) = path.get(&(
                    *face,
                    Point {
                        x: x % face_size,
                        y: y % face_size,
                    },
                )) {
                    print!("{}", direction);
                    continue;
                }
            }
            if let Some(u) = grid.get(&Point { x, y }) {
                print!("{}", u);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

pub fn part_one(_input: &str) -> Option<isize> {
    let (grid, actions) = parse_input(_input);
    // println!("{}", grid);
    // println!("{:?}", actions);

    let (mut current, mut facing) = get_start_position(&grid);
    let mut path: HashMap<Point<isize>, PointDirection> = HashMap::new();
    path.insert(current, facing.clone());

    for a in actions {
        (current, facing) = do_action(&grid, current, facing, &a, &mut path);
    }

    // println!(
    //     "{}",
    //     print_grid_with_path(&grid, &path.into_iter().collect_vec())
    // );

    Some(calc_password(&current, &facing))
}

pub fn part_two(_input: &str) -> Option<isize> {
    let (grid, actions) = parse_input(_input);
    let (faces, face_grid, face_size) = convert_into_cube_grids(&grid);
    // println!("face_grid: \n{}", face_grid);
    // for (i, pg) in faces.iter().enumerate() {
    //     println!("face {}: \n{}", i, pg);
    // }

    let (mut current_face, mut current_pos, mut facing) =
        (0, Point { x: 0, y: 0 }, PointDirection::East);
    let mut path: HashMap<(usize, Point<isize>), PointDirection> = HashMap::new();
    path.insert((0, current_pos), facing.clone());

    for a in actions {
        // let old_face = current_face;
        // let old_facing = facing.clone();
        // let before = print_grid_with_path(
        //     &faces[current_face],
        //     &path
        //         .iter()
        //         .filter(|((f, _), _)| *f == current_face)
        //         .map(|((_, p), d)| (*p, d.clone()))
        //         .collect_vec(),
        // );
        (current_face, current_pos, facing) = do_action_cube(
            &face_grid,
            &faces,
            &face_size,
            current_face,
            current_pos,
            facing,
            &a,
            &mut path,
        );
        // let after = print_grid_with_path(
        //     &faces[current_face],
        //     &path
        //         .iter()
        //         .filter(|((f, _), _)| *f == current_face)
        //         .map(|((_, p), d)| (*p, d.clone()))
        //         .collect_vec(),
        // );

        // if old_face != current_face {
        //     print!("\x1B[2J\x1B[1;1H");
        //     println!("Action: {:?}", a);
        //     println!("Face {}, facing {}", old_face, old_facing);
        //     println!("Face {} (After), facing {}", current_face, facing);
        //     for (bl, al) in before.lines().zip(after.lines()) {
        //         println!("{}          {}", bl, al);
        //     }

        //     let _ = io::stdin().read(&mut [0u8]).unwrap();

        //     println!();
        //     println!();
        //     println!();
        // }
    }

    // print_grid_with_path_cubed(&grid, &face_grid, face_size, &path);

    let face_pos = face_grid
        .points
        .iter()
        .find(|(_, i)| **i == current_face)
        .unwrap()
        .0;
    let calc_pos = Point {
        x: face_pos.x * face_size + current_pos.x,
        y: face_pos.y * face_size + current_pos.y,
    };

    Some(calc_password(&calc_pos, &facing))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), Some(5031));
    }
}
