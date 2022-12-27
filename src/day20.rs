use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::{any::Any, collections::VecDeque};
default_aoc_struct!(Day20, i64);
default_new_ctor!(Day20);

impl Day20 {
    fn decrypt(input_str: &str, decryption_key: i64, times: usize) -> i64 {
        let mut input: Vec<(usize, i64)> = input_str
            .lines()
            .map(|x| x.parse::<i64>().unwrap() * decryption_key)
            .enumerate()
            .collect();
        let og_input = input.clone();
        let length = input.len() as i64;
        for _ in 0..times {
            let mut queue: VecDeque<(usize, i64)> = VecDeque::from(og_input.clone());
            while !queue.is_empty() {
                let tuple = queue.pop_front().unwrap();
                let index = tuple.0;
                let number = tuple.1;

                let index_of_number = input.iter().position(|&r| r.0 == index).unwrap();
                let no_index = index_of_number as i64;

                let mut new_index = (no_index + number) % (length - 1);
                if new_index < 0 {
                    new_index = length + new_index - 1;
                }

                input.retain(|&x| x != tuple);
                input.insert(new_index as usize, tuple);
            }
        }

        let index_of_zero_number = input.iter().position(|&r| r.1 == 0).unwrap();
        let a: usize = (index_of_zero_number + 1000) % input.len();
        let b: usize = (index_of_zero_number + 2000) % input.len();
        let c: usize = (index_of_zero_number + 3000) % input.len();
        input.get(a).unwrap().1 + input.get(b).unwrap().1 + input.get(c).unwrap().1
    }
}

impl AdventOfCode for Day20 {
    fn day_str(&self) -> String {
        "day20".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        self.puzzle1_result = Day20::decrypt(&input_str, 1, 1);
    }

    fn run_puzzle2(&mut self, input_str: String) {
        self.puzzle2_result = Day20::decrypt(&input_str, 811589153, 10);
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
        let mut day = Day20::new();

        let test_str: String =
            file_util::file_to_string(format!("inputs/{}_test.txt", day.day_str()));
        day.run_puzzle1(test_str);
        let actual_value = *day
            .get_puzzle1_result()
            .unwrap_or_else(|| Box::new(""))
            .downcast::<i64>()
            .unwrap();
        assert_eq!(3, actual_value);

        let main_str: String = file_util::file_to_string(format!("inputs/{}.txt", day.day_str()));
        day.run_puzzle1(main_str);
        let actual_value = *day
            .get_puzzle1_result()
            .unwrap_or_else(|| Box::new(""))
            .downcast::<i64>()
            .unwrap();
        assert_eq!(15297, actual_value);
    }

    #[test]
    fn puzzle_2() {
        let mut day = Day20::new();

        let test_str: String =
            file_util::file_to_string(format!("inputs/{}_test.txt", day.day_str()));
        day.run_puzzle2(test_str);
        let actual_value = *day
            .get_puzzle2_result()
            .unwrap_or_else(|| Box::new(""))
            .downcast::<i64>()
            .unwrap();
        assert_eq!(1623178306, actual_value);

        let main_str: String = file_util::file_to_string(format!("inputs/{}.txt", day.day_str()));
        day.run_puzzle2(main_str);
        let actual_value = *day
            .get_puzzle2_result()
            .unwrap_or_else(|| Box::new(""))
            .downcast::<i64>()
            .unwrap();

        assert_eq!(2897373276210, actual_value);
    }
}
