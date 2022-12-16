use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Node {
    name: String,
    id: usize,
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

    for (i, line) in input.lines().enumerate() {
        let re_match = re.captures(line.trim()).unwrap();
        result.push(Node {
            name: re_match[1].to_string(),
            id: i,
            flow: re_match[2].parse().unwrap(),
            neighbor_strings: re_match[3].split(", ").map(|x| x.to_string()).collect(),
            neighbors: vec![],
        });
        if &re_match[1] == "AA" {
            start_id = i;
        }
    }

    for i in 0..result.len() {
        for j in 0..result[i].neighbor_strings.len() {
            let neighbor_name = &result[i].neighbor_strings[j];
            let neighbor_node = result.iter().find(|n| n.name == *neighbor_name).unwrap().id;
            result[i].neighbors.push(neighbor_node);
        }
    }

    (result, start_id)
}

fn print_dot_format(graph: &[Node]) {
    let mut closed = HashSet::new();
    println!("graph valves {{");
    for n in graph.iter() {
        println!("{} [label=\"{} - {}\"]", n.name, n.name, n.flow);
    }

    for n in graph.iter() {
        closed.insert(n.name.clone());
        for nn in &n.neighbor_strings {
            if closed.contains(nn) {
                continue;
            }
            println!("  {} -- {};", n.name, nn);
        }
    }
    println!("}}");
}

fn get_all_paths(graph: &[Node]) -> HashMap<(usize, usize), u32> {
    let mut result = HashMap::new();
    for node in graph.iter() {
        let solutions = bfs(graph, node.id);
        for (target, distance) in solutions {
            result.insert((node.id, target), distance);
        }
    }
    result
}

fn bfs(graph: &[Node], start: usize) -> HashMap<usize, u32> {
    let mut closed = HashSet::new();
    let mut solution = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((current_id, current_distance)) = queue.pop_front() {
        let current_node = graph.iter().find(|n| n.id == current_id).unwrap();

        for neighbor_id in current_node.neighbors.iter() {
            if closed.contains(&neighbor_id) {
                continue;
            }

            queue.push_back((*neighbor_id, current_distance + 1));
            closed.insert(neighbor_id);

            let neighbor = graph.iter().find(|n| n.id == *neighbor_id).unwrap();
            if neighbor.flow > 0 {
                solution.insert(*neighbor_id, current_distance + 1);
            }
        }
    }
    solution
}

fn find_maximum_pressure_path(
    graph: &[Node],
    precomputed_paths: &HashMap<(usize, usize), u32>,
    current_node_id: usize,
    current_time: u32,
    current_flow: u32,
    closed: &[usize],
) -> (Vec<usize>, u32) {
    if current_time >= 30 {
        return (vec![], current_flow);
    }

    let nodes_possible = precomputed_paths
        .keys()
        .filter(|(s, t)| *s == current_node_id && !closed.contains(t));

    let mut best_path = vec![];
    let mut best_flow = current_flow;
    for (_, target) in nodes_possible {
        let time_to_move_to_target_and_activate =
            precomputed_paths[&(current_node_id, *target)] + 1;
        if current_time + time_to_move_to_target_and_activate >= 30 {
            continue;
        }

        let target_flow = graph.iter().find(|n| n.id == *target).unwrap().flow
            * (30 - current_time - time_to_move_to_target_and_activate);
        let mut new_closed = closed.iter().cloned().collect_vec();
        new_closed.push(*target);

        let (path, flow) = find_maximum_pressure_path(
            graph,
            precomputed_paths,
            *target,
            current_time + time_to_move_to_target_and_activate,
            current_flow + target_flow,
            &new_closed,
        );

        if flow > best_flow {
            best_path = path;
            best_flow = flow;
        }
    }

    best_path.push(current_node_id);
    (best_path, best_flow)
}

fn find_maximum_pressure_path_with_elephant(
    graph: &[Node],
    precomputed_paths: &HashMap<(usize, usize), u32>,
    current_node_ids: (usize, usize),
    current_times: (u32, u32),
    current_flow: u32,
    closed: &[usize],
) -> (Vec<(usize, usize)>, u32) {
    if current_times.0 >= 26 && current_times.1 >= 26 {
        return (vec![], current_flow);
    }

    let (current_node_id, current_time, is_me) = if current_times.0 < current_times.1 {
        (current_node_ids.0, current_times.0, true)
    } else {
        (current_node_ids.1, current_times.1, false)
    };

    let nodes_possible = precomputed_paths
        .keys()
        .filter(|(s, t)| *s == current_node_id && !closed.contains(t));

    let mut best_path: Vec<(usize, usize)> = vec![];
    let mut best_flow = current_flow;

    for (_, target) in nodes_possible {
        let time_to_move_to_target_and_activate =
            precomputed_paths[&(current_node_id, *target)] + 1;

        if current_time + time_to_move_to_target_and_activate >= 26 {
            continue;
        }

        let target_flow = graph.iter().find(|n| n.id == *target).unwrap().flow
            * (26 - current_time - time_to_move_to_target_and_activate);
        let (next_ids, next_times) = if is_me {
            (
                (*target, current_node_ids.1),
                (
                    current_time + time_to_move_to_target_and_activate,
                    current_times.1,
                ),
            )
        } else {
            (
                (current_node_ids.0, *target),
                (
                    current_times.0,
                    current_time + time_to_move_to_target_and_activate,
                ),
            )
        };
        let mut new_closed = closed.iter().cloned().collect_vec();
        new_closed.push(*target);

        let (path, flow) = find_maximum_pressure_path_with_elephant(
            graph,
            precomputed_paths,
            next_ids,
            next_times,
            current_flow + target_flow,
            &new_closed,
        );

        if flow > best_flow {
            best_path = path;
            best_flow = flow;
        }
    }

    best_path.push(current_node_ids);
    (best_path, best_flow)
}

pub fn part_one(_input: &str) -> Option<u32> {
    let (graph, start) = parse_input_into_nodes(_input);
    let paths = get_all_paths(&graph);

    let (mut solution, flow) = find_maximum_pressure_path(&graph, &paths, start, 0, 0, &[]);
    solution.reverse();

    // print_dot_format(&graph);
    // println!("{:?}", solution);

    Some(flow)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let (graph, start) = parse_input_into_nodes(_input);
    let paths = get_all_paths(&graph);

    let (mut solution, flow) =
        find_maximum_pressure_path_with_elephant(&graph, &paths, (start, start), (0, 0), 0, &[]);
    solution.reverse();

    // print_dot_format(&graph);
    println!("{:?}", solution);

    Some(flow)
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
