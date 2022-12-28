use std::fmt;

fn convert_into_snafu(input: usize) -> String {
    let base: usize = 5;
    let mut result = String::new();
    let mut current_base_exp = 0;

    while base.pow(current_base_exp) <= input + base.pow(current_base_exp) / 2 {
        let digit = ((input + base.pow(current_base_exp) / 2) % base.pow(current_base_exp + 1))
            / base.pow(current_base_exp);
        result = format!("{}{}", convert_digit_to_snafu_digit(digit), result);
        current_base_exp += 1;
    }
    result
}

fn convert_digit_to_snafu_digit(input: usize) -> char {
    match input {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '=',
        4 => '-',
        _ => unimplemented!(),
    }
}

fn convert_from_snafu(input: &str) -> usize {
    let base: usize = 5;
    let mut result: isize = 0;

    for (current_exp, c) in input.trim().chars().rev().enumerate() {
        result += convert_snafu_digit_to_digit(c) * base.pow(current_exp as u32) as isize;
    }

    result as usize
}

fn convert_snafu_digit_to_digit(input: char) -> isize {
    match input {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '=' => -2,
        '-' => -1,
        _ => unimplemented!(),
    }
}

pub fn part_one(_input: &str) -> Option<String> {
    let sum: usize = _input.lines().map(convert_from_snafu).sum();
    println!("Result decimal: {}", sum);
    Some(convert_into_snafu(sum))
}

pub fn part_two(_input: &str) -> Option<u32> {
    // Nothing to do, Yay!
    Some(0)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), Some(0));
    }
}
