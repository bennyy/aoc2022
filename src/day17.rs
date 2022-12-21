use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::any::Any;
default_aoc_struct!(Day17, i32);
default_new_ctor!(Day17);

impl Day17 {
    fn parse_input(input_str: &str) -> Vec<char> {
        input_str.chars().collect()
    }

    fn chamber_to_str(
        chamber: &[u32],
        width: usize,
        current_shape: Option<Vec<(u32, u32)>>,
    ) -> String {
        let mut chamber_copy = chamber.to_vec();
        if current_shape.is_some() {
            for p in current_shape.unwrap().iter() {
                let i = (p.0 as u32) + (width as u32) * (p.1 as u32);
                *chamber_copy.get_mut(i as usize).unwrap() = 9;
            }
        }

        let mut return_string = "  0123456\n".to_owned();
        for (i, row) in chamber_copy.chunks(width).enumerate() {
            let joned: String = row
                .to_vec()
                .iter()
                .map(|&id| match id {
                    0 => ".",
                    1 => "#",
                    9 => "@",
                    _ => "?",
                })
                .collect();
            return_string.push_str(&i.to_string());
            return_string.push(' ');
            return_string.push_str(&joned);
            return_string.push('\n');
        }
        return_string
    }

    fn increase_chamber(chamber: &mut Vec<u32>, width: usize, height: usize) {
        let mut extra_chamber: Vec<u32> = vec![0; width * height];
        extra_chamber.extend(&mut chamber.iter());
        *chamber = extra_chamber;
    }

    fn translate_shape(shape: &[(u32, u32)], x: u32, y: u32) -> Vec<(u32, u32)> {
        let mut shape_copy = Vec::new();
        for s in shape.iter() {
            let new_x = s.0 + x;
            let new_y = y - s.1;
            shape_copy.push((new_x, new_y));
        }
        shape_copy
    }

    fn legal_x_move(shape: &[(u32, u32)], width: u32, chamber: &[u32]) -> bool {
        for c in shape.iter() {
            let i = (c.0 as u32) + (width as u32) * (c.1 as u32);

            let x = c.0;

            if x < width && *chamber.get(i as usize).unwrap() == 0 {
            } else {
                return false;
            }
        }

        true
    }

    fn legal_y_move(shape: &[(u32, u32)], width: u32, height: u32, chamber: &Vec<u32>) -> bool {
        for c in shape.iter() {
            let mut is_another_rock = false;
            let i = ((c.0 as u32) + (width as u32) * (c.1 as u32)) as usize;
            if i < chamber.len() {
                is_another_rock = *chamber.get(i).unwrap() == 1
            }

            let y = c.1;
            if y < height && !is_another_rock {
            } else {
                return false;
            }
        }

        true
    }

    fn get_shape_height(shape: &[(u32, u32)]) -> u32 {
        let min = shape.iter().min_by(|&x, &y| x.1.cmp(&y.1)).unwrap();
        let max = shape.iter().max_by(|&x, &y| x.1.cmp(&y.1)).unwrap();
        u32::abs_diff(min.1, max.1) + 1
    }

    fn get_current_cave_height(chamber: &[u32], width: usize) -> u32 {
        let mut res = 0;
        for (i, chonk) in chamber.to_vec().chunks(width).enumerate() {
            let all_air = chonk.iter().all(|&x| x == 0);
            if !all_air {
                return i as u32;
            }
            res += 1;
        }
        res
    }
    fn get_current_rock_height(chamber: &[u32], width: usize) -> u32 {
        let mut res = 0;
        for (_i, chonk) in chamber.to_vec().chunks(width).enumerate() {
            let rock = chonk.iter().any(|&x| x == 1);
            if rock {
                res += 1;
            }
        }
        res
    }

    fn get_next_floor_height(chamber: &[u32], width: usize) -> u32 {
        for (i, chonk) in chamber.to_vec().chunks(width).enumerate() {
            let all_rock = chonk.iter().all(|&x| x == 1);
            if all_rock {
                return i as u32;
            }
        }
        0
    }
}

impl AdventOfCode for Day17 {
    fn day_str(&self) -> String {
        "day17".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let shapes: Vec<Vec<(u32, u32)>> = vec![
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        ];

        let moves = Day17::parse_input(&input_str);

        let width: usize = 7;
        let mut height: usize = 20;
        let mut chamber = vec![0; width * height];

        let mut shape_counter = 0;
        let mut main_shape = shapes.first().unwrap();

        let current_cave_height = Day17::get_current_cave_height(&chamber, width);

        let mut curr_x = 2;
        let mut curr_y = current_cave_height - 3 - 1;
        let mut rocks_stopped = 0;
        let mut temp_rock_height = 0;
        let mut i = 0;
        while rocks_stopped < 2022_i64 {
            // First beeing pushed by a jet of hot gas
            let mov = moves.get(i % moves.len()).unwrap();
            let mut wanted_x = curr_x;
            let wanted_y = curr_y + 1;
            match mov {
                '>' => wanted_x += 1,
                '<' => {
                    if wanted_x > 0 {
                        wanted_x -= 1
                    }
                }
                _ => panic!("Unwanted char: {}", mov),
            }

            let mut temp_shape = Day17::translate_shape(main_shape, wanted_x, curr_y);

            let can_move = Day17::legal_x_move(&temp_shape, width as u32, &chamber);
            if can_move {
                curr_x = wanted_x;
            } else {
                temp_shape = Day17::translate_shape(main_shape, curr_x, curr_y);
            }
            let tempy_shape = Day17::translate_shape(main_shape, curr_x, wanted_y);

            let can_move_down =
                Day17::legal_y_move(&tempy_shape, width as u32, height as u32, &chamber);
            if can_move_down {
                curr_y = wanted_y;
            } else {
                for p in temp_shape.iter() {
                    let i = (p.0 as u32) + (width as u32) * (p.1 as u32);
                    *chamber.get_mut(i as usize).unwrap() = 1;
                }
                rocks_stopped += 1;

                let hhh = Day17::get_current_cave_height(&chamber, width);

                shape_counter += 1;
                let shape_index = shape_counter % shapes.len();
                main_shape = shapes.get(shape_index).unwrap();

                let shape_len = Day17::get_shape_height(main_shape);
                if shape_len + 3 > hhh {
                    let increased_size = shape_len as usize + 10;
                    Day17::increase_chamber(&mut chamber, width, increased_size);
                    height += increased_size
                }

                let y_where_all_rocks = Day17::get_next_floor_height(&chamber, width);
                if y_where_all_rocks > 1 {
                    let y = y_where_all_rocks;
                    let i_cut_point = y as usize * width as usize;
                    let diff = height - y_where_all_rocks as usize;
                    // Cut of the end of the vector

                    chamber.drain(i_cut_point..);
                    height -= diff;
                    temp_rock_height += diff;
                }

                let hhh = Day17::get_current_cave_height(&chamber, width);

                curr_x = 2;
                curr_y = hhh - 3 - 1;
            }
            i += 1;
        }

        let result = Day17::get_current_rock_height(&chamber, width) + temp_rock_height as u32;
        self.puzzle1_result = result as i32;
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

    puzzle1_test!(Day17, 3068, 3147);
    puzzle2_test!(Day17, 0, 0);
}
