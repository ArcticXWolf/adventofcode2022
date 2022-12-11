use itertools::Itertools;

// Yeah, yeah, yeah, I'm dumb.  I only realized after programming this whole
// thing that for part2 I dont need to keep track of all remainders for all
// possibile divisors, but since the division tests are made with prime numbers
// I could just multiply them together to get a better modular arithmatic which
// works for all of them at once.

// Welp, since this is how I submitted my result, I keep the bad code for now.
// Maybe later I refactor it..

#[derive(Debug, Clone)]
struct Item {
    starting_value: u32,
    current_value: u32,
    moduli: Vec<(u32, u32)>,
}

impl Item {
    fn init(&mut self, mod_list: &Vec<u32>) {
        self.current_value = self.starting_value;
        for modulos in mod_list {
            self.moduli.push((*modulos, self.starting_value % modulos));
        }
    }

    fn calculate(&mut self, op: &Operation, part2: bool) {
        if !part2 {
            let value = self.current_value;
            let mut new_value = match op {
                Operation::Square => value * value,
                Operation::Add(x) => value + x,
                Operation::Multipy(x) => value * x,
            };

            new_value /= 3;

            self.current_value = new_value;
            return;
        }

        for i in 0..self.moduli.len() {
            let (modulos, value) = self.moduli[i];
            let new_value = match op {
                Operation::Square => value * value,
                Operation::Add(x) => value + x,
                Operation::Multipy(x) => value * x,
            };

            self.moduli[i] = (modulos, new_value % modulos);
        }
    }

    fn test(&self, test_parameter: u32, part2: bool) -> bool {
        if !part2 {
            self.current_value % test_parameter == 0
        } else {
            for mod_val in self.moduli.iter() {
                if mod_val.0 == test_parameter {
                    return mod_val.1 == 0;
                }
            }
            unreachable!();
        }
    }
}

#[derive(Debug)]
enum Operation {
    Square,
    Add(u32),
    Multipy(u32),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test_parameter: u32,
    next_monkey_id_on_test_true: u32,
    next_monkey_id_on_test_false: u32,
}

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        let lines = value.lines().collect_vec();

        let items = lines[1]
            .trim()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .map(|x| Item {
                starting_value: x.parse::<u32>().unwrap(),
                current_value: 0,
                moduli: vec![],
            })
            .collect_vec();
        let mut operation_line = lines[2].split_whitespace();
        let operation = match (
            operation_line.nth_back(0).unwrap(),
            operation_line.nth_back(0).unwrap(),
        ) {
            ("old", "*") => Operation::Square,
            (x, "+") => Operation::Add(x.parse::<u32>().unwrap()),
            (x, "*") => Operation::Multipy(x.parse::<u32>().unwrap()),
            _ => unreachable!(),
        };
        let test_parameter = lines[3]
            .trim()
            .strip_prefix("Test: divisible by ")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let next_monkey_id_on_test_true = lines[4]
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let next_monkey_id_on_test_false = lines[5]
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse::<u32>()
            .unwrap();

        Monkey {
            items,
            operation,
            test_parameter,
            next_monkey_id_on_test_true,
            next_monkey_id_on_test_false,
        }
    }
}

impl Monkey {
    fn turn(&mut self, part2: bool) -> Vec<(u32, Item)> {
        let mut thrown_items = vec![];

        for item in self.items.iter() {
            let mut new_item = item.clone();
            new_item.calculate(&self.operation, part2);

            let next_monkey = match new_item.test(self.test_parameter, part2) {
                true => self.next_monkey_id_on_test_true,
                false => self.next_monkey_id_on_test_false,
            };

            // println!(
            //     "Throw item {} with value {} to monkey {} ({:?})",
            //     item.current_value, new_item.current_value, next_monkey, new_item
            // );
            thrown_items.push((next_monkey, new_item));
        }

        self.items.clear();

        thrown_items
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(Monkey::from).collect_vec()
}

fn init_items(monkeys: &mut Vec<Monkey>) {
    let monkey_mod_list = monkeys.iter().map(|m| m.test_parameter).collect_vec();
    for m in monkeys {
        for i in 0..m.items.len() {
            m.items[i].init(&monkey_mod_list);
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut monkeys = parse_input(input);
    let mut monkey_activity: Vec<u32> = vec![0; monkeys.len()];

    init_items(&mut monkeys);

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let thrown_items = monkeys[i].turn(false);
            monkey_activity[i] += thrown_items.len() as u32;

            for item in thrown_items {
                monkeys[item.0 as usize].items.push(item.1);
            }
        }
        // println!(
        //     "Round {}: {:?}",
        //     _round,
        //     monkeys
        //         .iter()
        //         .map(|m| m.items.iter().map(|i| i.current_value).collect_vec())
        //         .collect_vec()
        // );
    }

    Some(
        monkey_activity
            .iter()
            .sorted()
            .rev()
            .take(2)
            .product::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys = parse_input(input);
    let mut monkey_activity: Vec<u32> = vec![0; monkeys.len()];

    init_items(&mut monkeys);

    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            let thrown_items = monkeys[i].turn(true);
            monkey_activity[i] += thrown_items.len() as u32;

            for item in thrown_items {
                monkeys[item.0 as usize].items.push(item.1);
            }
        }
        // println!(
        //     "Round {}: {:?}",
        //     _round,
        //     monkeys
        //         .iter()
        //         .map(|m| m.items.iter().map(|i| i.current_value).collect_vec())
        //         .collect_vec()
        // );
        // println!("RoundActivity: {:?}", monkey_activity);
    }

    Some(
        monkey_activity
            .iter()
            .sorted()
            .rev()
            .take(2)
            .map(|x| *x as u64)
            .product::<u64>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
