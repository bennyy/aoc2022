use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::any::Any;
default_aoc_struct!(Day8, i32);
default_new_ctor!(Day8);

impl Day8 {}

impl AdventOfCode for Day8 {
    fn day_str(&self) -> String {
        "day08".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let width = input_str.lines().next().unwrap().len();
        let height = input_str.lines().count();

        let forest: Vec<i32> = input_str
            .replace('\n', "")
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect();

        let mut sum = 0;
        for (i, tree) in forest.iter().enumerate() {
            let x = i % width; // % is the "modulo operator", the remainder of i / width;
            let y = i / width; // where "/" is an integer division

            let x_slice = &forest[y * width..y * width + width];
            let y_slice: Vec<&i32> = forest[x..].iter().step_by(height).collect();

            let left = &x_slice[0..x];
            let right = &x_slice[x + 1..width];
            let top = &y_slice[0..y];
            let bottom = &y_slice[y + 1..height];

            if tree > left.iter().max().unwrap_or(&-1)
                || tree > right.iter().max().unwrap_or(&-1)
                || tree > top.iter().max().unwrap_or(&&-1)
                || tree > bottom.iter().max().unwrap_or(&&-1)
            {
                sum += 1;
            }
        }
        self.puzzle1_result = sum;
    }

    fn run_puzzle2(&mut self, input_str: String) {}

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

    puzzle1_test!(Day8, 21, 1870);
    puzzle2_test!(Day8, 0, 0);
}
