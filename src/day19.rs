use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use core::num;
use std::any::Any;
default_aoc_struct!(Day19, i32);
default_new_ctor!(Day19);

#[derive(Debug)]
struct Blueprint {
    no: i32,
    ore_robot_cost_ore: i32,
    clay_robot_cost_ore: i32,
    obsidian_robot_cost_ore: i32,
    obsidian_robot_cost_clay: i32,
    geode_robot_cost_ore: i32,
    geode_robot_cost_obsidian: i32,
}

impl Day19 {
    fn parse_input(input_str: &str) -> Vec<Blueprint> {
        input_str
            .lines()
            .map(|line| {
                let numbers: Vec<i32> = line
                    .replace(":", " ")
                    .split_ascii_whitespace()
                    .flat_map(|f| f.parse::<i32>().ok())
                    .collect();

                let mut number = numbers.iter();
                Blueprint {
                    no: *number.next().unwrap(),
                    ore_robot_cost_ore: *number.next().unwrap(),
                    clay_robot_cost_ore: *number.next().unwrap(),
                    obsidian_robot_cost_ore: *number.next().unwrap(),
                    obsidian_robot_cost_clay: *number.next().unwrap(),
                    geode_robot_cost_ore: *number.next().unwrap(),
                    geode_robot_cost_obsidian: *number.next().unwrap(),
                }
            })
            .collect()
    }
}

impl AdventOfCode for Day19 {
    fn day_str(&self) -> String {
        "day19".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let blueprints = Day19::parse_input(&input_str);
        println!("{:#?}", blueprints);
    }

    fn run_puzzle2(&mut self, _input_str: String) {}

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

    puzzle1_test!(Day19, 33, 0);
    puzzle2_test!(Day19, 0, 0);
}
