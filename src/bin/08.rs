use bitvec::prelude::*;
use itertools::Itertools;

pub fn part_one(_input: &str) -> Option<u32> {
    // create grid
    let grid = _input
        .lines()
        .flat_map(|l| l.chars().map(|c| c.to_string().parse::<i8>().unwrap()))
        .collect_vec();
    let (width, height) = (
        _input.lines().next().unwrap().chars().count(),
        _input.lines().count(),
    );

    // use vector with max values to keep track of highest tree per direction
    // use bitvector to count visible trees (to dont count trees double)
    let mut max_tree: Vec<Vec<i8>> = vec![
        vec![-1; height], // top
        vec![-1; width],  // right
        vec![-1; height], // bottom
        vec![-1; width],  // left
    ];
    let mut visibility = bitvec!(0; grid.len());

    // iterate forwards and check top and left
    for index in 0..grid.len() {
        let (x, y) = (index % width, index / width);
        let value = *grid.get(index).unwrap();

        // top
        if value > max_tree[0][x] {
            visibility.set(index, true);
            max_tree[0][x] = value;
        }

        // left
        if value > max_tree[3][y] {
            visibility.set(index, true);
            max_tree[3][y] = value;
        }
    }

    // iterate backwards and check right and bottom
    for index in (0..grid.len()).rev() {
        let (x, y) = (index % width, index / width);
        let value = *grid.get(index).unwrap();

        // bottom
        if value > max_tree[2][x] {
            visibility.set(index, true);
            max_tree[2][x] = value;
        }

        // right
        if value > max_tree[1][y] {
            visibility.set(index, true);
            max_tree[1][y] = value;
        }
    }

    Some(visibility.count_ones() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    // create grid
    let grid = _input
        .lines()
        .flat_map(|l| l.chars().map(|c| c.to_string().parse::<i32>().unwrap()))
        .collect_vec();
    let (width, height) = (
        _input.lines().next().unwrap().chars().count() as i32,
        _input.lines().count() as i32,
    );

    let directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut best_score = 0;

    // iterate all positions
    for index in 0..grid.len() as i32 {
        let (x, y) = (index % width, index / width);
        let value = *grid.get(index as usize).unwrap();
        let mut direction_score = vec![0, 0, 0, 0];

        // iterate all directions per position
        for (i, (dx, dy)) in directions.iter().enumerate() {
            let (mut cur_x, mut cur_y) = (x + dx, y + dy);
            let mut trees = 1;

            // move in direction until edge
            while 0 < cur_x && cur_x < width - 1 && 0 < cur_y && cur_y < height - 1 {
                // or until higher tree
                let cur_value = *grid.get((cur_y * width + cur_x) as usize).unwrap();
                if cur_value >= value {
                    break;
                }

                (cur_x, cur_y) = (cur_x + dx, cur_y + dy);
                trees += 1;
            }
            direction_score[i] = trees;
        }
        best_score = best_score.max(direction_score.iter().product());
    }
    Some(best_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
