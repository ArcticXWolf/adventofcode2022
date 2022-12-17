use advent_of_code::helpers::Point;

// Shape and Rock structs ------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum Shape {
    HorizontalLine,
    Cross,
    MirroredL,
    VerticalLine,
    Square,
}

impl Shape {
    pub fn cycle(cycle: usize) -> Shape {
        const SHAPES: [Shape; 5] = [
            Shape::HorizontalLine,
            Shape::Cross,
            Shape::MirroredL,
            Shape::VerticalLine,
            Shape::Square,
        ];
        SHAPES[cycle].clone()
    }

    fn height(&self) -> i64 {
        match self {
            Shape::HorizontalLine => 1,
            Shape::Cross => 3,
            Shape::MirroredL => 3,
            Shape::VerticalLine => 4,
            Shape::Square => 2,
        }
    }

    fn width(&self) -> i64 {
        match self {
            Shape::HorizontalLine => 4,
            Shape::Cross => 3,
            Shape::MirroredL => 3,
            Shape::VerticalLine => 1,
            Shape::Square => 2,
        }
    }

    fn points(&self) -> Vec<Point<i64>> {
        match self {
            Shape::HorizontalLine => vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 },
            ],
            Shape::Cross => vec![
                Point { x: 1, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
                Point { x: 1, y: 2 },
            ],
            Shape::MirroredL => vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 2, y: 1 },
                Point { x: 2, y: 2 },
            ],
            Shape::VerticalLine => vec![
                Point { x: 0, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 0, y: 2 },
                Point { x: 0, y: 3 },
            ],
            Shape::Square => vec![
                Point { x: 0, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 0 },
                Point { x: 1, y: 1 },
            ],
        }
    }
}

struct Rock {
    origin: Point<i64>, // bottom left square of bounding box
    shape: Shape,
}

impl Rock {
    fn spawn(shape: Shape, height: i64) -> Self {
        Rock {
            origin: Point { x: 2, y: height },
            shape,
        }
    }

    fn push(&mut self, direction: Point<i64>, rocks: &[Rock]) {
        self.move_direction(direction, rocks);
    }

    fn fall(&mut self, rocks: &[Rock]) -> bool {
        self.move_direction(Point { x: 0, y: -1 }, rocks)
    }

    fn move_direction(&mut self, direction: Point<i64>, rocks: &[Rock]) -> bool {
        if (direction.y < 0 && self.origin.y <= 0)
            || (direction.x < 0 && self.origin.x <= 0)
            || (direction.x > 0 && (self.origin.x + self.width()) >= 7)
        {
            return false;
        }

        self.origin = self.origin + direction;
        for r in rocks.iter() {
            if r.collide_with_bounding_box(self) && r.collide_precise(self) {
                self.origin = self.origin - direction;
                return false;
            }
        }
        true
    }

    fn height(&self) -> i64 {
        self.shape.height()
    }

    fn width(&self) -> i64 {
        self.shape.width()
    }

    fn collide_with_bounding_box(&self, other: &Rock) -> bool {
        ((self.origin.x + self.width() / 2).abs_diff(other.origin.x + other.width() / 2)) as i64 * 2
            < (self.width() + other.width())
            && ((self.origin.y + self.height() / 2).abs_diff(other.origin.y + other.height() / 2))
                as i64
                * 2
                < (self.height() + other.height())
    }

    fn collide_precise(&self, other: &Rock) -> bool {
        self.real_points()
            .iter()
            .any(|op| other.real_points().contains(op))
    }

    fn points(&self) -> Vec<Point<i64>> {
        self.shape.points()
    }

    fn real_points(&self) -> Vec<Point<i64>> {
        self.points().iter().map(|p| self.origin + *p).collect()
    }
}

// Helpers ---------------------------------------------------------------------

fn parse_input(input: &str) -> Vec<Point<i64>> {
    input
        .chars()
        .flat_map(|c| match c {
            '>' => Some(Point { x: 1, y: 0 }),
            '<' => Some(Point { x: -1, y: 0 }),
            _ => None,
        })
        .collect()
}

