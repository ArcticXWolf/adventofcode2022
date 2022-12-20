use itertools::Itertools;

struct IndexedList {
    numbers: Vec<i64>,
    indices: Vec<usize>,
}

impl IndexedList {
    fn new() -> Self {
        Self {
            numbers: vec![],
            indices: vec![],
        }
    }

    fn move_number(&mut self, numbers_vec_index: usize) {
        let amount_movement = self.numbers[numbers_vec_index];
        let old_position = self.indices[numbers_vec_index];
        let new_position = (old_position as i64 + amount_movement)
            .rem_euclid(self.numbers.len() as i64 - 1) as usize;

        if old_position == new_position {
            return;
        }
        if old_position < new_position {
            for element in self.indices.iter_mut() {
                if *element > old_position && *element <= new_position {
                    *element -= 1;
                }
            }
            self.indices[numbers_vec_index] = new_position;
        } else {
            for element in self.indices.iter_mut() {
                if *element < old_position && *element >= new_position {
                    *element += 1;
                }
            }
            self.indices[numbers_vec_index] = new_position;
        }
    }

    fn return_numbers_in_order(&self) -> Vec<i64> {
        self.numbers
            .iter()
            .zip(self.indices.iter())
            .sorted_by_key(|x| x.1)
            .map(|x| *x.0)
            .collect_vec()
    }
}

fn parse_input(input: &str) -> IndexedList {
    let mut list = IndexedList::new();
    list.numbers = input
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();
    list.indices = (0..list.numbers.len()).collect_vec();
    list
}

fn mix(list: &mut IndexedList) {
    for i in 0..list.numbers.len() {
        list.move_number(i);
    }
}

fn extract_coordinates(list: &IndexedList) -> (i64, i64, i64) {
    let zero_vec_index = list.numbers.iter().find_position(|i| **i == 0).unwrap().0;

    let x_vec_index = list
        .indices
        .iter()
        .find_position(|i| **i == ((list.indices[zero_vec_index] + 1000) % list.numbers.len()))
        .unwrap()
        .0;
    let x = list.numbers[x_vec_index];
    let y_vec_index = list
        .indices
        .iter()
        .find_position(|i| **i == ((list.indices[zero_vec_index] + 2000) % list.numbers.len()))
        .unwrap()
        .0;
    let y = list.numbers[y_vec_index];
    let z_vec_index = list
        .indices
        .iter()
        .find_position(|i| **i == ((list.indices[zero_vec_index] + 3000) % list.numbers.len()))
        .unwrap()
        .0;
    let z = list.numbers[z_vec_index];
    (x, y, z)
}

pub fn part_one(_input: &str) -> Option<i64> {
    let mut list = parse_input(_input);

    mix(&mut list);

    let (x, y, z) = extract_coordinates(&list);

    Some(x + y + z)
}

pub fn part_two(_input: &str) -> Option<i64> {
    let mut list = parse_input(_input);

    for element in list.numbers.iter_mut() {
        *element *= 811589153;
    }

    for _ in 0..10 {
        mix(&mut list);
    }

    let (x, y, z) = extract_coordinates(&list);

    Some(x + y + z)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
