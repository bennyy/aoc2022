use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::{
    any::Any,
    collections::{HashMap, VecDeque},
};
default_aoc_struct!(Day11, u64);
default_new_ctor!(Day11);

#[derive(Debug)]
struct Monkey {
    starting_items: VecDeque<u64>,

    operation_left: String,
    operation: char,
    operation_right: String,

    divisible_number: u64,
    if_true: u64,
    if_false: u64,
}

impl Day11 {
    fn parse_only_integers(str: &str) -> u64 {
        str.chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    }

    fn parse_input(str: &str) -> Vec<Monkey> {
        str.split("\n\n")
            .map(|monkey| {
                let mut monkey_split = monkey.lines().skip(1);

                /* Starting items */
                let starting_items: VecDeque<u64> = monkey_split
                    .next()
                    .unwrap()
                    .split(':')
                    .last()
                    .unwrap()
                    .split(',')
                    .map(|x| x.trim().parse::<u64>().unwrap())
                    .collect();

                let operation: Vec<&str> = monkey_split
                    .next()
                    .unwrap()
                    .split('=')
                    .last()
                    .unwrap()
                    .trim()
                    .split_ascii_whitespace()
                    .collect();
                let operation_left = operation.first().unwrap().to_string();
                let operation_char = operation.get(1).unwrap().chars().next().unwrap();
                let operation_right = operation.get(2).unwrap().to_string();

                let divisible_by: u64 = Day11::parse_only_integers(monkey_split.next().unwrap());
                let if_true: u64 = Day11::parse_only_integers(monkey_split.next().unwrap());
                let if_false: u64 = Day11::parse_only_integers(monkey_split.next().unwrap());

                Monkey {
                    starting_items,
                    operation_left,
                    operation: operation_char,
                    operation_right,
                    divisible_number: divisible_by,
                    if_true,
                    if_false,
                }
            })
            .collect()
    }

    fn monkey_calc(left: &str, op: char, right: &str, old: u64) -> u64 {
        let mut l = old;
        let mut r = old;
        if !left.contains("old") {
            l = left.parse::<u64>().unwrap();
        }
        if !right.contains("old") {
            r = right.parse::<u64>().unwrap();
        }

        match op {
            '+' => l + r,
            '*' => l * r,
            _ => panic!("Unknown operator"),
        }
    }

    fn run_monkey_business(monkeys: &mut [Monkey], steps: usize, part2: bool) -> u64 {
        // Create a temporay monkey item stack
        let mut monkey_stack: HashMap<u64, VecDeque<u64>> = HashMap::new();
        let mut monkey_count: Vec<u64> = Vec::new();
        for (i, _) in monkeys.iter().enumerate() {
            monkey_stack.insert(i as u64, VecDeque::new());
            monkey_count.push(0);
        }

        let modfactor: u64 = monkeys.iter().map(|m| m.divisible_number as u64).product();

        for _ in 0..steps {
            for (i, monkey) in monkeys.iter_mut().enumerate() {
                // Add the stored temporay items into the starting items.
                let temp_monkey_stack = monkey_stack.get_mut(&(i as u64)).unwrap();
                monkey.starting_items.append(temp_monkey_stack);

                // Add the items to the monkey count
                *monkey_count.get_mut(i).unwrap() += monkey.starting_items.len() as u64;

                // Run monkey business stuff until the items are empty
                while !monkey.starting_items.is_empty() {
                    let item = monkey.starting_items.pop_front().unwrap();
                    let mut worry_level = Day11::monkey_calc(
                        &monkey.operation_left,
                        monkey.operation,
                        &monkey.operation_right,
                        item,
                    );

                    if !part2 {
                        worry_level /= 3;
                    } else {
                        worry_level %= modfactor;
                    }

                    if worry_level % monkey.divisible_number as u64 == 0 {
                        monkey_stack
                            .get_mut(&monkey.if_true)
                            .unwrap()
                            .push_back(worry_level);
                    } else {
                        monkey_stack
                            .get_mut(&monkey.if_false)
                            .unwrap()
                            .push_back(worry_level);
                    }
                }
            }
        }

        // Sort the worst monkeys and take the two "worst".
        monkey_count.sort();
        let worst_monkeys: u64 = monkey_count.iter().rev().take(2).product();
        worst_monkeys
    }
}

impl AdventOfCode for Day11 {
    fn day_str(&self) -> String {
        "day11".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let mut monkeys: Vec<Monkey> = Day11::parse_input(&input_str);
        self.puzzle1_result = Day11::run_monkey_business(&mut monkeys, 20, false);
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let mut monkeys: Vec<Monkey> = Day11::parse_input(&input_str);
        self.puzzle2_result = Day11::run_monkey_business(&mut monkeys, 10000, true);
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
    use crate::file_util;

    #[test]
    fn puzzle_1() {
        let mut day = Day11::new();

        let test_str: String =
            file_util::file_to_string(format!("inputs/{}_test.txt", day.day_str()));
        day.run_puzzle1(test_str);
        let actual_value = *day
            .get_puzzle1_result()
            .unwrap_or_else(|| Box::new(""))
            .downcast::<u64>()
            .unwrap();
        assert_eq!(10605, actual_value);

        let main_str: String = file_util::file_to_string(format!("inputs/{}.txt", day.day_str()));
        day.run_puzzle1(main_str);
        let actual_value = *day
            .get_puzzle1_result()
            .unwrap_or_else(|| Box::new(""))
            .downcast::<u64>()
            .unwrap();
        assert_eq!(88208, actual_value);
    }

    #[test]
    fn puzzle_2() {
        let mut day = Day11::new();

        let test_str: String =
            file_util::file_to_string(format!("inputs/{}_test.txt", day.day_str()));
        day.run_puzzle2(test_str);
        let actual_value = *day
            .get_puzzle2_result()
            .unwrap_or_else(|| Box::new(""))
            .downcast::<u64>()
            .unwrap();
        assert_eq!(2713310158, actual_value);

        let main_str: String = file_util::file_to_string(format!("inputs/{}.txt", day.day_str()));
        day.run_puzzle2(main_str);
        let actual_value = *day
            .get_puzzle2_result()
            .unwrap_or_else(|| Box::new(""))
            .downcast::<u64>()
            .unwrap();

        assert_eq!(21115867968, actual_value);
    }
}