fn check_cycles(rocks: &[Rock]) -> Option<(i64, i64, i64)> {
    // i = index first rock in sequence
    // j+1 = length of sequence (because j is the offset after i)
    // k = multiplier for checking multiple cycles (k=1 check first cycle; k=2 check second; ...)
    // l = offset inside a cycle to check
    for (i, origin_rock) in rocks.iter().enumerate() {
        'inner: for (j, compare_rock) in rocks.iter().skip(i + 1).enumerate() {
            // Find any two "same" rocks in list
            if origin_rock.shape != compare_rock.shape
                || origin_rock.origin.x != compare_rock.origin.x
            {
                continue;
            }

            // Check if the rock sequence between both is the same for the next 5 cycles
            for k in 1..5 {
                for l in 0..(j + 1) {
                    if (i + k * (j + 1) + l) >= rocks.len() {
                        continue 'inner;
                    }

                    if rocks[i + l].shape != rocks[i + k * (j + 1) + l].shape
                        || rocks[i + l].origin.x != rocks[i + k * (j + 1) + l].origin.x
                    {
                        continue 'inner;
                    }
                }
            }

            return Some((
                compare_rock.origin.y - origin_rock.origin.y,
                (j + 1) as i64,
                (i + 1) as i64,
            ));
        }
    }

    None
}

fn print_chamber(rocks: &[Rock], current_rock: Option<&Rock>) {
    let height = rocks
        .iter()
        .map(|r| r.origin.y + r.height() + 8)
        .max()
        .unwrap_or(0)
        .max(10);

    for y in (0..height).rev() {
        print!("|");
        for x in 0..7 {
            let position = Point { x, y };
            if current_rock.is_some()
                && current_rock
                    .unwrap()
                    .real_points()
                    .iter()
                    .any(|p| *p == position)
            {
                print!("@");
            } else if rocks
                .iter()
                .any(|r| r.real_points().iter().any(|p| *p == position))
            {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("|");
        print!(" {}", y);
        println!();
    }
    println!("+-------+");
    println!();
}

// Tasks -----------------------------------------------------------------------

pub fn part_one(_input: &str) -> Option<u64> {
    let mut rocks: Vec<Rock> = vec![];
    let directions = parse_input(_input);
    let mut current_max_height = 0;
    let mut shapes_cycle = 0;
    let mut directions_cycle = 0;

    for _ in 0..2022 {
        let shape = Shape::cycle(shapes_cycle);
        shapes_cycle = (shapes_cycle + 1) % 5;
        let mut new_rock = Rock::spawn(shape, current_max_height + 3);

        loop {
            let direction = directions[directions_cycle];
            new_rock.push(direction, &rocks);
            directions_cycle = (directions_cycle + 1) % directions.len();

            if !new_rock.fall(&rocks) {
                break;
            }
        }
        rocks.push(new_rock);
        current_max_height = rocks.iter().map(|r| r.origin.y + r.height()).max().unwrap();
    }

    Some(current_max_height as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let mut rocks: Vec<Rock> = vec![];
    let directions = parse_input(_input);
    let mut current_max_height = 0;
    let mut shapes_cycle = 0;
    let mut directions_cycle = 0;

    // Find a cycle
    let (cycle_height, cycle_rock_count, cycle_offset) = loop {
        let shape = Shape::cycle(shapes_cycle);
        shapes_cycle = (shapes_cycle + 1) % 5;
        let mut new_rock = Rock::spawn(shape, current_max_height + 3);

        loop {
            let direction = directions[directions_cycle];
            new_rock.push(direction, &rocks);
            directions_cycle = (directions_cycle + 1) % directions.len();

            if !new_rock.fall(&rocks) {
                break;
            }
        }
        rocks.push(new_rock);
        current_max_height = rocks.iter().map(|r| r.origin.y + r.height()).max().unwrap();

        if current_max_height % 1000 == 0 {
            if let Some((cycle_height, cycle_rock_count, cycle_offset)) = check_cycles(&rocks) {
                break (cycle_height, cycle_rock_count, cycle_offset);
            }
        }
    };

    // Calculate the stats from cycles
    let rocks_wanted: i64 = 1000000000000;
    let amount_of_cycles = (rocks_wanted - cycle_offset) / cycle_rock_count;
    let height_of_cycles = cycle_height * amount_of_cycles;
    let amount_rocks_left = rocks_wanted - (amount_of_cycles * cycle_rock_count + cycle_offset);
    let partial_cycle_height = (rocks[(cycle_offset + amount_rocks_left) as usize].origin.y)
        - (rocks[(cycle_offset) as usize].origin.y);
    let total_height =
        rocks[(cycle_offset) as usize].origin.y + height_of_cycles + partial_cycle_height;

    Some(total_height as u64)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
