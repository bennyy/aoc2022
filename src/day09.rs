use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::{any::Any, collections::HashSet};
default_aoc_struct!(Day9, i32);
default_new_ctor!(Day9);

impl Day9 {
    fn is_close_to_head(head: (i32, i32), tail: (i32, i32)) -> bool {
        return (head.0 - tail.0).abs() < 2 && (head.1 - tail.1).abs() < 2
    }

    fn run_rope(input_str: String, ropes: i32) -> i32 {
        let no_ropes: usize = ropes as usize;
        let mut ropes = vec![(0, 0); no_ropes];
        let mut trail: Vec<(i32, i32)> = Vec::new();
        trail.push(*ropes.last().unwrap());

        let directions: Vec<(char, i32)> = input_str
            .lines()
            .map(|line| {
                let mut l = line.split_ascii_whitespace();
                let direction = l.next().unwrap().parse::<char>().unwrap();
                let steps = l.next().unwrap().parse::<i32>().unwrap();

                (direction, steps)
            })
            .collect();

        for (direction, steps) in directions {
            for _ in 0..steps {
                let head = ropes.first_mut().unwrap();
                match direction {
                    'L' => head.0 -= 1,
                    'R' => head.0 += 1,
                    'U' => head.1 += 1,
                    'D' => head.1 -= 1,
                    _ => panic!("Unknown direction"),
                }

                for k in 1..no_ropes {
                    let head_x = ropes.get(k - 1).unwrap().0;
                    let head_y = ropes.get(k - 1).unwrap().1;
                    let tail_x = ropes.get(k).unwrap().0;
                    let tail_y = ropes.get(k).unwrap().1;
                    if !Day9::is_close_to_head((head_x, head_y), (tail_x, tail_y)) {
                        ropes[k].0 += (head_x - tail_x).signum();
                        ropes[k].1 += (head_y - tail_y).signum();
                    }
                }

                trail.push(*ropes.last().unwrap());
            }
        }
         trail.into_iter().collect::<HashSet<_>>().into_iter().len() as i32
    }
}

impl AdventOfCode for Day9 {
    fn day_str(&self) -> String {
        "day09".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        self.puzzle1_result = Day9::run_rope(input_str, 2);
    }

    fn run_puzzle2(&mut self, input_str: String) {
        self.puzzle2_result = Day9::run_rope(input_str, 10);
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

    puzzle1_test!(Day9, 13, 6018);
    puzzle2_test!(Day9, 1, 2619);
}
