use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::any::Any;
default_aoc_struct!(Day22, i32);
default_new_ctor!(Day22);

impl Day22 {}

impl AdventOfCode for Day22 {
    fn day_str(&self) -> String {
        "day22".to_owned()
    }

    fn run_puzzle1(&mut self, _input_str: String) {}

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

    puzzle1_test!(Day22, 0 /* 6032 */, 0);
    puzzle2_test!(Day22, 0, 0);
}
