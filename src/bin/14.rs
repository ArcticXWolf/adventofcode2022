use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::fmt;

#[derive(Display, FromStr, Debug, PartialEq, Clone)]
#[display("{x},{y}")]
struct Coord2D {
    x: usize,
    y: usize,
}

impl Coord2D {
    fn get_line_points(start: &Coord2D, end: &Coord2D) -> Vec<Coord2D> {
        let mut result = vec![];
        let (minx, maxx) = (start.x.min(end.x), start.x.max(end.x));
        let (miny, maxy) = (start.y.min(end.y), start.y.max(end.y));

        for x in minx..=maxx {
            for y in miny..=maxy {
                result.push(Coord2D { x, y })
            }
        }

        result
    }

    fn get_next_sand_positions(&self) -> Vec<Coord2D> {
        vec![
            Coord2D {
                x: self.x,
                y: self.y + 1,
            },
            Coord2D {
                x: self.x - 1,
                y: self.y + 1,
            },
            Coord2D {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }
}

type Path = Vec<Coord2D>;

#[derive(Clone)]
enum Content {
    Rock,
    Sand,
}

struct Grid {
    content: Vec<Option<Content>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(lines: Vec<Path>, bottomless: bool) -> Self {
        let realwidth = lines.iter().flatten().max_by_key(|c| c.x).unwrap().x + 1;
        let width = realwidth * 2;
        let realheight = lines.iter().flatten().max_by_key(|c| c.y).unwrap().y + 1;
        let height = if bottomless {
            realheight
        } else {
            realheight + 2
        };

        let mut content = Vec::with_capacity(width * height);
        content.resize_with(width * height, || None::<Content>);

        let mut grid = Grid {
            content,
            width,
            height,
        };

        for path in lines {
            grid.draw_path(&path);
        }

        if !bottomless {
            grid.draw_path(&vec![
                Coord2D {
                    x: 0,
                    y: height - 1,
                },
                Coord2D {
                    x: width - 1,
                    y: height - 1,
                },
            ]);
        }

        grid
    }

    fn draw_path(&mut self, path: &Path) {
        for (a, b) in path.iter().tuple_windows() {
            for coord in Coord2D::get_line_points(a, b) {
                self.draw_position(&coord, Some(Content::Rock));
            }
        }
    }

    fn draw_position(&mut self, pos: &Coord2D, value: Option<Content>) {
        let index = pos.y * self.width + pos.x;
        self.content[index] = value;
    }

    fn get_content_at_pos(&self, pos: &Coord2D) -> Option<Content> {
        let index = pos.y * self.width + pos.x;
        self.content[index].clone()
    }

    // returns true if sand comes to rest, false if out of bounds
    fn pour_sand(&mut self, pos: &Coord2D) -> bool {
        let mut sand_position: Coord2D = pos.clone();
        'outer: loop {
            for new_pos in sand_position.get_next_sand_positions() {
                if new_pos.y >= self.height {
                    return false;
                }

                match self.get_content_at_pos(&new_pos) {
                    None => {
                        sand_position = new_pos;
                        continue 'outer;
                    }
                    Some(_) => {}
                }
            }
            self.draw_position(&sand_position, Some(Content::Sand));
            break sand_position != *pos;
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if y == 0 && x == 500 {
                    write!(f, "+")?;
                    continue;
                }
                let index = y * self.width + x;
                let draw_character = match self.content[index] {
                    Some(Content::Rock) => "#",
                    Some(Content::Sand) => "O",
                    None => ".",
                };
                write!(f, "{}", draw_character)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

pub fn part_one(_input: &str) -> Option<u32> {
    let rock_paths = _input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|c| c.parse::<Coord2D>().unwrap())
                .collect_vec() as Path
        })
        .collect_vec();

    let mut grid = Grid::new(rock_paths, true);
    let mut tracker = 0;

    while grid.pour_sand(&Coord2D { x: 500, y: 0 }) {
        // println!("{}", grid);
        tracker += 1;
    }

    Some(tracker)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let rock_paths = _input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|c| c.parse::<Coord2D>().unwrap())
                .collect_vec() as Path
        })
        .collect_vec();

    let mut grid = Grid::new(rock_paths, false);
    let mut tracker = 0;

    while grid.pour_sand(&Coord2D { x: 500, y: 0 }) {
        // println!("{}", grid);
        tracker += 1;
    }
    tracker += 1;

    Some(tracker)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
