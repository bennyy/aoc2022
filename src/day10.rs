use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::{any::Any, collections::VecDeque};
default_aoc_struct!(Day10, i32);
default_new_ctor!(Day10);

impl Day10 {
    fn parse_input(input_str: String) -> VecDeque<(String, i32)> {
        input_str
            .lines()
            .map(|line| {
                let mut l = line.split_ascii_whitespace();
                let operation = l.next().unwrap().to_string();
                let value = l.next().unwrap_or("0").parse::<i32>().unwrap();

                (operation, value)
            })
            .collect()
    }

    fn cpu(code: VecDeque<(String, i32)>, cycles: usize, crt_video: &mut String) -> i32 {
        let mut copy_code: VecDeque<(String, i32)> = code.clone();
        let mut register = 1;
        let mut exec_stack: VecDeque<i32> = VecDeque::new();

        let mut crt: [char; 240] = [' '; 240];

        for (cycle, crt_char) in crt.iter_mut().enumerate().take(cycles - 1) {
            let crt_pixel = ((cycle as i32) % 40) + 1;
            if crt_pixel >= register && crt_pixel < register + 3 {
                *crt_char = '█';
            }

            register += exec_stack.pop_front().unwrap_or(0);

            if !code.is_empty() {
                let op = copy_code.pop_front().unwrap();
                match op.0.as_str() {
                    "noop" => {}
                    "addx" => {
                        exec_stack.push_back(op.1);
                        copy_code.push_front(("noop".to_owned(), 0));
                    }
                    _ => panic!("Unknown operation!"),
                }
            } else {
                println!("Program done");
                break;
            }
        }

        crt_video.clear();
        crt.chunks(40).for_each(|line| {
            let s: String = line.iter().collect();
            crt_video.push_str(&s);
            crt_video.push('\n');
        });
        *crt_video = crt_video.trim_end().to_string();

        register
    }
}

impl AdventOfCode for Day10 {
    fn day_str(&self) -> String {
        "day10".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let code = Day10::parse_input(input_str);

        let mut crt_video: String = Default::default();
        let mut total_signal_strength: i32 = 0;
        let cycles: [i32; 6] = [20, 60, 100, 140, 180, 220];
        for cycle in cycles {
            let register_value = Day10::cpu(code.clone(), cycle as usize, &mut crt_video);
            let signal_strength = cycle * register_value;
            total_signal_strength += signal_strength;
        }
        self.puzzle1_result = total_signal_strength;
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let code = Day10::parse_input(input_str);

        let mut crt_video: String = Default::default();
        self.puzzle2_result = 240 * Day10::cpu(code, 240, &mut crt_video);

        println!("{}", crt_video);
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

    puzzle1_test!(Day10, 13140, 13680);
    puzzle2_test!(Day10, 4080, 8640);
}
