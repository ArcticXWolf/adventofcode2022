use std::collections::{HashMap, HashSet};

use bitvec::macros::internal::funty::Fundamental;
use itertools::Itertools;
use priority_queue::PriorityQueue;

type Coord2D = (isize, isize);

fn neighbors(pos: Coord2D, grid: &Vec<Vec<u32>>) -> Vec<Coord2D> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(|d| add(pos, d))
        .filter(|c| {
            c.0 >= 0 && c.1 >= 0 && c.0 < grid[0].len() as isize && c.1 < grid.len() as isize
        })
        .collect_vec()
}

fn add(coord: Coord2D, other: Coord2D) -> Coord2D {
    (coord.0 + other.0, coord.1 + other.1)
}

fn distance(one: Coord2D, other: Coord2D) -> usize {
    // Manhatten distance for now
    ((one.0 - other.0).abs() + (one.1 - other.1).abs()) as usize
}

fn pathfinding(grid: &Vec<Vec<u32>>, start: Coord2D, end: Coord2D) -> Vec<Coord2D> {
    let mut closed_nodes: HashSet<Coord2D> = HashSet::new();
    let mut solution: HashMap<Coord2D, (Coord2D, u32)> = HashMap::new();
    let mut queue: PriorityQueue<(Coord2D, u32), usize> = PriorityQueue::new();
    let max_distance = grid.len() * grid[0].len();

    queue.push((start, 0), max_distance - distance(start, end));

    while !queue.is_empty() {
        let current_node = queue.pop();

        if let Some(((pos, current_pathlength), _)) = current_node {
            closed_nodes.insert(pos);

            for n in neighbors(pos, grid) {
                if closed_nodes.contains(&n) {
                    continue;
                }
                if grid[n.1 as usize][n.0 as usize] > grid[pos.1 as usize][pos.0 as usize] + 1 {
                    continue;
                }

                queue.push(
                    (n, current_pathlength + 1),
                    max_distance - (distance(n, end) + current_pathlength as usize),
                );
                if let Some((_, old_pathlength)) = solution.get(&n) {
                    if *old_pathlength > current_pathlength {
                        solution.insert(n, (pos, current_pathlength + 1));
                    }
                } else {
                    solution.insert(n, (pos, current_pathlength + 1));
                }
            }
        }
    }

    let mut current: Coord2D = end;
    let mut path = vec![current];
    while current != start {
        let solution_item = solution.get(&current).unwrap();
        current = solution_item.0;
        path.push(current);
    }
    path.reverse();

    path
}

fn parse_input(input: &str) -> (Vec<Vec<u32>>, Coord2D, Coord2D) {
    let start_coord_index = input
        .replace('\n', "")
        .char_indices()
        .find(|x| x.1 == 'S')
        .unwrap()
        .0;
    let end_coord_index = input
        .replace('\n', "")
        .char_indices()
        .find(|x| x.1 == 'E')
        .unwrap()
        .0;
    let grid = input
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'S' => 0,
                    'E' => 25,
                    x => x as u32 - 97,
                })
                .collect_vec()
        })
        .collect_vec();
    let start: Coord2D = (
        (start_coord_index % grid[0].len()) as isize,
        (start_coord_index / grid[0].len()) as isize,
    );
    let end: Coord2D = (
        (end_coord_index % grid[0].len()) as isize,
        (end_coord_index / grid[0].len()) as isize,
    );

    (grid, start, end)
}

fn get_all_possible_starting_locations(grid: &Vec<Vec<u32>>) -> Vec<Coord2D> {
    // just take all a locations that have a b location adjacent
    let mut locations = vec![];
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col != 0 {
                continue;
            }

            if neighbors((x as isize, y as isize), grid)
                .iter()
                .any(|(nx, ny)| grid[*ny as usize][*nx as usize] == 1)
            {
                locations.push((x as isize, y as isize));
            }
        }
    }
    locations
}

fn print_grid(grid: &[Vec<u32>], highlight: &[Coord2D]) {
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if highlight.contains(&(x as isize, y as isize)) {
                print!("\x1b[93m{}\x1b[0m", (col + 97).as_char().unwrap());
            } else {
                print!("{}", (col + 97).as_char().unwrap());
            }
        }
        println!();
    }
}

pub fn part_one(_input: &str) -> Option<u32> {
    let (grid, start, end) = parse_input(_input);
    let path = pathfinding(&grid, start, end);
    print_grid(&grid, &path);

    Some(path.len() as u32 - 1)
}

pub fn part_two(_input: &str) -> Option<u32> {
    // brute force over all a locations adjacent to b locations
    let (grid, _, end) = parse_input(_input);
    let possible_starts = get_all_possible_starting_locations(&grid);
    let mut best_path = vec![];
    for (i, s) in possible_starts.iter().enumerate() {
        let path = pathfinding(&grid, *s, end);
        if i == 0 || path.len() < best_path.len() {
            best_path = path;
        }
    }
    print_grid(&grid, &best_path);

    Some(best_path.len() as u32 - 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
