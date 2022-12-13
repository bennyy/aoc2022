use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::any::Any;
default_aoc_struct!(Day12, i32);
default_new_ctor!(Day12);


impl Day12 {
}

impl AdventOfCode for Day12 {
    fn day_str(&self) -> String {
        "day12".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
    }

    fn run_puzzle2(&mut self, input_str: String) {
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

    puzzle1_test!(Day12, 31, 0);
    puzzle2_test!(Day12, 0, 0);
}
