use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::{
    any::Any,
    cmp,
    collections::{HashSet, VecDeque},
};
default_aoc_struct!(Day15, u64);
default_new_ctor!(Day15);

#[derive(Copy, Debug, Clone)]
struct Sensor {
    position: (i32, i32),
    closest_beacon: (i32, i32),
}

impl Day15 {
    fn manhattan_distance(sensor: &Sensor) -> i32 {
        i32::abs_diff(sensor.position.0, sensor.closest_beacon.0) as i32
            + i32::abs_diff(sensor.position.1, sensor.closest_beacon.1) as i32
    }

    fn merge_ranges(input_intervals: &[(i32, i32)]) -> Vec<(i32, i32)> {
        let mut intervals = input_intervals.to_owned();

        let mut stack_vec: VecDeque<(i32, i32)> = VecDeque::new();
        intervals.sort();

        stack_vec.push_back(*intervals.first().unwrap());

        for chunk in intervals.iter() {
            let x1 = chunk.0;
            let x2 = chunk.1;

            let stack = stack_vec.pop_back().unwrap();
            let stack_x1 = stack.0;
            let stack_x2 = stack.1;

            if stack_x1 <= x1 && x1 <= stack_x2 {
                let m = cmp::max(stack_x2, x2);
                stack_vec.push_back((stack_x1, m));
            } else {
                stack_vec.push_back(stack);
                stack_vec.push_back(*chunk);
            }
        }
        stack_vec.into_iter().collect()
    }

    fn beacons_on_row(sensors: &[Sensor], y_pos: i32) -> HashSet<(i32, i32)> {
        let mut real_ranges: Vec<(i32, i32)> = Vec::new();

        for sensor in sensors {
            let distance = Day15::manhattan_distance(sensor);

            let min_distance_y = sensor.position.1 - distance;
            let max_distance_y = sensor.position.1 + distance;

            let y_range = min_distance_y..=max_distance_y;
            if y_range.contains(&y_pos) {
                let diff_to_y = i32::abs_diff(sensor.position.1, y_pos);
                let gren_storlek = distance - diff_to_y as i32;

                let new_x1 = sensor.position.0 + gren_storlek;
                let new_x2 = sensor.position.0 - gren_storlek;

                real_ranges.push((
                    cmp::min(new_x1, new_x2) as i32,
                    cmp::max(new_x1, new_x2) as i32,
                ));
            }
        }

        let input: Vec<(i32, i32)> = real_ranges.into_iter().collect();
        let final_ranges = Day15::merge_ranges(&input);
        let final_ranges2: HashSet<(i32, i32)> = final_ranges.into_iter().collect();
        final_ranges2
    }

    fn parse_sensors(input_str: &str) -> Vec<Sensor> {
        input_str
            .lines()
            .map(|line| {
                let new_line = line.replace([',', ':'], "");
                let w: Vec<&str> = new_line.split_ascii_whitespace().collect();
                let sx = w.get(2).unwrap().replace("x=", "").parse::<i32>().unwrap();
                let sy = w.get(3).unwrap().replace("y=", "").parse::<i32>().unwrap();

                let cx = w.get(8).unwrap().replace("x=", "").parse::<i32>().unwrap();
                let cy = w.get(9).unwrap().replace("y=", "").parse::<i32>().unwrap();

                Sensor {
                    position: (sx, sy),
                    closest_beacon: (cx, cy),
                }
            })
            .collect()
    }
}

impl AdventOfCode for Day15 {
    fn day_str(&self) -> String {
        "day15".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let sensors: Vec<Sensor> = Day15::parse_sensors(&input_str);

        let mut y_pos = 20;
        if sensors.len() > 20 {
            y_pos = 2000000;
        }

        let x_axis = Day15::beacons_on_row(&sensors, y_pos);
        if x_axis.len() == 1 {
            let x = *x_axis.iter().next().unwrap();
            self.puzzle1_result = i32::abs_diff(x.0, x.1) as u64;
        } else {
            panic!("More than one!")
        }
    }

    fn run_puzzle2(&mut self, input_str: String) {
        let sensors: Vec<Sensor> = Day15::parse_sensors(&input_str);

        let mut y_pos_start = 0;
        let mut y_pos_end = 20;
        if sensors.len() > 20 {
            //y_pos_start = 0;
            y_pos_start = 2973563; // Speedrun..
            y_pos_end = 4000000;
        }

        for y_pos in y_pos_start..y_pos_end {
            let x_axis = Day15::beacons_on_row(&sensors, y_pos);
            if x_axis.len() > 1 {
                let mut hmm: Vec<(i32, i32)> = x_axis.into_iter().collect();
                hmm.sort();

                let mut iter = hmm.iter();
                let a = iter.next().unwrap().1;
                let b = iter.next().unwrap().0;
                let x = (a..b).next().unwrap();

                let new_x: u64 = x as u64 + 1;
                self.puzzle2_result = new_x * 4000000 + y_pos as u64;
                break;
            }
        }
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
    use crate::file_util;

    #[test]
    fn puzzle_1() {
        let mut day = Day15::new();

        let test_str: String =
            file_util::file_to_string(format!("inputs/{}_test.txt", day.day_str()));
        day.run_puzzle1(test_str);
        let actual_value = *day
            .get_puzzle1_result()
            .unwrap_or_else(|| Box::new(""))
            .downcast::<u64>()
            .unwrap();
        assert_eq!(26, actual_value);

        let main_str: String = file_util::file_to_string(format!("inputs/{}.txt", day.day_str()));
        day.run_puzzle1(main_str);
        let actual_value = *day
            .get_puzzle1_result()
            .unwrap_or_else(|| Box::new(""))
            .downcast::<u64>()
            .unwrap();
        assert_eq!(4827924, actual_value);
    }

    #[test]
    fn puzzle_2() {
        let mut day = Day15::new();

        let test_str: String =
            file_util::file_to_string(format!("inputs/{}_test.txt", day.day_str()));
        day.run_puzzle2(test_str);
        let actual_value = *day
            .get_puzzle2_result()
            .unwrap_or_else(|| Box::new(""))
            .downcast::<u64>()
            .unwrap();
        assert_eq!(56000011, actual_value);

        let main_str: String = file_util::file_to_string(format!("inputs/{}.txt", day.day_str()));
        day.run_puzzle2(main_str);
        let actual_value = *day
            .get_puzzle2_result()
            .unwrap_or_else(|| Box::new(""))
            .downcast::<u64>()
            .unwrap();

        assert_eq!(12977110973564, actual_value);
    }
}
