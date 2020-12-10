use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse_input_day10(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(str::parse)
        .map(|result| result.expect("This entry is not an integer."))
        .collect()
}

#[aoc(day10, part1)]
fn get_differences_multiplied(input: &Vec<usize>) -> usize {
    let mut sorted_input = input.clone();
    sorted_input.push(0);
    sorted_input.sort();

    let length = sorted_input.len();
    let mut diff_one = 0;
    let mut diff_three = 0;

    for index in 1..length {
        if sorted_input[index - 1] + 1 == sorted_input[index] {
            diff_one += 1;
            continue;
        }
        if sorted_input[index - 1] + 2 == sorted_input[index] {
            continue;
        }
        if sorted_input[index - 1] + 3 == sorted_input[index] {
            diff_three += 1;
            continue;
        }
        panic!("Could not find multiplication of number of one jolt and three jolt differences.")
    }
    // Difference between internal adapter and highest-rated adapter is always three
    diff_three += 1;
    diff_one * diff_three
}

#[test]
fn test_part_1_get_differences_multiplied() {
    let input = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    let instructions = parse_input_day10(input);
    assert_eq!(220, get_differences_multiplied(&instructions));
}

#[aoc(day10, part2)]
fn total_number_of_arrangments(input: &Vec<usize>) -> usize {
    unimplemented!();
}
