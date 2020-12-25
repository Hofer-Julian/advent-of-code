use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day13, part2)]
fn parse_input_day13_part2(input: &str) -> Vec<Option<usize>> {
    input
        .lines()
        .skip(1)
        .flat_map(|ids| ids.split(","))
        .map(|id| id.parse().ok())
        .collect()
}

/// This algorithms assumes a vector of coprimes
/// which happens to be the case
#[aoc(day13, part2)]
fn get_answer_day13_part2(input: &Vec<Option<usize>>) -> usize {
    let mut final_id = 0;
    let mut prod = 1;
    for (index, id) in input.iter().enumerate() {
        if let Some(id) = id {
            while (final_id + index) % id != 0 {
                final_id += prod;
            }
            prod *= id;
        }
    }
    final_id
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
