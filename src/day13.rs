use crate::{aoc::AdventOfCode, default_aoc_struct, default_new_ctor};
use std::{any::Any, cmp::Ordering};
default_aoc_struct!(Day13, i32);
default_new_ctor!(Day13);

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    Num(u32),
    List(Vec<Packet>),
}

impl Day13 {
    fn parse(input_str: String) -> Packet {
        // If the string begins with "[" lets parse this
        if input_str.starts_with('[') {
            let mut level = 0;
            // It's a list, then return a list
            Packet::List(
                // Remove the '[' and continue parsning..
                input_str[1..input_str.len() - 1]
                    // Step for each character
                    .split(|c| {
                        // '[' and ']' see how deep we're in the list-maniac
                        if c == '[' {
                            level += 1;
                        } else if c == ']' {
                            level -= 1;
                        }
                        // If level is 0 and we find a ','; we're in a list to be ready to parsed.
                        c == ',' && level == 0
                    })
                    .filter_map(|s| {
                        // Here is the input ready to be continued parsed..
                        (!s.is_empty()).then(|| Day13::parse(s.to_string()))
                    })
                    .collect(),
            )
        } else {
            // We have pure input, it's a number and return it.
            Packet::Num(input_str.parse().unwrap())
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // Nothing fancy with comparing numbers.
            (Packet::Num(left), Packet::Num(right)) => left.cmp(right),
            // Or lists, since the row above will be called for each element
            (Packet::List(left), Packet::List(right)) => left.cmp(right),
            // Take "self"  and move it to a list, since self it's a number.
            (Packet::Num(_), Packet::List(_)) => Packet::List(vec![self.clone()]).cmp(other),
            // "Self" is a list, but convert the other ("right") to a list, like above.
            (Packet::List(_), Packet::Num(_)) => self.cmp(&Packet::List(vec![other.clone()])),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl AdventOfCode for Day13 {
    fn day_str(&self) -> String {
        "day13".to_owned()
    }

    fn run_puzzle1(&mut self, input_str: String) {
        self.puzzle1_result = 0;

        let packet_pairs: Vec<(String, String)> = input_str
            .split("\n\n")
            .map(|chunk| {
                let mut lines = chunk.lines();

                (
                    lines.next().unwrap().to_string(),
                    lines.next().unwrap().to_string(),
                )
            })
            .collect();
        let mut sum = 0;

        for (i, packet_pair) in packet_pairs.iter().enumerate() {
            let left_stack = Day13::parse(packet_pair.0.to_owned());
            let right_stack = Day13::parse(packet_pair.1.to_owned());

            if left_stack.cmp(&right_stack) == Ordering::Less {
                sum += i + 1;
            }
        }
        self.puzzle1_result = sum as i32;
    }

    fn run_puzzle2(&mut self, input_str: String) {
        self.puzzle2_result = 0;

        let packet_pairs: Vec<(String, String)> = input_str
            .split("\n\n")
            .map(|chunk| {
                let mut lines = chunk.lines();
                (
                    lines.next().unwrap().to_string(),
                    lines.next().unwrap().to_string(),
                )
            })
            .collect();

        let start_packet = Day13::parse("[[2]]".to_owned());
        let end_packet = Day13::parse("[[6]]".to_owned());

        let mut packets = vec![start_packet.clone(), end_packet.clone()];

        for packet_pair in packet_pairs.iter() {
            packets.push(Day13::parse(packet_pair.0.to_owned()));
            packets.push(Day13::parse(packet_pair.1.to_owned()));
        }

        packets.sort();
        let mut start_index = 1;
        let mut end_index = 1;
        for (i, p) in packets.iter().enumerate() {
            if p.eq(&start_packet) {
                start_index += i;
            }
            if p.eq(&end_packet) {
                end_index += i;
            }
        }

        self.puzzle2_result = (start_index * end_index) as i32;
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

    puzzle1_test!(Day13, 13, 5208);
    puzzle2_test!(Day13, 140, 25792);
}
