use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse_input_day9(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(str::parse)
        .map(|result| result.expect("This entry is not an integer."))
        .collect()
}

#[aoc(day9, part1)]
fn first_number_no_sum_specific(input: &[usize]) -> usize {
    first_number_no_sum_general(input, 25)
}

fn first_number_no_sum_general(input: &[usize], number: usize) -> usize {
    let total_size = input.len();
    'outer: for index_number in number..total_size {
        for index_first in (index_number - number)..index_number {
            for index_second in index_first..index_number {
                if input[index_number] == input[index_first] + input[index_second] {
                    continue 'outer;
                }
            }
        }
        return input[index_number];
    }
    panic!(
        "Could not find number which is not the sum of two of the {} numbers before it",
        number
    )
}

#[test]
fn test_part_1_first_number_without_property() {
    let input = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    let instructions = parse_input_day9(input);
    assert_eq!(127, first_number_no_sum_general(&instructions, 5));
}

#[aoc(day9, part2)]
fn contiguous_set_specific(input: &[usize]) -> usize {
    contiguous_set_general(input, 36845998)
}

fn contiguous_set_general(input: &[usize], number: usize) -> usize {
    let total_size = input.len();
    for index_lower in 0..total_size {
        let mut sum = 0;
        for index_upper in index_lower..total_size {
            sum += input[index_upper];
            if sum == number {
                let range = input[index_lower..=index_upper].to_vec();
                return range.iter().min().unwrap() + range.iter().max().unwrap();
            }
        }
    }
    panic!(
        "Could not find a contiguous set of at least two numbers which sum to {}",
        number
    )
}

#[test]
fn test_part_2_contiguous_set() {
    let input = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    let instructions = parse_input_day9(input);
    assert_eq!(62, contiguous_set_general(&instructions, 127));
}
