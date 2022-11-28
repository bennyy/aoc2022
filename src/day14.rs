use crate::{aoc::AdventOfCode, not_implemented};

pub struct Day14 {}

impl Day14 {}

not_implemented!(Day14, "day14");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1() {
        let day: Day14 = Day14 {};

        let test_file = format!("inputs/{}_test.txt", day.day_str());
        day.run_puzzle1(test_file);
        assert_eq!(0, day.get_puzzle1_result());

        let main_file = format!("inputs/{}.txt", day.day_str());
        day.run_puzzle1(main_file);
        assert_eq!(0, day.get_puzzle1_result());
    }

    #[test]
    fn puzzle_2() {
        let day: Day14 = Day14 {};

        let test_file = format!("inputs/{}_test.txt", day.day_str());
        day.run_puzzle2(test_file);
        assert_eq!(0, day.get_puzzle2_result());

        let main_file = format!("inputs/{}.txt", day.day_str());
        day.run_puzzle2(main_file);
        assert_eq!(0, day.get_puzzle2_result());
    }
}
