use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::{any::Any, collections::HashSet};
default_aoc_struct!(Day9, i32);
default_new_ctor!(Day9);

impl Day9 {
    fn step(direction: (i32, i32), steps: i32, head: &(i32, i32)) -> Vec<(i32, i32)> {
        let mut vec = Vec::new();
        for i in 1..steps + 1 {
            let dx = direction.0;
            let dy = direction.1;

            let x = head.0;
            let y = head.1;

            vec.push((x + (dx * i), y + (dy * i)));
        }

        vec
    }

    fn is_close_to_head(head: (i32, i32), tail: (i32, i32)) -> bool {
        let distance = f64::sqrt(((head.0 - tail.0).pow(2) + (head.1 - tail.1).pow(2)).into());
        if distance < 2.0 {
            return true;
        }
        false
    }
}

impl AdventOfCode for Day9 {
    fn day_str(&self) -> String {
        "day09".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let directions: Vec<(char, i32)> = input_str
            .lines()
            .map(|line| {
                let mut l = line.split_ascii_whitespace();
                let direction = l.next().unwrap().parse::<char>().unwrap();
                let steps = l.next().unwrap().parse::<i32>().unwrap();

                (direction, steps)
            })
            .collect();

        let mut head = (0, 0);
        let mut head_trail: Vec<(i32, i32)> = Vec::new();

        let mut tail = (0, 0);
        let mut tail_trail: Vec<(i32, i32)> = Vec::new();
        tail_trail.push((0, 0));

        for (direction, steps) in directions {
            match direction {
                'L' => {
                    head_trail.extend(Day9::step((-1, 0), steps, &head));
                }
                'R' => {
                    head_trail.extend(Day9::step((1, 0), steps, &head));
                }
                'U' => {
                    head_trail.extend(Day9::step((0, 1), steps, &head));
                }
                'D' => {
                    head_trail.extend(Day9::step((0, -1), steps, &head));
                }
                _ => panic!("Unknown direction: {}", direction),
            }

            head = *head_trail.last().unwrap();
        }

        for (i, head) in head_trail.iter().enumerate() {
            let close = Day9::is_close_to_head(*head, tail);
            if !close {
                let prev = head_trail.get(i - 1).unwrap();
                tail = *prev;
                tail_trail.push(*prev);
            }
        }

        let trail_length = tail_trail
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .len();

        self.puzzle1_result = trail_length as i32;
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

    puzzle1_test!(Day9, 13, 6018);
    puzzle2_test!(Day9, 0, 0);
}
