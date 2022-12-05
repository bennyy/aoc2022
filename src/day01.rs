use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::any::Any;
default_aoc_struct!(Day1, i32);
default_new_ctor!(Day1);

impl Day1 {
    fn get_elf_inventory(&self, input_str: String) -> Vec<Vec<i32>> {
        let elf_foods_strings: Vec<&str> = input_str.split("\n\n").collect();
        let mut elf_inventory: Vec<Vec<i32>> = Vec::new();

        for elf_foods_string in elf_foods_strings.iter() {
            let v: Vec<i32> = elf_foods_string
                .split('\n')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            elf_inventory.push(v);
        }
        elf_inventory
    }
}

impl AdventOfCode for Day1 {
    fn day_str(&self) -> String {
        "day01".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let elf_inventory: Vec<Vec<i32>> = self.get_elf_inventory(input_str);

        self.puzzle1_result = elf_inventory
            .iter()
            .map(|x| x.iter().sum::<i32>())
            .max()
            .unwrap();
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let elf_inventory: Vec<Vec<i32>> = self.get_elf_inventory(input_str);

        let mut total_elf_cal_inventory: Vec<i32> = elf_inventory
            .iter()
            .map(|x| x.iter().sum::<i32>())
            .collect();

        total_elf_cal_inventory.sort();

        self.puzzle2_result = total_elf_cal_inventory.iter().rev().take(3).sum();
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

    puzzle1_test!(Day1, 24000, 69836);
    puzzle2_test!(Day1, 45000, 207968);
}
