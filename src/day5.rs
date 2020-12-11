use crate::regex;
use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp;

enum Partition {
    Front,
    Back,
    Left,
    Right,
}

#[aoc_generator(day5)]
fn parse_input_day5(input: &str) -> Vec<Vec<Partition>> {
    input.lines().map(|v| parse_line(v)).collect()
}

fn parse_line(line: &str) -> Vec<Partition> {
    let pattern = regex!(r"^(\w)(\w)(\w)(\w)(\w)(\w)(\w)(\w)(\w)(\w)$");
    let captures = pattern.captures(line).unwrap();
    captures
        .iter()
        .skip(1)
        .map(|cap| parse_partition(cap.unwrap().as_str()))
        .collect()
}

fn parse_partition(part: &str) -> Partition {
    match part {
        "F" => Partition::Front,
        "B" => Partition::Back,
        "L" => Partition::Left,
        "R" => Partition::Right,
        _ => panic!("Invalid partition."),
    }
}

#[aoc(day5, part1)]
fn highest_seat_id(input: &Vec<Vec<Partition>>) -> usize {
    input
        .iter()
        .fold(0, |acc: usize, element: &Vec<Partition>| {
            cmp::max(acc, seat_id(element))
        })
}

#[aoc(day5, part2)]
fn my_seat_id(input: &Vec<Vec<Partition>>) -> usize {
    let mut seat_ids: Vec<usize> = input.iter().map(seat_id).collect();
    seat_ids.sort();

    for (index, id) in seat_ids.iter().enumerate() {
        if seat_ids[index + 1] != id + 1 && seat_ids[index + 1] == id + 2 {
            return id + 1;
        }
    }
    panic!("Could not find id");
}

#[derive(Debug)]
struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn new(min: usize, max: usize) -> Range {
        Range { min, max }
    }
    fn lower_half(&mut self) {
        self.max = self.max - (self.max - self.min) / 2 - 1
    }
    fn upper_half(&mut self) {
        self.min = self.min + (self.max - self.min) / 2 + 1
    }

    fn get_value(self) -> usize {
        assert_eq!(self.min, self.max);
        self.min
    }
}

fn seat_id(partitions: &Vec<Partition>) -> usize {
    let (row, column) = calc_position(partitions);
    row * 8 + column
}

fn calc_position(partitions: &Vec<Partition>) -> (usize, usize) {
    let mut row = Range::new(0, 127);
    let mut column = Range::new(0, 7);

    for partition in partitions {
        match partition {
            Partition::Front => row.lower_half(),
            Partition::Back => row.upper_half(),
            Partition::Right => column.upper_half(),
            Partition::Left => column.lower_half(),
        }
    }
    (row.get_value(), column.get_value())
}
#[cfg(test)]
#[test]
fn test_calc_position() {
    let partitions = vec![
        Partition::Front,
        Partition::Back,
        Partition::Front,
        Partition::Back,
        Partition::Back,
        Partition::Front,
        Partition::Front,
        Partition::Right,
        Partition::Left,
        Partition::Right,
    ];

    let (row, column) = calc_position(&partitions);
    assert_eq!(row, 44);
    assert_eq!(column, 5);
}
