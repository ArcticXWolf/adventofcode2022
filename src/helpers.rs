use std::{
    collections::HashMap,
    fmt::{self, Display},
    ops::{Add, Sub},
    slice::Iter,
};

use parse_display::{Display, FromStr};
/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

#[derive(Debug, Clone, Display, FromStr)]
pub enum PointDirection {
    #[display("^")]
    North,
    #[display("/")]
    NorthEast,
    #[display(">")]
    East,
    #[display("\\")]
    SouthEast,
    #[display("v")]
    South,
    #[display("/")]
    SouthWest,
    #[display("<")]
    West,
    #[display("\\")]
    NorthWest,
}

impl PointDirection {
    pub fn all_with_diagonals() -> Iter<'static, PointDirection> {
        static D: [PointDirection; 8] = [
            PointDirection::North,
            PointDirection::NorthEast,
            PointDirection::East,
            PointDirection::SouthEast,
            PointDirection::South,
            PointDirection::SouthWest,
            PointDirection::West,
            PointDirection::NorthWest,
        ];

        D.iter()
    }

    pub fn all() -> Iter<'static, PointDirection> {
        static D: [PointDirection; 4] = [
            PointDirection::North,
            PointDirection::East,
            PointDirection::South,
            PointDirection::West,
        ];

        D.iter()
    }

    pub fn direction_left(&self) -> Self {
        match self {
            PointDirection::North => PointDirection::West,
            PointDirection::East => PointDirection::North,
            PointDirection::South => PointDirection::East,
            PointDirection::West => PointDirection::South,
            _ => unimplemented!(),
        }
    }

    pub fn direction_right(&self) -> Self {
        match self {
            PointDirection::North => PointDirection::East,
            PointDirection::East => PointDirection::South,
            PointDirection::South => PointDirection::West,
            PointDirection::West => PointDirection::North,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display, FromStr)]
#[display("({x},{y})")]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point<isize> {
    pub fn get_point_in_direction(&self, direction: &PointDirection, distance: isize) -> Self {
        match direction {
            PointDirection::North => Self {
                x: self.x,
                y: self.y - distance,
            },
            PointDirection::NorthEast => Self {
                x: self.x - distance,
                y: self.y - distance,
            },
            PointDirection::East => Self {
                x: self.x + distance,
                y: self.y,
            },
            PointDirection::SouthEast => Self {
                x: self.x + distance,
                y: self.y + distance,
            },
            PointDirection::South => Self {
                x: self.x,
                y: self.y + distance,
            },
            PointDirection::SouthWest => Self {
                x: self.x - distance,
                y: self.y + distance,
            },
            PointDirection::West => Self {
                x: self.x - distance,
                y: self.y,
            },
            PointDirection::NorthWest => Self {
                x: self.x - distance,
                y: self.y - distance,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct PointGrid<U> {
    pub points: HashMap<Point<isize>, U>,
}

impl<U> Default for PointGrid<U> {
    fn default() -> Self {
        Self {
            points: HashMap::new(),
        }
    }
}

impl<U> PointGrid<U> {
    pub fn insert(&mut self, coord: Point<isize>, value: U) {
        self.points.insert(coord, value);
    }
    pub fn get(&self, coord: &Point<isize>) -> Option<&U> {
        self.points.get(coord)
    }

    pub fn dimensions(&self) -> (Point<isize>, Point<isize>) {
        (
            Point {
                x: self.points.keys().map(|p| p.x).min().unwrap(),
                y: self.points.keys().map(|p| p.y).min().unwrap(),
            },
            Point {
                x: self.points.keys().map(|p| p.x).max().unwrap(),
                y: self.points.keys().map(|p| p.y).max().unwrap(),
            },
        )
    }

    pub fn wrap_around(&self, point: &Point<isize>, direction: &PointDirection) -> Point<isize> {
        match direction {
            PointDirection::North => *self
                .points
                .keys()
                .filter(|p| p.x == point.x)
                .max_by_key(|p| p.y)
                .unwrap(),
            PointDirection::East => *self
                .points
                .keys()
                .filter(|p| p.y == point.y)
                .min_by_key(|p| p.x)
                .unwrap(),
            PointDirection::South => *self
                .points
                .keys()
                .filter(|p| p.x == point.x)
                .min_by_key(|p| p.y)
                .unwrap(),
            PointDirection::West => *self
                .points
                .keys()
                .filter(|p| p.y == point.y)
                .max_by_key(|p| p.x)
                .unwrap(),
            _ => unimplemented!(),
        }
    }
}

impl<U> Display for PointGrid<U>
where
    U: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (min, max) = self.dimensions();
        writeln!(f, "Grid ({}, {}):", min, max)?;
        for y in min.y..(max.y + 1) {
            for x in min.x..(max.x + 1) {
                if let Some(u) = self.get(&Point { x, y }) {
                    write!(f, "{}", u)?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Add<Output = T>> Add for Point3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Point3<i32> {
    pub fn directions() -> Vec<Point3<i32>> {
        vec![
            Point3 { x: -1, y: 0, z: 0 }, // West
            Point3 { x: 1, y: 0, z: 0 },  // East
            Point3 { x: 0, y: -1, z: 0 }, // South
            Point3 { x: 0, y: 1, z: 0 },  // North
            Point3 { x: 0, y: 0, z: -1 }, // Above
            Point3 { x: 0, y: 0, z: 1 },  // Below
        ]
    }
}
