use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::any::Any;

use std::cmp::Ordering;
use std::collections::HashMap;

use std::collections::BinaryHeap;
use std::mem;

default_aoc_struct!(Day12, i32);
default_new_ctor!(Day12);

#[derive(Copy, Debug, Clone, Eq, PartialEq)]
struct Node {
    pos: usize,
    prio: u32,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .prio
            .cmp(&self.prio)
            .then_with(|| self.prio.cmp(&other.prio))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day12 {
    fn heuristic(start: (i32, i32), goal: (i32, i32)) -> i32 {
        let dx = (start.0 - goal.0).abs();
        let dy = (start.1 - goal.1).abs();
        dx + dy
    }

    fn a_star(
        risk_level_map: &[u32],
        width: usize,
        start_index: usize,
        end_index: usize,
        part2: bool,
    ) -> Vec<usize> {
        let start_x = (start_index % width) as i32;
        let start_y = (start_index / width) as i32;
        let start_pos = (start_x, start_y);

        let end_x = (end_index % width) as i32;
        let end_y = (end_index / width) as i32;
        let end_pos = (end_x, end_y);

        let mut prio_queue = BinaryHeap::new();
        let mut came_from: HashMap<usize, usize> = HashMap::new();
        let mut cost_so_far: HashMap<usize, u32> = HashMap::new();

        let start_node = Node {
            pos: start_index,
            prio: Day12::heuristic(start_pos, end_pos) as u32,
        };
        cost_so_far.insert(start_index, 0);
        prio_queue.push(start_node);

        while !prio_queue.is_empty() {
            let current_node = prio_queue.pop().unwrap();
            let current_index = current_node.pos;

            // Part 1: Find best path
            // Part 2: Find first best 'a' from the end
            let mut goal_found = false;
            if part2 {
                if *risk_level_map.get(current_index).unwrap() == 'a' as u32 {
                    goal_found = true
                }
            } else if current_index == end_index {
                goal_found = true
            }

            if goal_found {
                // Backtrack where we came from.
                let mut total_path: Vec<usize> = Vec::new();
                let mut path_index = current_index;
                total_path.push(path_index);
                while came_from.contains_key(&path_index) {
                    path_index = *came_from.get(&path_index).unwrap();
                    total_path.push(path_index);
                }
                return total_path;
            }

            // Find all neighbours.
            let directions: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
            let mut neighbors: Vec<usize> = Vec::new();
            for v in directions {
                let x = (current_index % width) as i32 + v.0;
                let y = (current_index / width) as i32 + v.1;

                if x >= width as i32
                    || x < 0
                    || y < 0
                    || y >= risk_level_map.len() as i32 / width as i32
                {
                    // Outside the limit..
                    continue;
                }
                let i = (x as u32) + (width as u32) * (y as u32);
                neighbors.push(i as usize);
            }

            let current_elevation = *risk_level_map.get(current_index).unwrap() as u32;

            for neighbor_index in neighbors.iter_mut() {
                let neighbor_elevation = *risk_level_map.get(*neighbor_index).unwrap() as u32;

                // We should not climb, so make that path very annoying.
                const HARD_TO_CLIMB: u32 = 999999999;
                let mut tentative_gscore = HARD_TO_CLIMB;
                let diff = current_elevation as i32 - neighbor_elevation as i32;
                if part2 {
                    // If Part 2, we starting from the end and climbing down.
                    // We can go only go one step down (elevation).. But we can "climb".
                    if diff == 1 || diff <= 0 {
                        tentative_gscore = 1;
                    }
                } else {
                    // We can only go upp one step (elevation),
                    // but no limit on how far we can go/fall in one step.
                    if diff == -1 || diff >= 0 {
                        tentative_gscore = 1;
                    }
                }

                let new_cost = *cost_so_far.get(&current_index).unwrap() + tentative_gscore;
                if !cost_so_far.contains_key(neighbor_index)
                    || new_cost < *cost_so_far.get(neighbor_index).unwrap()
                {
                    cost_so_far.insert(*neighbor_index, new_cost);

                    let nx = (*neighbor_index % width) as i32;
                    let ny = (*neighbor_index / width) as i32;

                    // For Part 2 use Dijkstra (Uniform Cost Search)
                    let mut prio = new_cost;
                    if !part2 {
                        // Use A* for Part 1, to make it go fast!
                        prio = new_cost + Day12::heuristic(end_pos, (nx, ny)) as u32;
                    }

                    // Not even worth to explore it if climbing is required..
                    if prio < HARD_TO_CLIMB {
                        prio_queue.push(Node {
                            pos: *neighbor_index,
                            prio,
                        });
                    }

                    came_from.insert(*neighbor_index, current_index);
                }
            }
        }
        panic!("No valid path!");
    }

    fn run(input_str: String, is_part2: bool) -> i32 {
        let width = input_str.lines().next().unwrap().len();
        let _height = input_str.lines().count();

        let mut forest: Vec<u32> = input_str
            .replace('\n', "")
            .chars()
            .map(|x| x as u32)
            .collect();
        let mut start = forest.iter().position(|&x| x == ('S' as u32)).unwrap();
        let mut end = forest.iter().position(|&x| x == ('E' as u32)).unwrap();

        *forest.get_mut(start).unwrap() = 'a' as u32;
        *forest.get_mut(end).unwrap() = 'z' as u32;

        if is_part2 {
            // In part 2, better to go from the top and down to reach the nearest 'a'
            mem::swap(&mut start, &mut end);
        }

        let total_path = Day12::a_star(&forest, width, start, end, is_part2);
        total_path.len() as i32 - 1
    }
}

impl AdventOfCode for Day12 {
    fn day_str(&self) -> String {
        "day12".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        self.puzzle1_result = Day12::run(input_str, false);
    }

    fn run_puzzle2(&mut self, input_str: String) {
        self.puzzle2_result = Day12::run(input_str, true);
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

    puzzle1_test!(Day12, 31, 423);
    puzzle2_test!(Day12, 29, 416);
}
