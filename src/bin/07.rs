use std::{cell::RefCell, fmt, rc::Rc};

use itertools::Itertools;

#[derive(PartialEq, Debug)]
struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    children: Vec<Rc<RefCell<Directory>>>,
    files: Vec<File>,
}

impl Directory {
    fn get_size(&self) -> u32 {
        let file_size = self.files.iter().map(|f| f.size).sum::<u32>();
        let dir_size = self
            .children
            .iter()
            .map(|d| d.borrow().get_size())
            .sum::<u32>();
        file_size + dir_size
    }

    fn get_sum_of_sizes_of_atmost(&self, n: u32) -> u32 {
        let mut result = self
            .children
            .iter()
            .map(|d| d.borrow().get_sum_of_sizes_of_atmost(n))
            .sum::<u32>();

        if self.get_size() < n {
            result += self.get_size();
        }

        result
    }

    fn get_size_of_smallest_dir_with_size_atleast(&self, n: u32) -> Option<u32> {
        let children = self
            .children
            .iter()
            .filter_map(|d| d.borrow().get_size_of_smallest_dir_with_size_atleast(n))
            .collect_vec();

        if children.is_empty() {
            if self.get_size() > n {
                return Some(self.get_size());
            }
            return None;
        }

        let childrensum = children.into_iter().min();

        if childrensum.is_some() {
            return childrensum;
        }

        if self.get_size() > n {
            return Some(self.get_size());
        }
        None
    }
}

impl fmt::Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for file in &self.files {
            write!(f, "{},", file.name)?;
        }
        for dir in &self.children {
            write!(f, "{},", dir.borrow())?;
        }
        write!(f, "]")
    }
}

fn mutate_from_ls_output(dir: Rc<RefCell<Directory>>, output: &str) {
    for l in output.lines() {
        let (typesize, name) = l.split_once(' ').unwrap();
        if typesize == "dir" {
            let child = Rc::new(RefCell::new(Directory {
                name: name.to_string(),
                parent: None,
                children: vec![],
                files: vec![],
            }));
            dir.borrow_mut().children.push(Rc::clone(&child));
            {
                let mut mut_child = child.borrow_mut();
                mut_child.parent = Some(Rc::clone(&dir));
            }
        } else {
            dir.borrow_mut().files.push(File {
                name: name.to_string(),
                size: typesize.parse::<u32>().unwrap(),
            })
        }
    }
}

fn setup_directory_from_commands(input: &str) -> Rc<RefCell<Directory>> {
    let commands = input
        .split("$ ")
        .skip(1)
        .map(|block| block.split_once('\n').unwrap())
        .collect_vec();

    let root = Rc::new(RefCell::new(Directory {
        name: String::from("/"),
        parent: None,
        children: vec![],
        files: vec![],
    }));

    let mut current_dir = Rc::clone(&root);
    for (command, output) in commands {
        match command.get(0..2) {
            Some("ls") => {
                mutate_from_ls_output(Rc::clone(&current_dir), output);
            }
            Some("cd") => {
                let (_, path) = command.split_once(' ').unwrap();
                match path {
                    ".." => {
                        let current_clone = Rc::clone(&current_dir);
                        current_dir = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                    }
                    "/" => {
                        current_dir = Rc::clone(&root);
                    }
                    x => {
                        let current_clone = Rc::clone(&current_dir);
                        for child in &current_clone.borrow().children {
                            if child.borrow().name == x {
                                current_dir = Rc::clone(child);
                                break;
                            }
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    root
}

#[derive(PartialEq, Debug)]
struct File {
    name: String,
    size: u32,
}

pub fn part_one(_input: &str) -> Option<u32> {
    let dir = setup_directory_from_commands(_input);
    let result = Some(dir.borrow().get_sum_of_sizes_of_atmost(100000));
    result
}

pub fn part_two(_input: &str) -> Option<u32> {
    let dir = setup_directory_from_commands(_input);
    let size_to_free = dir.borrow().get_size() - 40000000;
    println!("Find: {:?}", size_to_free);
    let result = dir
        .borrow()
        .get_size_of_smallest_dir_with_size_atleast(size_to_free);
    result
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
