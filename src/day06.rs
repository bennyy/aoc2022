use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::{any::Any, collections::HashSet};
default_aoc_struct!(Day6, i32);
default_new_ctor!(Day6);

impl Day6 {
    fn find_index(input_chars: &[char], size: usize) -> i32 {
        input_chars
            .windows(size)
            .enumerate()
            .find_map(|(i, x)| {
                if x.iter().collect::<HashSet<_>>().len() == size {
                    return Some(i + size);
                }
                None
            })
            .unwrap_or(0) as i32
    }
}

impl AdventOfCode for Day6 {
    fn day_str(&self) -> String {
        "day06".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let chars: Vec<char> = input_str.chars().collect::<Vec<char>>();
        self.puzzle1_result = Day6::find_index(&chars, 4);
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let chars: Vec<char> = input_str.chars().collect();
        self.puzzle2_result = Day6::find_index(&chars, 14);
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

    puzzle1_test!(Day6, 7, 1920);
    puzzle2_test!(Day6, 19, 2334);
}
