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
fn first_number_without_property(input: &Vec<usize>) -> usize {
    'outer: for (index_number, number) in input.iter().enumerate() {
        if index_number < 25 {
            continue;
        }
        for (index_first, first_addend) in input.iter().enumerate() {
            if index_first >= index_number {
                break;
            }
            for (index_second, second_addend) in input.iter().enumerate() {
                if index_first >= index_second || index_second >= index_number {
                    break;
                }
                if *number == *first_addend + *second_addend {
                    continue 'outer;
                }
            }
        }
        return *number;
    }
    panic!("Could not find number which is not the sum of two of the 25 numbers before it")
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
    assert_eq!(127, first_number_without_property(&instructions));
}
