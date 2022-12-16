use itertools::Itertools;
use ndarray::Array3;
use regex::Regex;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Node {
    name: String,
    flow: u32,
    neighbor_strings: Vec<String>,
    neighbors: Vec<usize>,
}

fn parse_input_into_nodes(input: &str) -> (Vec<Node>, usize) {
    let re =
        Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.+)$")
            .unwrap();
    let mut result = vec![];
    let mut start_id = 0;

    for line in input.lines() {
        let re_match = re.captures(line.trim()).unwrap();
        result.push(Node {
            name: re_match[1].to_string(),
            flow: re_match[2].parse().unwrap(),
            neighbor_strings: re_match[3].split(", ").map(|x| x.to_string()).collect(),
            neighbors: vec![],
        });
    }

    result.sort_by(|a, b| b.flow.cmp(&a.flow));

    for i in 0..result.len() {
        if result[i].name == "AA" {
            start_id = i;
        }
        for j in 0..result[i].neighbor_strings.len() {
            let neighbor_name = &result[i].neighbor_strings[j];
            let neighbor_node = result
                .iter()
                .find_position(|n| n.name == *neighbor_name)
                .unwrap()
                .0;
            result[i].neighbors.push(neighbor_node);
        }
    }

    (result, start_id)
}

// Dynamic programming approach used from
// https://www.reddit.com/r/adventofcode/comments/zn6k1l/2022_day_16_solutions/
// My initial original solution can be found in the git history, but part 2 took
// 2min runtime (because of brute force depth search) and thus was too long for
// my complete AOC runtime. So if you are interested in my initial solution
// scroll back in commit history.
fn find_maximum_pressure_path_dp(
    graph: &[Node],
    max_time_left: u32,
) -> ndarray::ArrayBase<ndarray::OwnedRepr<u32>, ndarray::Dim<[usize; 3]>> {
    let amount_of_relevant_valves = graph.iter().filter(|n| n.flow > 0).count();
    let amount_of_relevant_valve_configurations: u32 = 1 << amount_of_relevant_valves;

    // ndarrays are so much faster than HashMap/Array, etc
    let mut solution = Array3::<u32>::zeros((
        max_time_left as usize,
        graph.len(),
        amount_of_relevant_valve_configurations as usize,
    ));

    // Iterate over all possible states
    // States are: amount of time left, position in graph, opened valves
    // Valve configuration is saved as bitmask with 0 = open and 1 = closed
    // remember we move backwards in time (start from last timeslot), so we can
    // only refer to future states
    for time_left in 1..max_time_left {
        for current_node in 0..graph.len() {
            let current_node_valve_bit = 1 << current_node;
            for valve_configuration in 0..amount_of_relevant_valve_configurations {
                // Calculate result from surrounding states (future states!)
                let mut result = 0;

                if current_node_valve_bit & valve_configuration != 0 && time_left > 1 {
                    // Check valve opening transition
                    // -> we are at valve closed, so the step later is open
                    // -> take step later result and add flow of our valve until end of time
                    result = solution[(
                        (time_left - 1) as usize,
                        current_node,
                        (valve_configuration - current_node_valve_bit) as usize,
                    )] + graph[current_node].flow * time_left;
                }

                // result now might contain our best flow if we open the valve
                // at current position now check if it is better to not open and
                // move to another position (same configuration)
                for neighbor in graph[current_node].neighbors.iter() {
                    result = result.max(
                        solution[(
                            (time_left - 1) as usize,
                            *neighbor,
                            valve_configuration as usize,
                        )],
                    )
                }

                // save
                solution[(
                    time_left as usize,
                    current_node,
                    valve_configuration as usize,
                )] = result;
            }
        }
    }

    solution
}

pub fn part_one(_input: &str) -> Option<u32> {
    let (graph, start) = parse_input_into_nodes(_input);
    let amount_of_relevant_valves = graph.iter().filter(|n| n.flow > 0).count();
    let amount_of_relevant_valve_configurations = 1 << amount_of_relevant_valves;

    let solution = find_maximum_pressure_path_dp(&graph, 30);

    Some(solution[(29, start, amount_of_relevant_valve_configurations - 1)])
}

pub fn part_two(_input: &str) -> Option<u32> {
    let (graph, start) = parse_input_into_nodes(_input);
    let amount_of_relevant_valves = graph.iter().filter(|n| n.flow > 0).count();
    let amount_of_relevant_valve_configurations = 1 << amount_of_relevant_valves;
    let mut tracker = 0;

    let solution = find_maximum_pressure_path_dp(&graph, 30);

    // Give me and the elephant different valves to work with.  Those
    // assignments must be disjunct. Also it is okay to only check total
    // assignments (no valves not assigned) because the dynamic programming
    // table accounts for not using valves
    for me in 0..amount_of_relevant_valve_configurations {
        for elephant in 0..amount_of_relevant_valve_configurations {
            if me & elephant != 0 {
                continue;
            }
            let flow = solution[(25, start, me)] + solution[(25, start, elephant)];
            tracker = tracker.max(flow);
        }
    }

    Some(tracker)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
