use crate::regex;
use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day14, part1)]
fn parse_input_day14_part1(input: &str) -> Vec<(u64, u64, HashMap<u64, u64>)> {
    let mut zero_bitmap = u64::MAX;
    let mut one_bitmap = 0;
    let mut instruction = HashMap::new();

    let mut parsed_data = Vec::new();
    let re = regex!(r"mem\[(\d+)\] = (\d+)");

    for line in input.lines() {
        if let Some(captures) = re.captures(line) {
            let position = captures.get(1).unwrap().as_str().parse().unwrap();
            let value = captures.get(2).unwrap().as_str().parse().unwrap();
            instruction.insert(position, value);
            continue;
        } else {
            parsed_data.push((zero_bitmap, one_bitmap, instruction.clone()));
            zero_bitmap = u64::MAX;
            one_bitmap = 0;
            instruction.clear();
        }

        for (index, value) in line.chars().rev().enumerate() {
            match value {
                '0' => zero_bitmap &= u64::MAX - (1 << index),
                '1' => one_bitmap |= 1 << index,
                _ => (),
            }
        }
    }
    parsed_data.push((zero_bitmap, one_bitmap, instruction.clone()));
    parsed_data.remove(0);
    parsed_data
}

#[aoc(day14, part1)]
fn sum_values_in_memory(input: &Vec<(u64, u64, HashMap<u64, u64>)>) -> u64 {
    let mut final_memory = HashMap::new();

    for (zero_bitmap, one_bitmap, instruction) in input {
        for (location, value) in instruction {
            let new_value = (value & zero_bitmap) | one_bitmap;
            final_memory.insert(location, new_value);
        }
    }
    final_memory.iter().fold(0, |acc, (_, value)| acc + value)
}

#[cfg(test)]
#[test]
fn test_sum_values_in_memory() {
    let input = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    let instructions = parse_input_day14_part1(input);
    assert_eq!(165, sum_values_in_memory(&instructions));
}
