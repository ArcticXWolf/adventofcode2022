use std::fmt;

use itertools::Itertools;

type CargoStack = Vec<char>;

struct Ship {
    stacks: Vec<CargoStack>,
}

impl Ship {
    fn execute_instruction(&mut self, inst: &Instruction) {
        for _ in 0..inst.amount_of_crates {
            let moving_crate = self.stacks[inst.starting_pos as usize].pop().unwrap();
            self.stacks[inst.ending_pos as usize].push(moving_crate);
        }
    }

    fn execute_instruction9001(&mut self, inst: &Instruction) {
        let splitat =
            self.stacks[inst.starting_pos as usize].len() - inst.amount_of_crates as usize;
        let mut moving_crates = self.stacks[inst.starting_pos as usize].split_off(splitat);
        self.stacks[inst.ending_pos as usize].append(&mut moving_crates);
    }

    fn top_of_stacks(&self) -> String {
        let mut result = String::new();
        for s in self.stacks.iter() {
            result = format!("{}{}", result, s.last().unwrap());
        }
        result
    }
}

impl TryFrom<Vec<&str>> for Ship {
    type Error = &'static str;
    fn try_from(input: Vec<&str>) -> Result<Self, Self::Error> {
        if input.is_empty() {
            return Err("Input array is empty.");
        }
        let number_of_stacks = input[input.len() - 1]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let mut stacks: Vec<CargoStack> = vec![];
        let stack_drawing = input.iter().rev().skip(1).copied().collect_vec();

        for i in 0..number_of_stacks {
            let mut stack: CargoStack = vec![];
            for l in &stack_drawing {
                let crate_key = l.chars().nth(i as usize * 4 + 1).unwrap();
                if crate_key != ' ' {
                    stack.push(crate_key);
                }
            }
            stacks.push(stack);
        }

        Ok(Self { stacks })
    }
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for s in self.stacks.iter() {
            write!(f, "Stack: ")?;
            for c in s {
                write!(f, "[{}]", c)?;
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

#[derive(Debug)]
struct Instruction {
    amount_of_crates: u32,
    starting_pos: u32,
    ending_pos: u32,
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let splits = input.split_whitespace().collect_vec();
        if splits.len() != 6 {
            return Err("Wrong instruction format.");
        }
        Ok(Self {
            amount_of_crates: splits.get(1).unwrap().parse::<u32>().unwrap(),
            starting_pos: splits.get(3).unwrap().parse::<u32>().unwrap() - 1,
            ending_pos: splits.get(5).unwrap().parse::<u32>().unwrap() - 1,
        })
    }
}

fn split_stacks_and_instructions(input: &str) -> (Ship, Vec<Instruction>) {
    let (lines_ship, lines_instructions) = input.split_once("\n\n").unwrap();

    (
        Ship::try_from(lines_ship.lines().collect_vec()).unwrap(),
        lines_instructions
            .lines()
            .map(|l| Instruction::try_from(l).unwrap())
            .collect_vec(),
    )
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut ship, instructions) = split_stacks_and_instructions(input);

    for i in instructions {
        ship.execute_instruction(&i);
    }

    Some(ship.top_of_stacks())
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut ship, instructions) = split_stacks_and_instructions(input);

    for i in instructions {
        ship.execute_instruction9001(&i);
    }

    Some(ship.top_of_stacks())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
