use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::{any::Any, cmp};
default_aoc_struct!(Day14, i32);
default_new_ctor!(Day14);

const AIR: usize = 0;
const WALL: usize = 1;
const SAND: usize = 2;
const PATH: usize = 99;

impl Day14 {
    fn get_neighbours(current_index: usize, width: usize, cave_length: usize) -> Vec<usize> {
        let directions: Vec<(i32, i32)> = vec![
            (0, 1),  /*down */
            (-1, 1), /* dia left */
            (1, 1),  /* dia right */
        ];
        let mut neighbors: Vec<usize> = Vec::new();
        for v in directions {
            let x = (current_index % width) as i32 + v.0;
            let y = (current_index / width) as i32 + v.1;

            if x >= width as i32 || x < 0 || y < 0 || y >= cave_length as i32 / width as i32 {
                // Outside the limit..
                continue;
            }
            let i = (x as u32) + (width as u32) * (y as u32);
            neighbors.push(i as usize);
        }
        neighbors
    }

    fn print_cave(cave: &[usize], width: usize) {
        let mut cave_str: String = "".to_owned();
        cave.chunks(width as usize).for_each(|line| {
            let joned: String = line[400..]
                .to_vec()
                .iter()
                .map(|&id| match id {
                    AIR => ".",
                    WALL => "#",
                    SAND => "o",
                    PATH => "~",
                    _ => "?",
                })
                .collect();
            cave_str.push_str(&joned);
            cave_str.push('\n');
        });

        println!("{}", cave_str);
    }

    fn get_rock_lines(input_str: &str) -> Vec<Vec<(u32, u32)>> {
        input_str
            .lines()
            .map(|line| {
                let coords = line
                    .split(" -> ")
                    .map(|co| {
                        let mut xy = co.split(',');
                        let x = xy.next().unwrap().parse::<u32>().unwrap();
                        let y = xy.next().unwrap().parse::<u32>().unwrap();
                        (x, y)
                    })
                    .collect();
                coords
            })
            .collect()
    }

    fn get_min_max_coords(rock_lines: &[Vec<(u32, u32)>]) -> (usize, usize) {
        let mut max_x = 0;
        let mut max_y = 0;
        for rock_line in rock_lines.iter() {
            for rock_coord in rock_line {
                if rock_coord.0 > max_x {
                    max_x = rock_coord.0;
                }
                if rock_coord.1 > max_y {
                    max_y = rock_coord.1;
                }
            }
        }
        (max_x as usize, max_y as usize)
    }

    fn create_cave(rock_lines: &[Vec<(u32, u32)>], width: usize, height: usize) -> Vec<usize> {
        let total_size = height * width;
        let mut cave = vec![AIR; total_size as usize];

        for rock_lines_ in rock_lines.iter() {
            for rock_line in rock_lines_.windows(2) {
                let start_x = (rock_line[0].0) as usize;
                let start_y = (rock_line[0].1) as usize;
                let end_x = (rock_line[1].0) as usize;
                let end_y = (rock_line[1].1) as usize;

                let start_i = (start_x + width * (start_y)) as usize;
                let end_i = (end_x + width * end_y) as usize;

                if start_x == end_x {
                    let steps = usize::abs_diff(start_y, end_y) as usize + 1;
                    let min = cmp::min(start_i, end_i) as usize;
                    cave[min..]
                        .iter_mut()
                        .step_by(width as usize)
                        .take(steps)
                        .for_each(|x| *x = 1);
                } else if start_y == end_y {
                    let diff = usize::abs_diff(end_i, start_i) + 1;
                    let min = cmp::min(start_i, end_i) as usize;
                    cave[min..min + diff].fill(1);
                } else {
                    panic!("Wrong coordinate...");
                }
            }
        }
        cave
    }

    fn check_part1(neighbors: &[usize]) -> bool {
        neighbors.len() != 3
    }

    fn check_part2(current_sand: usize, neighbors: &[usize], cave: &mut [usize]) -> bool {
        let mut iter = neighbors.iter();

        let down = *iter.next().unwrap();
        let down_left = *iter.next().unwrap();
        let down_right = *iter.next().unwrap();

        let cave_down = *cave.get(down).unwrap();
        let cave_down_left = *cave.get(down_left).unwrap();
        let cave_down_right = *cave.get(down_right).unwrap();

        if current_sand == 500
            && cave_down == SAND
            && cave_down_left == SAND
            && cave_down_right == SAND
        {
            return true;
        }
        false
    }

