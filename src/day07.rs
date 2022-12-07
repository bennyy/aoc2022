use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::any::Any;
default_aoc_struct!(Day7, i32);
default_new_ctor!(Day7);

impl Day7 {}

impl AdventOfCode for Day7 {
    fn day_str(&self) -> String {
        "day07".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let cmd_history: Vec<Vec<&str>> = input_str
            .split('$')
            .map(|cmd| cmd.trim())
            .filter(|x| !x.is_empty())
            .map(|y| y.lines().collect::<Vec<&str>>())
            .collect();

        let mut stack: Vec<i32> = Vec::new();
        let mut dir_sizes: Vec<i32> = vec![0];
        self.puzzle1_result = 0;

        stack.push(0);

        for cmd in cmd_history.iter() {
            let mut cmd_iter = cmd.iter();
            let command_type = cmd_iter.next().unwrap();

            if command_type.starts_with("cd") {
                let folder = command_type.split_ascii_whitespace().last().unwrap();

                if folder.contains("..") {
                    // If we going back, we're done with this folder.

                    // Check what's the result from the folder
                    let dir_size = stack.pop().unwrap();
                    dir_sizes.push(dir_size);
                    if dir_size <= 100000 {
                        // If under 100 000, add it to the result
                        self.puzzle1_result += dir_size;
                    }
                    // Add the current dir size to it's parent.
                    *stack.last_mut().unwrap() += dir_size;
                } else {
                    // Going deeper, restart on a new count
                    stack.push(0);
                } //24933642
            } else if command_type.starts_with("ls") {
                for ca in cmd_iter {
                    if !ca.contains("dir") {
                        // Get the current dir size.
                        let current_dir_size = stack.last_mut().unwrap();
                        let file_size = ca
                            .split_ascii_whitespace()
                            .next()
                            .unwrap()
                            .parse::<i32>()
                            .unwrap();
                        // Add this file size to the directory size.
                        *current_dir_size += file_size;
                    }
                }
            } else {
                panic!("Unknown cmd!");
            }
        }

        // cd.. back to the root.
        while stack.len() > 1 {
            // Check what's the result from the folder
            let dir_size = stack.pop().unwrap();
            dir_sizes.push(dir_size);
            if dir_size <= 100000 {
                // If under 100 000, add it to the result
                self.puzzle1_result += dir_size;
            }
            // Add the current dir size to it's parent.
            *stack.last_mut().unwrap() += dir_size;
        }
        let mut v: Vec<i32> = Vec::new();
        let sum: i32 = stack.iter().sum();

        v.append(&mut dir_sizes);
        v.append(&mut stack);

        let amount_to_be_deleted = 30000000 - (70000000 - sum);
        let mut candidates: Vec<i32> = Vec::new();
        for s in v {
            if s >= amount_to_be_deleted {
                candidates.push(s);
            }
        }

        self.puzzle2_result = *candidates.iter().min().unwrap();
    }

    fn run_puzzle2(&mut self, input_str: String) {
        self.run_puzzle1(input_str);
    }

    fn get_puzzle1_result(&self) -> Option<Box<dyn Any>> {
        Some(Box::new(self.puzzle1_result))
    }

    fn get_puzzle2_result(&self) -> Option<Box<dyn Any>> {
        Some(Box::new(self.puzzle2_result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{puzzle1_test, puzzle2_test};

    puzzle1_test!(Day7, 95437, 1792222);
    puzzle2_test!(Day7, 24933642, 1112963);
}
