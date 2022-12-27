use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};

use std::{any::Any, collections::HashMap};
default_aoc_struct!(Day21, i64);
default_new_ctor!(Day21);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Mult,
    Div,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    Num(i64),
    Equation((String, Operator, String)),
}

impl Day21 {
    fn eval(input: &HashMap<String, Packet>, packet: &Packet) -> i64 {
        match packet {
            Packet::Num(number) => *number,
            Packet::Equation((left, op, right)) => {
                let lhs = input.get(left).unwrap();
                let rhs = input.get(right).unwrap();

                let lhsn = Day21::eval(input, lhs);
                let rhsn = Day21::eval(input, rhs);

                match op {
                    Operator::Plus => lhsn + rhsn,
                    Operator::Minus => lhsn - rhsn,
                    Operator::Mult => lhsn * rhsn,
                    Operator::Div => lhsn / rhsn,
                }
            }
        }
    }
}

impl AdventOfCode for Day21 {
    fn day_str(&self) -> String {
        "day21".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let input: HashMap<String, Packet> = input_str
            .lines()
            .map(|line| {
                let mut split = line.split(':');
                let key = split.next().unwrap().trim().to_string();
                let packet = split.next().unwrap().trim();

                let splitted_packet: Vec<&str> = packet.split_ascii_whitespace().collect();
                if splitted_packet.len() == 1 {
                    let number = splitted_packet.first().unwrap().parse::<i64>().unwrap();
                    (key, Packet::Num(number))
                } else {
                    let lhs = splitted_packet.first().unwrap().to_string();
                    let op = splitted_packet.get(1).unwrap().chars().next().unwrap();
                    let rhs = splitted_packet.get(2).unwrap().to_string();

                    match op {
                        '+' => (key, Packet::Equation((lhs, Operator::Plus, rhs))),
                        '-' => (key, Packet::Equation((lhs, Operator::Minus, rhs))),
                        '/' => (key, Packet::Equation((lhs, Operator::Div, rhs))),
                        '*' => (key, Packet::Equation((lhs, Operator::Mult, rhs))),
                        _ => panic!("Unknown operator"),
                    }
                }
            })
            .collect();

        let root = input.get(&"root".to_string()).unwrap();
        self.puzzle1_result = Day21::eval(&input, root);
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
    use crate::file_util;

    #[test]
    fn puzzle_1() {
        let mut day = Day21::new();

        let test_str: String =
            file_util::file_to_string(format!("inputs/{}_test.txt", day.day_str()));
        day.run_puzzle1(test_str);
        let actual_value = *day
            .get_puzzle1_result()
            .unwrap_or_else(|| Box::new(""))
            .downcast::<i64>()
            .unwrap();
        assert_eq!(152, actual_value);

        let main_str: String = file_util::file_to_string(format!("inputs/{}.txt", day.day_str()));
        day.run_puzzle1(main_str);
        let actual_value = *day
            .get_puzzle1_result()
            .unwrap_or_else(|| Box::new(""))
            .downcast::<i64>()
            .unwrap();
        assert_eq!(142707821472432, actual_value);
    }

    #[test]
    fn puzzle_2() {
        let mut day = Day21::new();

        let test_str: String =
            file_util::file_to_string(format!("inputs/{}_test.txt", day.day_str()));
        day.run_puzzle2(test_str);
        let actual_value = *day
            .get_puzzle2_result()
            .unwrap_or_else(|| Box::new(""))
            .downcast::<i64>()
            .unwrap();
        assert_eq!(0, actual_value);

        let main_str: String = file_util::file_to_string(format!("inputs/{}.txt", day.day_str()));
        day.run_puzzle2(main_str);
        let actual_value = *day
            .get_puzzle2_result()
            .unwrap_or_else(|| Box::new(""))
            .downcast::<i64>()
            .unwrap();

        assert_eq!(0, actual_value);
    }
}