    fn drop_sand(current_sand: usize, neighbors: &[usize], cave: &mut [usize]) -> Option<usize> {
        *cave.get_mut(current_sand).unwrap() = 99;

        let mut iter = neighbors.iter();

        let down = *iter.next().unwrap();
        let down_left = *iter.next().unwrap();
        let down_right = *iter.next().unwrap();

        let cave_down = *cave.get(down).unwrap();
        let cave_down_left = *cave.get(down_left).unwrap();
        let cave_down_right = *cave.get(down_right).unwrap();

        if cave_down == AIR {
            // If we can go down, go down!!
            return Some(down);
        }

        // Next step is a wall or sand.
        if cave_down == WALL || cave_down == SAND {
            // The unit of sand attempts to instead move diagonally
            // one step down and to the left.
            if cave_down_left == AIR {
                return Some(down_left);
            }

            //If that tile is blocked, the unit of sand attempts to
            // instead move diagonally one step down and to the right
            if cave_down_right == AIR {
                return Some(down_right);
            }

            // If all three possible destinations are blocked,
            //the unit of sand comes to rest and no longer moves
            if cave_down > AIR && cave_down_left > AIR && cave_down_right > AIR {
                *cave.get_mut(current_sand).unwrap() = 2;
                return None;
            }
        }

        panic!("Not implemented");
    }
}

impl AdventOfCode for Day14 {
    fn day_str(&self) -> String {
        "day14".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let rock_lines = Day14::get_rock_lines(&input_str);
        let (max_x, max_y) = Day14::get_min_max_coords(&rock_lines);

        let width = max_x as usize + 1;
        let height = max_y as usize + 1;

        let mut cave = Day14::create_cave(&rock_lines, width, height);

        let rock_start_index = 500;
        let cave_length = cave.len();

        let mut flood_done = false;
        while !flood_done {
            let mut current_sand = rock_start_index;

            // Clear path from the map, useful for debugging.
            cave.iter_mut()
                .filter(|n| **n == PATH)
                .for_each(|x| *x = AIR);

            loop {
                let neighbors = Day14::get_neighbours(current_sand, width, cave_length);

                if Day14::check_part1(&neighbors) {
                    flood_done = true;
                    break;
                }

                if let Some(asdf) = Day14::drop_sand(current_sand, &neighbors, &mut cave) {
                    current_sand = asdf;
                    continue;
                } else {
                    break;
                }
            }
        }
        // Day14::print_cave(&cave, width);

        self.puzzle1_result = cave.iter().filter(|&n| *n == SAND).count() as i32;
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let mut rock_lines = Day14::get_rock_lines(&input_str);
        let (_, mut max_y) = Day14::get_min_max_coords(&rock_lines);

        let max_x = 1000;
        max_y += 2;

        let bottom_line: Vec<(u32, u32)> = vec![(0, max_y as u32), (800, max_y as u32)];

        rock_lines.push(bottom_line);

        let width = max_x as usize + 1;
        let height = max_y as usize + 1;

        let mut cave = Day14::create_cave(&rock_lines, width, height);

        let rock_start_index = 500;
        let cave_length = cave.len();

        let mut flood_done = false;
        while !flood_done {
            let mut current_sand = rock_start_index;

            // Clear path from the map, useful for debugging.
            cave.iter_mut()
                .filter(|n| **n == PATH)
                .for_each(|x| *x = AIR);

            loop {
                let neighbors = Day14::get_neighbours(current_sand, width, cave_length);
                let res = Day14::drop_sand(current_sand, &neighbors, &mut cave);
                if Day14::check_part2(current_sand, &neighbors, &mut cave) {
                    flood_done = true;
                    break;
                }

                if let Some(new_sand_pos) = res {
                    current_sand = new_sand_pos;
                    continue;
                } else {
                    break;
                }
            }
            //Day14::print_cave(&cave, width);
        }

        self.puzzle2_result = cave.iter().filter(|&n| *n == SAND).count() as i32;
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

    puzzle1_test!(Day14, 24, 719);
    puzzle2_test!(Day14, 93, 23390);
}
