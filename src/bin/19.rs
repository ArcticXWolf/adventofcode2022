use std::collections::HashMap;

use itertools::Itertools;

enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, PartialEq)]
struct Costs {
    ore_robot: (u32, u32, u32),
    clay_robot: (u32, u32, u32),
    obsidian_robot: (u32, u32, u32),
    geode_robot: (u32, u32, u32),
    highest_ore_cost: u32,
    highest_clay_cost: u32,
    highest_obsidian_cost: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    amount_robots: (u32, u32, u32, u32),
    currency: (u32, u32, u32, u32),
}

impl State {
    fn generate(&self) -> Self {
        Self {
            amount_robots: (
                self.amount_robots.0,
                self.amount_robots.1,
                self.amount_robots.2,
                self.amount_robots.3,
            ),
            currency: (
                self.currency.0 + self.amount_robots.0,
                self.currency.1 + self.amount_robots.1,
                self.currency.2 + self.amount_robots.2,
                self.currency.3 + self.amount_robots.3,
            ),
        }
    }

    fn can_buy(&self, robot_type: &RobotType, costs: &Costs) -> bool {
        match robot_type {
            RobotType::Ore => {
                self.currency.0 >= costs.ore_robot.0
                    && self.currency.1 >= costs.ore_robot.1
                    && self.currency.2 >= costs.ore_robot.2
            }
            RobotType::Clay => {
                self.currency.0 >= costs.clay_robot.0
                    && self.currency.1 >= costs.clay_robot.1
                    && self.currency.2 >= costs.clay_robot.2
            }
            RobotType::Obsidian => {
                self.currency.0 >= costs.obsidian_robot.0
                    && self.currency.1 >= costs.obsidian_robot.1
                    && self.currency.2 >= costs.obsidian_robot.2
            }
            RobotType::Geode => {
                self.currency.0 >= costs.geode_robot.0
                    && self.currency.1 >= costs.geode_robot.1
                    && self.currency.2 >= costs.geode_robot.2
            }
        }
    }

    fn buy(&self, robot_type: &RobotType, costs: &Costs) -> Self {
        let (cost, oi, ci, obi, gi) = match robot_type {
            RobotType::Ore => (costs.ore_robot, 1, 0, 0, 0),
            RobotType::Clay => (costs.clay_robot, 0, 1, 0, 0),
            RobotType::Obsidian => (costs.obsidian_robot, 0, 0, 1, 0),
            RobotType::Geode => (costs.geode_robot, 0, 0, 0, 1),
        };
        Self {
            amount_robots: (
                self.amount_robots.0 + oi,
                self.amount_robots.1 + ci,
                self.amount_robots.2 + obi,
                self.amount_robots.3 + gi,
            ),
            currency: (
                self.currency.0 - cost.0,
                self.currency.1 - cost.1,
                self.currency.2 - cost.2,
                self.currency.3,
            ),
        }
    }
}

fn parse_input(input: &str) -> Vec<Costs> {
    let re = regex::Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    input
        .lines()
        .map(|l| {
            let captures = re.captures(l).unwrap();
            Costs {
                ore_robot: (captures[2].parse::<u32>().unwrap(), 0, 0),
                clay_robot: (captures[3].parse::<u32>().unwrap(), 0, 0),
                obsidian_robot: (
                    captures[4].parse::<u32>().unwrap(),
                    captures[5].parse::<u32>().unwrap(),
                    0,
                ),
                geode_robot: (
                    captures[6].parse::<u32>().unwrap(),
                    0,
                    captures[7].parse::<u32>().unwrap(),
                ),
                highest_ore_cost: captures[2]
                    .parse::<u32>()
                    .unwrap()
                    .max(captures[3].parse::<u32>().unwrap())
                    .max(captures[4].parse::<u32>().unwrap())
                    .max(captures[6].parse::<u32>().unwrap()),
                highest_clay_cost: captures[5].parse::<u32>().unwrap(),
                highest_obsidian_cost: captures[7].parse::<u32>().unwrap(),
            }
        })
        .collect_vec()
}

fn find_optimum_return_recursivly(
    time_left: u32,
    costs: &Costs,
    state: &State,
    cache: &mut HashMap<(u32, State), u32>,
) -> u32 {
    if time_left == 0 {
        return state.currency.3;
    }

    if let Some(result) = cache.get(&(time_left, state.clone())) {
        return *result;
    }

    let generated_state = state.generate();
    let mut max_geodes = 0;

    if state.can_buy(&RobotType::Geode, costs) {
        let new_state = generated_state.buy(&RobotType::Geode, costs);
        max_geodes = max_geodes.max(find_optimum_return_recursivly(
            time_left - 1,
            costs,
            &new_state,
            cache,
        ));
    }

    if state.amount_robots.2 < costs.highest_obsidian_cost
        && state.can_buy(&RobotType::Obsidian, costs)
    {
        let new_state = generated_state.buy(&RobotType::Obsidian, costs);
        max_geodes = max_geodes.max(find_optimum_return_recursivly(
            time_left - 1,
            costs,
            &new_state,
            cache,
        ));
    }

    if state.amount_robots.1 < costs.highest_clay_cost && state.can_buy(&RobotType::Clay, costs) {
        let new_state = generated_state.buy(&RobotType::Clay, costs);
        max_geodes = max_geodes.max(find_optimum_return_recursivly(
            time_left - 1,
            costs,
            &new_state,
            cache,
        ));
    }

    if state.amount_robots.0 < costs.highest_ore_cost && state.can_buy(&RobotType::Ore, costs) {
        let new_state = generated_state.buy(&RobotType::Ore, costs);
        max_geodes = max_geodes.max(find_optimum_return_recursivly(
            time_left - 1,
            costs,
            &new_state,
            cache,
        ));
    }

    // buy nothing
    max_geodes = max_geodes.max(find_optimum_return_recursivly(
        time_left - 1,
        costs,
        &generated_state,
        cache,
    ));

    cache.insert((time_left, state.clone()), max_geodes);

    max_geodes
}

pub fn part_one(_input: &str) -> Option<u32> {
    let blueprints = parse_input(_input);
    println!("{:?}", blueprints);
    let mut tracker = 0;
    for (i, b) in blueprints.iter().enumerate() {
        let result = find_optimum_return_recursivly(
            24,
            b,
            &State {
                amount_robots: (1, 0, 0, 0),
                currency: (0, 0, 0, 0),
            },
            &mut HashMap::new(),
        );
        //let result = find_optimum_return2(24, &b);
        println!("Got blueprint {:02} with {}", i, result);
        tracker += result * (i as u32 + 1);
    }
    Some(tracker)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let blueprints = parse_input(_input.lines().take(3).join("\n").as_str());
    println!("{:?}", blueprints);
    let mut tracker = 1;
    for (i, b) in blueprints.iter().enumerate() {
        let result = find_optimum_return_recursivly(
            32,
            b,
            &State {
                amount_robots: (1, 0, 0, 0),
                currency: (0, 0, 0, 0),
            },
            &mut HashMap::new(),
        );
        //let result = find_optimum_return2(24, &b);
        println!("Got blueprint {:02} with {}", i, result);
        tracker *= result;
    }
    Some(tracker)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(56 * 62));
    }
}
