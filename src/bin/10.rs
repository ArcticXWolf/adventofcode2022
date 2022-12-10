use std::{fmt, num::ParseIntError};

use itertools::Itertools;

// Instruction -----------------------------------------------------------------

#[derive(Debug, Clone)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn get_clock_timing(&self) -> u32 {
        match self {
            Self::Noop => 1,
            Self::AddX(_) => 2,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExtractInstructionError {
    InvalidInstructionCode,
    InvalidInteger(ParseIntError),
}

impl From<ParseIntError> for ExtractInstructionError {
    fn from(e: ParseIntError) -> Self {
        Self::InvalidInteger(e)
    }
}

impl TryFrom<&str> for Instruction {
    type Error = ExtractInstructionError;
    fn try_from(value: &str) -> Result<Self, ExtractInstructionError> {
        let (code, rest) = match value.split_once(' ') {
            Some(x) => x,
            None => (value, ""),
        };

        match code {
            "noop" => Ok(Instruction::Noop),
            "addx" => {
                let x = rest.parse::<i32>()?;
                Ok(Instruction::AddX(x))
            }
            _ => {
                println!("Error unknown instruction \"{:?}\"", value);
                Err(ExtractInstructionError::InvalidInstructionCode)
            }
        }
    }
}

// CPU -------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Registers {
    x: i32,
}

struct CPU {
    last_instruction_clock: u32,
    clock: u32,
    program_counter: usize,
    code: Vec<Instruction>,
    registers: Registers,
}

impl CPU {
    fn new(instructions: Vec<Instruction>) -> Self {
        CPU {
            last_instruction_clock: 0,
            clock: 0,
            program_counter: 0,
            code: instructions,
            registers: Registers { x: 1 },
        }
    }

    fn tick(&mut self) -> bool {
        if self.waiting_for_next_instruction() {
            self.clock += 1;
            return true;
        }

        let instruction = match self.fetch_instruction() {
            Some(x) => x,
            None => return false,
        };

        self.execute(instruction);

        self.program_counter += 1;
        self.last_instruction_clock = self.clock;
        self.clock += 1;
        true
    }

    fn fetch_instruction(&self) -> Option<Instruction> {
        self.code.get(self.program_counter).cloned()
    }

    fn waiting_for_next_instruction(&self) -> bool {
        let instruction = self.fetch_instruction();

        if let Some(inst) = instruction {
            return self.clock < self.last_instruction_clock + inst.get_clock_timing();
        }
        false
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => {}
            Instruction::AddX(x) => self.registers.x += x,
        };
    }

    fn signal_strength(&self) -> i32 {
        self.clock as i32 * self.registers.x
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "C{:04} X{:03} I{:?}",
            self.clock,
            self.registers.x,
            self.fetch_instruction()
        )
    }
}

// CRT -------------------------------------------------------------------------

struct CRT {
    width: usize,
    height: usize,
    screen: Vec<u32>,
    position: usize,
}

impl CRT {
    fn new(width: usize, height: usize) -> Self {
        CRT {
            screen: vec![0; width * height],
            width,
            height,
            position: 0,
        }
    }

    fn tick(&mut self, registers: &Registers) {
        if self.must_draw_pixel(registers) {
            self.draw_pixel();
        }
        self.position += 1;
    }

    fn must_draw_pixel(&self, registers: &Registers) -> bool {
        ((self.position % self.width) as i32 == (registers.x - 1))
            || ((self.position % self.width) as i32 == (registers.x))
            || ((self.position % self.width) as i32 == (registers.x + 1))
    }

    fn draw_pixel(&mut self) {
        self.screen[self.position] = 1;
    }
}

impl fmt::Display for CRT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for pos in 0..(self.width * self.height) {
            if self.screen[pos] > 0 {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }

            if pos % self.width == self.width - 1 {
                writeln!(f)?;
            }
        }
        write!(f, "")
    }
}

// Device ----------------------------------------------------------------------

struct Device {
    crt: CRT,
    cpu: CPU,
}

impl Device {
    fn new(instructions: Vec<Instruction>, width: usize, height: usize) -> Self {
        Device {
            crt: CRT::new(width, height),
            cpu: CPU::new(instructions),
        }
    }

    fn tick(&mut self) -> bool {
        let result = self.cpu.tick();
        self.crt.tick(&self.cpu.registers);
        result
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "CPU: {:#}", self.cpu)?;
        write!(f, "Screen: {:?}\n{:#}", self.crt.position, self.crt)
    }
}

// Task ------------------------------------------------------------------------

pub fn part_one(input: &str) -> Option<i32> {
    let instructions = input
        .lines()
        .map(|x| Instruction::try_from(x).unwrap())
        .collect_vec();
    let mut device = Device::new(instructions, 40, 6);
    let mut tracker = 0;

    while device.tick() {
        if [20, 60, 100, 140, 180, 220].contains(&(device.cpu.clock as i32)) {
            tracker += device.cpu.signal_strength();
        }
    }
    Some(tracker)
}

pub fn part_two(input: &str) -> Option<i32> {
    let instructions = input
        .lines()
        .map(|x| Instruction::try_from(x).unwrap())
        .collect_vec();
    let mut device = Device::new(instructions, 40, 6);

    while device.tick() {}

    println!("{:#}", device);
    Some(device.crt.screen.iter().sum::<u32>() as i32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(124));
    }
}
