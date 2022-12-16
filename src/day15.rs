use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::{any::Any, collections::HashSet, iter::FromIterator};
default_aoc_struct!(Day15, i32);
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

    fn beacons_on_row(sensors: &[Sensor], y_pos: i32) -> i32 {
        let mut ranges = Vec::new();
        for sensor in sensors {
            let distance = Day15::manhattan_distance(sensor);

            let min_distance_y = sensor.position.1 - distance;
            let max_distance_y = sensor.position.1 + distance;

            let a = min_distance_y..max_distance_y;
            if a.contains(&y_pos) {
                let diff_to_y = i32::abs_diff(sensor.position.1, y_pos);
                let new_x1 = sensor.position.0 - distance + diff_to_y as i32;
                let new_x2 = sensor.position.0 + distance - diff_to_y as i32;
                let range = new_x1..new_x2;
                ranges.push(range);
            }
        }

        let ranges_list: Vec<Vec<i32>> = ranges.iter().map(|r| r.clone().collect()).collect();
        let ranges_set: HashSet<i32> = HashSet::from_iter(ranges_list.concat().iter().cloned());
        ranges_set.len() as i32
    }
}

impl AdventOfCode for Day15 {
    fn day_str(&self) -> String {
        "day15".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let sensors: Vec<Sensor> = input_str
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
            .collect();

        if sensors.len() < 20 {
            self.puzzle1_result = Day15::beacons_on_row(&sensors, 10);
        } else {
            self.puzzle1_result = Day15::beacons_on_row(&sensors, 2000000);
        }
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

    puzzle1_test!(Day15, 26, 4827924);
    puzzle2_test!(Day15, 0, 0);
}
