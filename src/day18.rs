use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::{
    any::Any,
    collections::{HashSet, VecDeque},
};
default_aoc_struct!(Day18, i32);
default_new_ctor!(Day18);

impl Day18 {
    fn parse_input(input_str: &str) -> Vec<(i32, i32, i32)> {
        input_str
            .lines()
            .map(|line| {
                let mut iter = line.split(',');
                (
                    iter.next().unwrap().parse::<i32>().unwrap(),
                    iter.next().unwrap().parse::<i32>().unwrap(),
                    iter.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .collect()
    }

    fn flatten_to_1d(coords: &[(i32, i32, i32)], width: i32, height: i32, depth: i32) -> Vec<i32> {
        coords
            .iter()
            .map(|(x, y, z)| (x + 1) + width * (y + 1) + height * depth * (z + 1))
            .collect()
    }
}

impl AdventOfCode for Day18 {
    fn day_str(&self) -> String {
        "day18".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let neighbours = vec![
            (1, 0, 0),
            (-1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ];
        const WIDTH: i32 = 22;
        const HEIGHT: i32 = 22;
        const DEPTH: i32 = 22;
        let coords = Day18::parse_input(&input_str);
        let space = Day18::flatten_to_1d(&coords, WIDTH, HEIGHT, DEPTH);

        let mut side = 0;
        for i in space.iter() {
            let x = i % WIDTH;
            let y = (i / WIDTH) % HEIGHT;
            let z = i / (WIDTH * HEIGHT);

            for (xo, yo, zo) in neighbours.iter() {
                let nx = x + xo;
                let ny = y + yo;
                let nz = z + zo;

                let ni = nx + WIDTH * ny + WIDTH * HEIGHT * nz;
                if !space.contains(&ni) {
                    side += 1;
                }
            }
        }
        self.puzzle1_result = side;
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let neighbours = vec![
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ];
        const WIDTH: i32 = 24;
        const HEIGHT: i32 = 24;
        const DEPTH: i32 = 24;

        let coords = Day18::parse_input(&input_str);
        let space = Day18::flatten_to_1d(&coords, WIDTH, HEIGHT, DEPTH);

        let mut prio_queue: VecDeque<i32> = VecDeque::new();
        let mut visited: HashSet<i32> = HashSet::new();

        prio_queue.push_back(0);
        let mut side = 0;
        while !prio_queue.is_empty() {
            let i = prio_queue.pop_front().unwrap();
            let x = i % WIDTH;
            let y = (i / WIDTH) % HEIGHT;
            let z = i / (WIDTH * HEIGHT);

            for (xo, yo, zo) in neighbours.iter() {
                let nx = x + xo;
                let ny = y + yo;
                let nz = z + zo;
                if !(0..=HEIGHT).contains(&nx)
                    || !(0..=HEIGHT).contains(&ny)
                    || !(0..=HEIGHT).contains(&nz)
                {
                    continue;
                }

                let ni = nx + WIDTH * ny + WIDTH * HEIGHT * nz;
                if ni < 0 {
                    panic!("> {}: {},{},{}", i, x, y, z);
                }

                if visited.contains(&ni) {
                    continue;
                }

                if space.contains(&ni) {
                    side += 1;
                    continue;
                }

                prio_queue.push_back(ni);
                visited.insert(ni);
            }
        }

        self.puzzle2_result = side;
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

    puzzle1_test!(Day18, 64, 4340);

    puzzle2_test!(Day18, 58, 2468);
}
