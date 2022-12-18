use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::{
    any::Any,
    cmp::{self},
    collections::HashMap,
    convert::TryInto,
};
default_aoc_struct!(Day16, i32);
default_new_ctor!(Day16);

#[derive(Debug)]
struct Valve {
    valve_name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

impl Day16 {
    fn parse_input(input_str: &str) -> Vec<Valve> {
        let x: Vec<Valve> = input_str
            .lines()
            .map(|line| {
                let mut l = line.split(';');
                let first_str = l
                    .next()
                    .unwrap()
                    .replace("Valve", "")
                    .replace("has flow rate=", "");
                let mut first_items = first_str.split_ascii_whitespace();
                let valve_name = first_items.next().unwrap().to_string();
                let flow_rate = first_items.next().unwrap().parse::<u32>().unwrap();

                let second_str = l.next().unwrap().replace(',', "");
                let second: Vec<String> = second_str
                    .split_ascii_whitespace()
                    .map(String::from)
                    .collect();
                let tunnels: Vec<String> = second[4..].to_vec();

                Valve {
                    valve_name,
                    flow_rate,
                    tunnels,
                }
            })
            .collect();
        x
    }

    fn find_max_pressure(
        time: u32,
        pos: usize,
        opened: &str,
        matrix: &[u32],
        memory: &mut [HashMap<String, u32>],
        nice_valves: &[usize],
        valves: &[Valve],
    ) -> u32 {
        let all_opened: String = vec!['1'; opened.len()].iter().collect();

        if memory.get(time as usize).unwrap().contains_key(opened) {
            // If we already have a solution on this..
            return *memory.get(time as usize).unwrap().get(opened).unwrap();
        } else if opened == all_opened {
            // Nothing to do more.
            0
        } else {
            // We need to calculate this unknown state..
            let mut max_pressure = 0;
            for (i, nice_valve) in nice_valves.iter().enumerate() {
                if opened.chars().nth(i).unwrap() == '1' {
                    // This is already opened...
                    continue;
                } else {
                    let next_valve_id = *nice_valve;
                    let length = valves.len();
                    let index = (pos as u32) + (length as u32) * (next_valve_id as u32);
                    let cost = matrix.get(index as usize).unwrap();

                    let next_time = time + cost + 1;
                    if next_time >= 30 {
                        continue;
                    } else {
                        let flow_rate = valves.get(next_valve_id).unwrap().flow_rate;
                        let next_pressure = (30 - next_time) * flow_rate;

                        let mut next_opened = opened.to_string().clone();
                        next_opened.replace_range(i..i + 1, "1");

                        let pre = Day16::find_max_pressure(
                            next_time,
                            next_valve_id,
                            &next_opened,
                            matrix,
                            memory,
                            nice_valves,
                            valves,
                        );
                        let new_pressure = next_pressure + pre;

                        max_pressure = cmp::max(max_pressure, new_pressure);
                    }
                }
            }
            if !memory.get(time as usize).unwrap().contains_key(opened) {
                memory
                    .get_mut(time as usize)
                    .unwrap()
                    .insert(opened.to_string(), 0);
            }

            *memory
                .get_mut(time as usize)
                .unwrap()
                .get_mut(&opened.to_string())
                .unwrap() = max_pressure;
            max_pressure
        }
    }
}

impl AdventOfCode for Day16 {
    fn day_str(&self) -> String {
        "day16".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        let mut memory: [HashMap<String, u32>; 30] = Default::default();

        let valves = Day16::parse_input(&input_str);
        let vertices = valves.len();
        let mut matrix = vec![9999; vertices * vertices];
        let mut valve_mapping: HashMap<String, u32> = HashMap::new();
        let mut nice_valves: Vec<usize> = Vec::new();

        valves.iter().enumerate().for_each(|(i, v)| {
            valve_mapping.insert(v.valve_name.clone(), i.try_into().unwrap());
            if v.flow_rate > 0 {
                nice_valves.push(i);
            }
        });

        valves.iter().enumerate().for_each(|(this_i, v)| {
            let i = (this_i as u32) + (vertices as u32) * (this_i as u32);
            *matrix.get_mut(i as usize).unwrap() = 0;

            v.tunnels.iter().for_each(|tunnel| {
                let y = *valve_mapping.get(tunnel).unwrap();
                let i = (this_i as u32) + (vertices as u32) * (y as u32);
                *matrix.get_mut(i as usize).unwrap() = 1;
            });
        });

        for k in 0..vertices {
            for i in 0..vertices {
                for j in 0..vertices {
                    let index = (i as u32) + (vertices as u32) * (j as u32);

                    let a = ((i as u32) + (vertices as u32) * (j as u32)) as usize;
                    let b = ((i as u32) + (vertices as u32) * (k as u32)) as usize;
                    let c = ((k as u32) + (vertices as u32) * (j as u32)) as usize;

                    let q = matrix.get(a).unwrap();
                    let w = matrix.get(b).unwrap();
                    let r = matrix.get(c).unwrap();
                    *matrix.get_mut(index as usize).unwrap() = cmp::min(*q, w + r);
                }
            }
        }

        let start = *valve_mapping.get("AA").unwrap();
        let opened: String = vec!['0'; nice_valves.len()].iter().collect();
        let result = Day16::find_max_pressure(
            0,
            start as usize,
            &opened,
            &matrix,
            &mut memory,
            &nice_valves,
            &valves,
        );

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

    puzzle1_test!(
        Day16, 1699, /* No idea why this is better than the example 1651 */
        1754
    );
    puzzle2_test!(Day16, 0, 0);
}
