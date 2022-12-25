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

    fn can_buy_when(&self, robot_type: &RobotType, costs: &Costs) -> Option<u32> {
        match robot_type {
            RobotType::Ore => {
                if self.amount_robots.0 == 0 || self.amount_robots.0 >= costs.highest_ore_cost {
                    return None;
                }
                Some(
                    (0.max(costs.ore_robot.0 as i32 - self.currency.0 as i32) as u32
                        + self.amount_robots.0
                        - 1)
                        / self.amount_robots.0,
                )
            }
            RobotType::Clay => {
                if self.amount_robots.0 == 0 || self.amount_robots.1 >= costs.highest_clay_cost {
                    return None;
                }
                Some(
                    (0.max(costs.clay_robot.0 as i32 - self.currency.0 as i32) as u32
                        + self.amount_robots.0
                        - 1)
                        / self.amount_robots.0,
                )
            }
            RobotType::Obsidian => {
                if self.amount_robots.0 == 0
                    || self.amount_robots.1 == 0
                    || self.amount_robots.2 >= costs.highest_obsidian_cost
                {
                    return None;
                }
                let time_ore = (0.max(costs.obsidian_robot.0 as i32 - self.currency.0 as i32)
                    as u32
                    + self.amount_robots.0
                    - 1)
                    / self.amount_robots.0;
                let time_clay = (0.max(costs.obsidian_robot.1 as i32 - self.currency.1 as i32)
                    as u32
                    + self.amount_robots.1
                    - 1)
                    / self.amount_robots.1;
                Some(time_ore.max(time_clay))
            }
            RobotType::Geode => {
                if self.amount_robots.0 == 0 || self.amount_robots.2 == 0 {
                    return None;
                }
                let time_ore = (0.max(costs.geode_robot.0 as i32 - self.currency.0 as i32) as u32
                    + self.amount_robots.0
                    - 1)
                    / self.amount_robots.0;
                let time_obsidian = (0.max(costs.geode_robot.2 as i32 - self.currency.2 as i32)
                    as u32
                    + self.amount_robots.2
                    - 1)
                    / self.amount_robots.2;
                Some(time_ore.max(time_obsidian))
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

    let mut max_geodes = 0;

    if let Some(time) = state.can_buy_when(&RobotType::Geode, costs) {
        if time < time_left {
            let mut new_state = state.generate();
            for _ in 0..time {
                new_state = new_state.generate();
            }
            new_state = new_state.buy(&RobotType::Geode, costs);

            max_geodes = max_geodes.max(find_optimum_return_recursivly(
                time_left - time - 1,
                costs,
                &new_state,
                cache,
            ));
        }
    }

    if let Some(time) = state.can_buy_when(&RobotType::Obsidian, costs) {
        if time < time_left {
            let mut new_state = state.generate();
            for _ in 0..time {
                new_state = new_state.generate();
            }
            new_state = new_state.buy(&RobotType::Obsidian, costs);

            max_geodes = max_geodes.max(find_optimum_return_recursivly(
                time_left - time - 1,
                costs,
                &new_state,
                cache,
            ));
        }
    }

    if let Some(time) = state.can_buy_when(&RobotType::Clay, costs) {
        if time < time_left {
            let mut new_state = state.generate();
            for _ in 0..time {
                new_state = new_state.generate();
            }
            new_state = new_state.buy(&RobotType::Clay, costs);

            max_geodes = max_geodes.max(find_optimum_return_recursivly(
                time_left - time - 1,
                costs,
                &new_state,
                cache,
            ));
        }
    }

    if let Some(time) = state.can_buy_when(&RobotType::Ore, costs) {
        if time < time_left {
            let mut new_state = state.generate();
            for _ in 0..time {
                new_state = new_state.generate();
            }

            new_state = new_state.buy(&RobotType::Ore, costs);

            max_geodes = max_geodes.max(find_optimum_return_recursivly(
                time_left - time - 1,
                costs,
                &new_state,
                cache,
            ));
        }
    }

    // buy nothing until end of time
    let mut new_state = state.generate();
    for _ in 0..(time_left - 1) {
        new_state = new_state.generate();
    }
    max_geodes = max_geodes.max(find_optimum_return_recursivly(0, costs, &new_state, cache));

    cache.insert((time_left, state.clone()), max_geodes);

    max_geodes
}

pub fn part_one(_input: &str) -> Option<u32> {
    let blueprints = parse_input(_input);
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
        tracker += result * (i as u32 + 1);
    }
    Some(tracker)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let blueprints = parse_input(_input.lines().take(3).join("\n").as_str());
    let mut tracker = 1;
    for (_, b) in blueprints.iter().enumerate() {
        let result = find_optimum_return_recursivly(
            32,
            b,
            &State {
                amount_robots: (1, 0, 0, 0),
                currency: (0, 0, 0, 0),
            },
            &mut HashMap::new(),
        );
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

    #[ignore]
    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(56 * 62));
    }
}
