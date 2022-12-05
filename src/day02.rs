use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::any::Any;
default_aoc_struct!(Day2, i32);
default_new_ctor!(Day2);

impl Day2 {
    pub fn translate_hand(&self, action: char) -> char {
        match action {
            'X' => 'A',
            'Y' => 'B',
            'Z' => 'C',
            _ => panic!("Bad input"),
        }
    }

    pub fn winning_action(&self, action: char) -> char {
        match action {
            'A' => 'B',
            'B' => 'C',
            'C' => 'A',
            _ => panic!("Bad input"),
        }
    }

    pub fn lose_action(&self, action: char) -> char {
        match action {
            'A' => 'C',
            'B' => 'A',
            'C' => 'B',
            _ => panic!("Bad input"),
        }
    }

    pub fn translate_hand_v2(&self, elf_hand: char, my_action: char) -> char {
        match my_action {
            'X' => self.lose_action(elf_hand),
            'Y' => elf_hand,
            'Z' => self.winning_action(elf_hand),
            _ => panic!("Bad input"),
        }
    }

    pub fn get_point(&self, hand: char) -> i32 {
        match hand {
            'A' => 1, // Stone
            'B' => 2, // Paper
            'C' => 3, // Scissors
            _ => 0,
        }
    }

    pub fn get_score(&self, elf_hand: char, my_hand: char) -> i32 {
        if elf_hand == my_hand {
            self.get_point(elf_hand) + 3
        } else if self.winning_action(elf_hand) == my_hand {
            self.get_point(my_hand) + 6
        } else {
            self.get_point(my_hand)
        }
    }
}

impl AdventOfCode for Day2 {
    fn day_str(&self) -> String {
        "day02".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let rps_rounds: Vec<&str> = input_str.split('\n').collect();

        let mut score: i32 = 0;
        for rps in rps_rounds {
            let mut game = rps.chars().filter(|c| !c.is_whitespace());
            let elf_hand = game.next().unwrap();
            let my_hand = self.translate_hand(game.next().unwrap());

            score += self.get_score(elf_hand, my_hand);
        }
        self.puzzle1_result = score;
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let rps_rounds: Vec<&str> = input_str.split('\n').collect();

        let mut score: i32 = 0;
        for rps in rps_rounds {
            let mut game = rps.chars().filter(|c| !c.is_whitespace());
            let elf_hand = game.next().unwrap();
            let my_hand = self.translate_hand_v2(elf_hand, game.next().unwrap());

            score += self.get_score(elf_hand, my_hand);
        }
        self.puzzle2_result = score;
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

    puzzle1_test!(Day2, 15, 11475);
    puzzle2_test!(Day2, 12, 16862);
}
