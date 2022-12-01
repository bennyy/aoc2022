use crate::aoc::AdventOfCode;

use crate::day01::Day1;
use crate::day02::Day2;
use crate::day03::Day3;
use crate::day04::Day4;
use crate::day05::Day5;
use crate::day06::Day6;
use crate::day07::Day7;
use crate::day08::Day8;
use crate::day09::Day9;
use crate::day10::Day10;
use crate::day11::Day11;
use crate::day12::Day12;
use crate::day13::Day13;
use crate::day14::Day14;
use crate::day15::Day15;
use crate::day16::Day16;
use crate::day17::Day17;
use crate::day18::Day18;
use crate::day19::Day19;
use crate::day20::Day20;
use crate::day21::Day21;
use crate::day22::Day22;
use crate::day23::Day23;
use crate::day24::Day24;
use crate::day25::Day25;
use std::time::Instant;

mod aoc;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod file_util;

fn run_all_puzzles() {
    let array: [Box<dyn AdventOfCode>; 25] = [
        Box::new(Day1::new()),
        Box::new(Day2::new()),
        Box::new(Day3::new()),
        Box::new(Day4::new()),
        Box::new(Day5::new()),
        Box::new(Day6::new()),
        Box::new(Day7::new()),
        Box::new(Day8::new()),
        Box::new(Day9::new()),
        Box::new(Day10::new()),
        Box::new(Day11::new()),
        Box::new(Day12::new()),
        Box::new(Day13::new()),
        Box::new(Day14::new()),
        Box::new(Day15::new()),
        Box::new(Day16::new()),
        Box::new(Day17::new()),
        Box::new(Day18::new()),
        Box::new(Day19::new()),
        Box::new(Day20::new()),
        Box::new(Day21::new()),
        Box::new(Day22::new()),
        Box::new(Day23::new()),
        Box::new(Day24::new()),
        Box::new(Day25::new()),
    ];

    let mut days: Vec<Box<dyn AdventOfCode>> = Vec::from(array);
    let value_width = 14;
    let time_width = 7;
    println!(
        "+ {:->4} + {:-<value_width$} + {:-<time_width$} + {:-<value_width$} + {:-<time_width$} +",
        "", "", "", "", ""
    );
    println!(
        "| {:>4} | {:<value_width$} | {:<time_width$} | {:<value_width$} | {:<time_width$} |",
        "Day", "Puzzle 1", "Time", "Puzzle 2", "Time"
    );
    println!(
        "+ {:->4} + {:-<value_width$} + {:-<time_width$} + {:-<value_width$} + {:-<time_width$} +",
        "", "", "", "", ""
    );
    for (day, puzzle) in days.iter_mut().enumerate() {
        let input_str: String =
            file_util::file_to_string(format!("inputs/{}.txt", puzzle.day_str()));

        let not_done = "-".to_string();

        let start1 = Instant::now();
        puzzle.run_puzzle1(input_str.clone());
        let result1 = puzzle.get_puzzle1_result();
        let mut puzzle1_time_str: String = start1.elapsed().as_micros().to_string() + " us";
        let mut result1_str: String = result1.to_string();

        if result1 == 0 {
            result1_str = not_done.clone();
            puzzle1_time_str = not_done.clone() + " us";
        }

        let start2 = Instant::now();
        puzzle.run_puzzle2(input_str.clone());
        let result2 = puzzle.get_puzzle2_result();
        let mut puzzle2_time_str = start2.elapsed().as_micros().to_string() + " us";
        let mut result2_str: String = result2.to_string();
        if result2 == 0 {
            result2_str = not_done.clone();
            puzzle2_time_str = not_done.clone() + " us";
        }

        if result1 == 0 && result2 == 0 {
            //continue;
        }
        println!(
            "| {:>4} | {:<value_width$} | {:<time_width$} | {:<value_width$} | {:<time_width$} |",
            day + 1,
            result1_str,
            puzzle1_time_str,
            result2_str,
            puzzle2_time_str
        );
    }
    println!(
        "+ {:->4} + {:-<value_width$} + {:-<time_width$} + {:-<value_width$} + {:-<time_width$} +",
        "", "", "", "", ""
    );
}

fn main() {
    run_all_puzzles();
}
