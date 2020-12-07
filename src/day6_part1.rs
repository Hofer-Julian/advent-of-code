use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day6, part1)]
fn parse_input_day6_part1(input: &str) -> Vec<HashSet<char>> {
    input
        .split("\n\n")
        .map(|group_str| parse_group_part1(group_str))
        .collect()
}
fn parse_group_part1(group_str: &str) -> HashSet<char> {
    group_str.chars().filter(|c| *c != '\n').collect()
}

#[aoc(day6, part1)]
fn number_of_inclusive_yes(input: &Vec<HashSet<char>>) -> usize {
    input.iter().fold(0, |acc, set| acc + set.len())
}

#[test]
fn test_1() {
    let input = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

    let groups: Vec<HashSet<char>> = input
        .split("\n\n")
        .map(|group_str| parse_group_part1(group_str))
        .collect();

    assert_eq!(groups[0].len(), 3);
    assert_eq!(groups[1].len(), 3);
    assert_eq!(groups[2].len(), 3);
    assert_eq!(groups[3].len(), 1);
    assert_eq!(groups[4].len(), 1);
}
