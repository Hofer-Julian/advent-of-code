use crate::regex;
use std::{collections::HashMap, todo};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day14, part2)]
fn parse_input_day14_part2(input: &str) -> Vec<Program> {
    let mut program = Program::default();

    let mut parsed_data = Vec::new();
    let re = regex!(r"mem\[(\d+)\] = (\d+)");

    for line in input.lines() {
        if let Some(captures) = re.captures(line) {
            let position = captures.get(1).unwrap().as_str().parse().unwrap();
            let value = captures.get(2).unwrap().as_str().parse().unwrap();
            program.write_instructions.insert(position, value);
        } else {
            parsed_data.push(program);
            program = Program::default();
            program.bitmask = line.chars().rev().collect();
        }
    }
    parsed_data.push(program);
    parsed_data.remove(0);

    parsed_data
}

#[derive(Debug, Default)]
struct Program {
    bitmask: Vec<char>,
    write_instructions: HashMap<u64, u64>,
}

#[aoc(day14, part2)]
fn sum_values_in_memory(input: &Vec<Program>) -> u64 {
    let actual_write_instruction = get_actual_write_instruction(input);
    actual_write_instruction
        .iter()
        .fold(0, |acc, (_, value)| acc + value)
}

fn get_actual_write_instruction(input: &Vec<Program>) -> HashMap<u64, u64> {
    input.iter().flat_map(get_single_mask).collect()
}
fn get_single_mask(program: &Program) -> HashMap<u64, u64> {
    program
        .write_instructions
        .iter()
        .flat_map(|(memory_address, value)| {
            get_single_write_instruction(&program.bitmask, memory_address, value)
        })
        .collect()
}

fn get_single_write_instruction(
    bitmask: &Vec<char>,
    memory_address: &u64,
    value: &u64,
) -> HashMap<u64, u64> {
    let mut memory_addresses = HashMap::new();
    for (index, bit) in bitmask.iter().enumerate() {
        match bit {
            'X' => todo!(),
            '1' => todo!(),
            '0' => (),
            _ => panic!("Invalid bit"),
        }
    }
    memory_addresses
}

#[cfg(test)]
#[test]
fn test_sum_values_in_memory() {
    let input = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    let instructions = parse_input_day14_part2(input);
    assert_eq!(208, sum_values_in_memory(&instructions));
}
