use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};

use std::{
    any::Any,
    cmp,
    collections::{HashSet, VecDeque},
};
default_aoc_struct!(Day19, i32);
default_new_ctor!(Day19);

#[derive(Debug)]
struct Blueprint {
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
                    .replace(':', " ")
                    .split_ascii_whitespace()
                    .flat_map(|f| f.parse::<i32>().ok())
                    .collect();

                let mut number = numbers.iter();
                number.next();
                Blueprint {
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

    fn run(times: i32, blueprints: &[Blueprint]) -> Vec<i32> {
        const ORE: usize = 0;
        const CLAY: usize = 1;
        const OBSIDIAN: usize = 2;
        const GEODE: usize = 3;

        let mut return_list = Vec::new();
        for blueprint in blueprints.iter() {
            let max_ore_cost = *[
                blueprint.ore_robot_cost_ore,
                blueprint.clay_robot_cost_ore,
                blueprint.obsidian_robot_cost_ore,
                blueprint.geode_robot_cost_ore,
            ]
            .iter()
            .max()
            .unwrap();

            let start_robots = [1, 0, 0, 0];
            let start_resources = [0; 4];
            let mut queue: VecDeque<(i32, [i32; 4], [i32; 4])> = VecDeque::new();
            let mut has_visited: HashSet<(i32, [i32; 4], [i32; 4])> = HashSet::new();
            let mut best_geodes = 0;

            queue.push_front((0, start_robots, start_resources));

            while !queue.is_empty() {
                let tuple = queue.pop_front().unwrap();
                let minute = tuple.0;
                let robots = tuple.1;
                let mut resources = tuple.2;

                if minute >= times {
                    best_geodes = cmp::max(best_geodes, *resources.get_mut(GEODE).unwrap());
                    continue;
                }

                if has_visited.contains(&tuple) {
                    continue;
                }
                has_visited.insert(tuple);

                let ore = *resources.get(ORE).unwrap();
                let clay = *resources.get(CLAY).unwrap();
                let obsidian = *resources.get(OBSIDIAN).unwrap();
                *resources.get_mut(ORE).unwrap() += robots.get(ORE).unwrap();
                *resources.get_mut(CLAY).unwrap() += robots.get(CLAY).unwrap();
                *resources.get_mut(OBSIDIAN).unwrap() += robots.get(OBSIDIAN).unwrap();
                *resources.get_mut(GEODE).unwrap() += robots.get(GEODE).unwrap();
                let ore_robots = *robots.get(ORE).unwrap();
                let clay_robots = *robots.get(CLAY).unwrap();
                let osidian_robots = *robots.get(OBSIDIAN).unwrap();

                // Check if we can build a geode robot
                if ore >= blueprint.geode_robot_cost_ore
                    && obsidian >= blueprint.geode_robot_cost_obsidian
                {
                    let mut robots = robots;
                    let mut resources = resources;

                    *robots.get_mut(GEODE).unwrap() += 1;
                    *resources.get_mut(ORE).unwrap() -= blueprint.geode_robot_cost_ore;
                    *resources.get_mut(OBSIDIAN).unwrap() -= blueprint.geode_robot_cost_obsidian;

                    queue.push_front((minute + 1, robots, resources));
                    continue;
                }

                // Check if we can build a obisidan robot
                if osidian_robots < blueprint.geode_robot_cost_obsidian
                    && ore >= blueprint.obsidian_robot_cost_ore
                    && clay >= blueprint.obsidian_robot_cost_clay
                {
                    let mut robots = robots;
                    let mut resources = resources;

                    *robots.get_mut(OBSIDIAN).unwrap() += 1;
                    *resources.get_mut(ORE).unwrap() -= blueprint.obsidian_robot_cost_ore;
                    *resources.get_mut(CLAY).unwrap() -= blueprint.obsidian_robot_cost_clay;

                    queue.push_front((minute + 1, robots, resources));
                }

                // Check if we can build a ore robot
                if ore_robots < max_ore_cost && ore >= blueprint.ore_robot_cost_ore {
                    let mut robots = robots;
                    let mut resources = resources;

                    *robots.get_mut(ORE).unwrap() += 1;
                    *resources.get_mut(ORE).unwrap() -= blueprint.ore_robot_cost_ore;

                    queue.push_front((minute + 1, robots, resources));
                }

                // Check if we can build a clay robot
                if clay_robots < blueprint.obsidian_robot_cost_clay
                    && ore >= blueprint.clay_robot_cost_ore
                {
                    let mut robots = robots;
                    let mut resources = resources;

                    *robots.get_mut(CLAY).unwrap() += 1;
                    *resources.get_mut(ORE).unwrap() -= blueprint.clay_robot_cost_ore;

                    queue.push_front((minute + 1, robots, resources));
                }

                queue.push_front((minute + 1, robots, resources));
            }

            return_list.push(best_geodes);
        }
        return_list
    }
}

impl AdventOfCode for Day19 {
    fn day_str(&self) -> String {
        "day19".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let blueprints = Day19::parse_input(&input_str);
        let best_geodes = Day19::run(24, &blueprints);
        let result = best_geodes
            .iter()
            .enumerate()
            .map(|(i, &g)| (i as i32 + 1) * g)
            .sum();
        self.puzzle1_result = result;
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

    puzzle1_test!(Day19, 33, 1294);
    puzzle2_test!(Day19, 0, 0);
}
