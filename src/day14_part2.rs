use crate::regex;
use std::{collections::HashMap, todo};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Default)]
struct Program {
    bitmask: Vec<char>,
    instruction: HashMap<u64, u64>,
}

impl Program {
    fn new(bitmask: Vec<char>, write_instruction: HashMap<u64, u64>) -> Self {
        Self {
            bitmask,
            instruction: write_instruction,
        }
    }
}

#[aoc_generator(day14, part2)]
fn parse_input_day14_part2(input: &str) -> Vec<Program> {
    let mut program;

    let mut parsed_data = Vec::new();
    let re = regex!(r"mem\[(\d+)\] = (\d+)");

    for line in input.lines() {
        if let Some(captures) = re.captures(line) {
            let position = captures.get(1).unwrap().as_str().parse().unwrap();
            let value = captures.get(2).unwrap().as_str().parse().unwrap();
            program.instruction.insert(position, value);
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

#[aoc(day14, part2)]
fn sum_values_in_memory(input: &Vec<Program>) -> u64 {
    todo!()
}
