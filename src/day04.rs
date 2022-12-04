use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};

default_aoc_struct!(Day4);
default_new_ctor!(Day4);

impl Day4 {
    fn get_sections(input_str: String) -> Vec<Vec<i32>> {
        input_str
            .lines()
            .map(|x| {
                x.split(&[',', '-'])
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect()
    }
}

impl AdventOfCode for Day4 {
    fn day_str(&self) -> String {
        "day04".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        self.puzzle1_result = 0;

        let sections = Day4::get_sections(input_str);
        for sec in sections.iter() {
            let mut s = sec.iter();
            let (a1, a2, b1, b2) = (
                s.next().unwrap(),
                s.next().unwrap(),
                s.next().unwrap(),
                s.next().unwrap(),
            );

            if (a1 >= b1 && a2 <= b2) || (b1 >= a1 && b2 <= a2) {
                self.puzzle1_result += 1;
            }
        }
    }

    fn run_puzzle2(&mut self, input_str: String) {
        self.puzzle2_result = 0;

        let sections = Day4::get_sections(input_str);
        for sec in sections.iter() {
            let mut s = sec.iter();
            let (a1, a2, b1, b2) = (
                s.next().unwrap(),
                s.next().unwrap(),
                s.next().unwrap(),
                s.next().unwrap(),
            );

            if a1 >= b1 && a1 <= b2 {
                self.puzzle2_result += 1;
            } else if b1 >= a1 && b1 <= a2 {
                self.puzzle2_result += 1
            } else if b2 >= a1 && b2 <= a2 {
                self.puzzle2_result += 1;
            }
        }
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
    use crate::{puzzle1_test, puzzle2_test};

    puzzle1_test!(Day4, 2, 466);
    puzzle2_test!(Day4, 4, 865);
}
