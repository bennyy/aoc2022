use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::any::Any;
use std::cmp;
use std::collections::vec_deque::Drain;
use std::collections::VecDeque;

default_aoc_struct!(Day5, String);
default_new_ctor!(Day5);

impl Day5 {
    fn get_container(input_str: String) -> Vec<VecDeque<char>> {
        let start_stack: Vec<Vec<char>> = input_str
            .lines()
            .map(|line| line.chars().skip(1).step_by(4).collect())
            .collect();

        let mut containers = vec![VecDeque::new(); start_stack.first().unwrap().len()];

        start_stack.iter().rev().skip(1).for_each(|layer| {
            layer.iter().enumerate().for_each(|(index, stack)| {
                if !stack.is_whitespace() && !stack.is_numeric() {
                    containers.get_mut(index).unwrap().push_front(*stack);
                }
            })
        });

        containers
    }

    fn get_instructions(input_str: String) -> Vec<Vec<usize>> {
        let instructions: Vec<Vec<usize>> = input_str
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .skip(1)
                    .step_by(2)
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        instructions
    }

    fn get_code(containers: Vec<VecDeque<char>>) -> String {
        let mut st: String = "".to_owned();
        for c in containers {
            st.push(*c.front().unwrap());
        }
        st
    }

    fn run_crate_mover(
        is9001: bool,
        instructions: &[Vec<usize>],
        containers: &mut [VecDeque<char>],
    ) {
        for instruction in instructions.iter() {
            let number_of_containers = *instruction.first().unwrap();
            let from = instruction.get(1).unwrap() - 1;
            let to = *instruction.get(2).unwrap() - 1;

            let container_size = containers.get_mut(from).unwrap().len();
            let length = cmp::min(number_of_containers, container_size);

            let crate_mover_setting = |i: Drain<char>| -> VecDeque<char> {
                if is9001 {
                    i.rev().collect()
                } else {
                    i.collect()
                }
            };
            let current_stack =
                crate_mover_setting(containers.get_mut(from).unwrap().drain(0..length));

            for c in current_stack {
                containers.get_mut(to).unwrap().push_front(c);
            }
        }
    }
}

impl AdventOfCode for Day5 {
    fn day_str(&self) -> String {
        "day05".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let mut inputs = input_str.split("\n\n");

        let mut containers = Day5::get_container(inputs.next().unwrap().to_string());
        let instructions = Day5::get_instructions(inputs.last().unwrap().to_string());

        Day5::run_crate_mover(false, &instructions, &mut containers);

        self.puzzle1_result = Day5::get_code(containers);
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let mut inputs = input_str.split("\n\n");

        let mut containers = Day5::get_container(inputs.next().unwrap().to_string());
        let instructions = Day5::get_instructions(inputs.last().unwrap().to_string());

        Day5::run_crate_mover(true, &instructions, &mut containers);

        self.puzzle2_result = Day5::get_code(containers);
    }

    fn get_puzzle1_result(&self) -> Option<Box<dyn Any>> {
        Some(Box::new(self.puzzle1_result.clone()))
    }

    fn get_puzzle2_result(&self) -> Option<Box<dyn Any>> {
        Some(Box::new(self.puzzle2_result.clone()))
    }
}

#[cfg(test)]
mod tests {
    use crate::file_util;

    use super::*;

    #[test]
    fn puzzle_1() {
        let mut day = Day5::new();

        let test_str: String =
            file_util::file_to_string(format!("inputs/{}_test.txt", day.day_str()));
        day.run_puzzle1(test_str);
        let actual_value = *day
            .get_puzzle1_result()
            .unwrap_or(Box::new(0))
            .downcast::<String>()
            .unwrap();
        assert_eq!("CMZ", actual_value);

        let main_str: String = file_util::file_to_string(format!("inputs/{}.txt", day.day_str()));
        day.run_puzzle1(main_str);
        let actual_value = *day
            .get_puzzle1_result()
            .unwrap_or(Box::new(0))
            .downcast::<String>()
            .unwrap();
        assert_eq!("TQRFCBSJJ", actual_value);
    }

    #[test]
    fn puzzle_2() {
        let mut day = Day5::new();

        let test_str: String =
            file_util::file_to_string(format!("inputs/{}_test.txt", day.day_str()));
        day.run_puzzle2(test_str);
        let actual_value = *day
            .get_puzzle2_result()
            .unwrap_or(Box::new(""))
            .downcast::<String>()
            .unwrap();
        assert_eq!("MCD", actual_value);

        let main_str: String = file_util::file_to_string(format!("inputs/{}.txt", day.day_str()));
        day.run_puzzle2(main_str);
        let actual_value = *day
            .get_puzzle2_result()
            .unwrap_or(Box::new(""))
            .downcast::<String>()
            .unwrap();
        assert_eq!("RMHFJNVFP", actual_value);
    }
}
