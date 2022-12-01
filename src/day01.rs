use crate::{aoc::AdventOfCode, file_util};

#[derive(Default)]
pub struct Day1 {
    pub puzzle1_result: i32,
    pub puzzle2_result: i32,
}

impl Day1 {
    fn get_elf_inventory(&self, file_path: String) -> Vec<Vec<i32>> {
        let input_str: String = file_util::file_to_string(file_path);

        let elf_foods: Vec<&str> = input_str.split("\n\n").collect();
        let mut elf_vector: Vec<Vec<i32>> = Vec::new();

        for elf_food in elf_foods.iter() {
            let v: Vec<i32> = elf_food
                .split('\n')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            elf_vector.push(v);
        }
        elf_vector
    }
}

impl AdventOfCode for Day1 {
    fn day_str(&self) -> String {
        "day01".to_owned()
    }

    fn run_puzzle1(&mut self, file_path: String) {
        let elf_inventory: Vec<Vec<i32>> = self.get_elf_inventory(file_path);

        self.puzzle1_result = elf_inventory
            .iter()
            .map(|x| x.iter().sum::<i32>())
            .max()
            .unwrap();
    }

    fn run_puzzle2(&mut self, file_path: String) {
        let elf_inventory: Vec<Vec<i32>> = self.get_elf_inventory(file_path);

        let mut total_elf_cal_inventory: Vec<i32> = elf_inventory
            .iter()
            .map(|x| x.iter().sum::<i32>())
            .collect();

        total_elf_cal_inventory.sort();

        self.puzzle2_result = total_elf_cal_inventory.iter().rev().take(3).sum();
    }

    fn get_puzzle1_result(&self) -> i32 {
        self.puzzle1_result
    }

    fn get_puzzle2_result(&self) -> i32 {
        self.puzzle2_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1() {
        let mut day = Day1 {
            puzzle1_result: 0,
            puzzle2_result: 0,
        };

        let test_file = format!("inputs/{}_test.txt", day.day_str());
        day.run_puzzle1(test_file);
        assert_eq!(24000, day.get_puzzle1_result());

        let main_file = format!("inputs/{}.txt", day.day_str());
        day.run_puzzle1(main_file);
        assert_eq!(69836, day.get_puzzle1_result());
    }

    #[test]
    fn puzzle_2() {
        let mut day = Day1 {
            puzzle1_result: 0,
            puzzle2_result: 0,
        };

        let test_file = format!("inputs/{}_test.txt", day.day_str());
        day.run_puzzle2(test_file);
        assert_eq!(45000, day.get_puzzle2_result());

        let main_file = format!("inputs/{}.txt", day.day_str());
        day.run_puzzle2(main_file);
        assert_eq!(207968, day.get_puzzle2_result());
    }
}
