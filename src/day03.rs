use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::any::Any;
use std::collections::HashSet;

default_aoc_struct!(Day3, i32);
default_new_ctor!(Day3);

impl Day3 {
    fn calculate_prio(c: char) -> u32 {
        const START_UPPER: u32 = ('A' as u32) - 1;
        const START_LOWER: u32 = ('a' as u32) - 1;

        let ascii_value = c as u32;
        if c.is_lowercase() {
            ascii_value - START_LOWER
        } else {
            ascii_value - START_UPPER + 26
        }
    }
}

impl AdventOfCode for Day3 {
    fn day_str(&self) -> String {
        "day03".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        self.puzzle1_result = 0;

        let rucksacks = input_str.split('\n');
        for rucksack in rucksacks {
            let (first, second) = rucksack.split_at(rucksack.len() / 2);

            let difference: Vec<char> = first
                .chars()
                .filter(|item| second.chars().any(|x| &x == item)) // Find only unique elements
                .collect::<HashSet<_>>() // Clear out all duplicates
                .into_iter()
                .collect();

            let common_item = *difference.first().unwrap();
            self.puzzle1_result += Day3::calculate_prio(common_item) as i32;
        }
    }

    fn run_puzzle2(&mut self, input_str: String) {
        self.puzzle2_result = 0;

        let all_rucksacks: Vec<&str> = input_str.split('\n').collect();
        for mut group_rucksacks in all_rucksacks.chunks(3).map(|x| x.iter().copied()) {
            let first = group_rucksacks.next().unwrap();
            let second = group_rucksacks.next().unwrap();
            let third = group_rucksacks.next().unwrap();

            let difference: Vec<char> = first
                .chars()
                .filter(|item| second.chars().any(|x| &x == item)) // Find only unique elements
                .filter(|item| third.chars().any(|x| &x == item)) // Find only unique elements
                .collect::<HashSet<_>>() // Clear out all duplicates
                .into_iter()
                .collect();

            let common_item = *difference.first().unwrap();
            self.puzzle2_result += Day3::calculate_prio(common_item) as i32;
        }
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

    puzzle1_test!(Day3, 157, 7821);
    puzzle2_test!(Day3, 70, 2752);

    #[test]
    fn ascii_prio_test() {
        let mut expected_prio = 0;
        for i in 97..123 {
            // a --> z
            expected_prio += 1;
            let c = char::from_u32(i).unwrap();
            assert_eq!(expected_prio, Day3::calculate_prio(c))
        }

        for i in 65..91 {
            // A --> Z
            expected_prio += 1;
            let c = char::from_u32(i).unwrap();
            assert_eq!(expected_prio, Day3::calculate_prio(c))
        }
    }
}
