use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::{any::Any, collections::VecDeque};
default_aoc_struct!(Day10, i32);
default_new_ctor!(Day10);

impl Day10 {
    fn cpu(code: VecDeque<(&str, i32)>, cycles: usize) -> i32 {
        let mut copy_code: VecDeque<(&str, i32)> = code.clone();
        let mut register = 1;
        let mut exec_stack: VecDeque<i32> = VecDeque::new();

        for _ in 0..cycles - 1 {
            register += exec_stack.pop_front().unwrap_or(0);

            if !code.is_empty() {
                let op = copy_code.pop_front().unwrap();
                match op.0 {
                    "noop" => {}
                    "addx" => {
                        exec_stack.push_back(op.1);
                        copy_code.push_front(("noop", 0));
                    }
                    _ => panic!("Unknown operation!"),
                }
            } else {
                println!("Program done");
                break;
            }
        }
        register
    }
}

impl AdventOfCode for Day10 {
    fn day_str(&self) -> String {
        "day10".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let code: VecDeque<(&str, i32)> = input_str
            .lines()
            .map(|line| {
                let mut l = line.split_ascii_whitespace();
                let operation = l.next().unwrap();
                let value = l.next().unwrap_or("0").parse::<i32>().unwrap();

                (operation, value)
            })
            .collect();

        let mut total_signal_strength: i32 = 0;
        let cycles: [i32; 6] = [20, 60, 100, 140, 180, 220];
        for cycle in cycles {
            let register_value = Day10::cpu(code.clone(), cycle as usize);
            let signal_strength = cycle * register_value;
            total_signal_strength += signal_strength;
        }
        self.puzzle1_result = total_signal_strength;
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

    puzzle1_test!(Day10, 13140, 13680);
    puzzle2_test!(Day10, 0, 0);
}
