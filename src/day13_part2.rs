use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day13, part2)]
fn parse_input_day13_part2(input: &str) -> Vec<Option<i32>> {
    input
        .lines()
        .skip(1)
        .flat_map(|ids| ids.split(","))
        .map(|id| id.parse().ok())
        .collect()
}

#[aoc(day13, part2)]
fn get_answer_day13_part2(input: &[Option<i32>]) -> i32 {
    todo!()
}

#[cfg(test)]
#[test]
fn test_get_answer_day13_part2() {
    let input = "\
939
7,13,x,x,59,x,31,19";

    let instructions = parse_input_day13_part2(input);
    assert_eq!(1068781, get_answer_day13_part2(&instructions));
}
