use crate::{aoc::AdventOfCode, file_util};

pub struct Day1 {}

impl Day1 {}

impl AdventOfCode for Day1 {
    fn day_str(&self) -> String {
        "day01".to_owned()
    }

    fn run_puzzle1(&self, file_path: String) {
        let input_str = file_util::file_to_string(file_path);
    }

    fn run_puzzle2(&self, file_path: String) {
        let input_str = file_util::file_to_string(file_path);
    }

    fn get_puzzle1_result(&self) -> i32 {
        0
    }

    fn get_puzzle2_result(&self) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1() {
        let day = Day1 {};

        let test_file = format!("inputs/{}_test.txt", day.day_str());
        day.run_puzzle1(test_file);
        assert_eq!(0, day.get_puzzle1_result());

        let main_file = format!("inputs/{}.txt", day.day_str());
        day.run_puzzle1(main_file);
        assert_eq!(0, day.get_puzzle1_result());
    }

    #[test]
    fn puzzle_2() {
        let day = Day1 {};

        let test_file = format!("inputs/{}_test.txt", day.day_str());
        day.run_puzzle2(test_file);
        assert_eq!(0, day.get_puzzle2_result());

        let main_file = format!("inputs/{}.txt", day.day_str());
        day.run_puzzle2(main_file);
        assert_eq!(0, day.get_puzzle2_result());
    }
}
