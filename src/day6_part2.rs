use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day6, part2)]
fn parse_input_day6_part2(input: &str) -> Vec<HashSet<char>> {
    input
        .split("\n\n")
        .map(|group_str| parse_group_part2(group_str))
        .collect()
}

fn parse_group_part2(group_str: &str) -> HashSet<char> {
    let answers: Vec<HashSet<char>> = group_str.lines().map(|str| str.chars().collect()).collect();
    let first_answer = answers[0].clone();
    answers.iter().skip(1).fold(first_answer, |acc, answer| {
        answer.intersection(&acc).map(|c| c.to_owned()).collect()
    })
}

#[aoc(day6, part2)]
fn number_of_exclusive_yes(input: &Vec<HashSet<char>>) -> usize {
    input.iter().fold(0, |acc, set| acc + set.len())
}

#[test]
fn test_2() {
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
        .map(|group_str| parse_group_part2(group_str))
        .collect();

    assert_eq!(groups[0].len(), 3);
    assert_eq!(groups[1].len(), 0);
    assert_eq!(groups[2].len(), 1);
    assert_eq!(groups[3].len(), 1);
    assert_eq!(groups[4].len(), 1);
}
