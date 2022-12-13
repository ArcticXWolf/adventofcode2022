use std::cmp::Ordering;

use itertools::Itertools;

// Implement Packet with Ordering for easy comparisons and sorting
#[derive(Debug, Clone, Eq)]
enum PacketNode {
    Number(u32),
    List(Vec<PacketNode>),
}

impl Ord for PacketNode {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketNode::Number(x), PacketNode::Number(y)) => x.cmp(y),
            (PacketNode::List(x), PacketNode::List(y)) => {
                for (px, py) in x.iter().zip(y.iter()) {
                    let result = px.cmp(py);
                    if result != Ordering::Equal {
                        return result;
                    }
                }

                x.len().cmp(&y.len())
            }
            (PacketNode::List(_), PacketNode::Number(_)) => {
                let created_list = PacketNode::List(vec![other.clone()]);
                self.cmp(&created_list)
            }
            (PacketNode::Number(_), PacketNode::List(_)) => {
                let created_list = PacketNode::List(vec![self.clone()]);
                created_list.cmp(other)
            }
        }
    }
}

impl PartialOrd for PacketNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PacketNode {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

// Parsing the input recursivly
// Urgh, this could be done more elegantly
impl From<&str> for PacketNode {
    fn from(input: &str) -> Self {
        if !input.starts_with('[') {
            return PacketNode::Number(input.parse::<u32>().unwrap());
        }

        let mut subnodes = vec![];
        let mut index = 1;
        while index < input.len() - 1 {
            let node: PacketNode = match input.get(index..index + 1) {
                Some("[") => {
                    let start = index;
                    let mut listcount: u32 = 1;
                    index += 1;
                    while listcount > 0 {
                        match input.get(index..index + 1) {
                            Some("[") => listcount += 1,
                            Some("]") => listcount -= 1,
                            _ => {}
                        }
                        index += 1;
                    }
                    let subpacketstr = input.get(start..index).unwrap();
                    PacketNode::from(subpacketstr)
                }
                Some(_) => {
                    let start = index;
                    while index < input.len() - 1 && input.get(index..index + 1).unwrap() != "," {
                        index += 1;
                    }
                    let subpacketstr = input.get(start..index).unwrap();
                    PacketNode::from(subpacketstr)
                }
                None => unreachable!(),
            };
            subnodes.push(node);
            index += 1;
        }

        PacketNode::List(subnodes)
    }
}

// From here it is just gathering the input and filtering for the required solution
pub fn part_one(_input: &str) -> Option<u32> {
    let pairs = _input.split("\n\n").map(|p| {
        p.lines()
            .map(PacketNode::from)
            .collect_tuple::<(PacketNode, PacketNode)>()
            .unwrap()
    });

    Some(
        pairs
            .enumerate()
            .filter_map(|(i, (x, y))| {
                if x.cmp(&y) == Ordering::Less {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .sum::<usize>() as u32,
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    let mut nodes = _input
        .replace("\n\n", "\n")
        .split_whitespace()
        .map(PacketNode::from)
        .collect_vec();

    let divider1 = PacketNode::List(vec![PacketNode::List(vec![PacketNode::Number(2)])]);
    let divider2 = PacketNode::List(vec![PacketNode::List(vec![PacketNode::Number(6)])]);
    nodes.push(divider1.clone());
    nodes.push(divider2.clone());

    nodes.sort();

    let index1 = nodes.iter().find_position(|x| **x == divider1).unwrap().0 + 1;
    let index2 = nodes.iter().find_position(|x| **x == divider2).unwrap().0 + 1;

    Some((index1 * index2) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
