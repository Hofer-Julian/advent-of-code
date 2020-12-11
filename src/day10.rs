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
fn get_differences_multiplied(input: &[usize]) -> usize {
    let mut sorted_input = input.clone().to_vec();
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
#[cfg(test)]
#[test]
fn test_part_1_example_1_get_differences_multiplied() {
    let input = "\
16
10
15
5
1
11
7
19
6
12
4";

    let instructions = parse_input_day10(input);
    assert_eq!(35, get_differences_multiplied(&instructions));
}
#[cfg(test)]
#[test]
fn test_part_1_example_2_get_differences_multiplied() {
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
fn get_total_number_of_arrangments(input: &[usize]) -> usize {
    let mut sorted_input = input.clone().to_vec();
    sorted_input.push(0);
    sorted_input.sort();

    get_number_of_arrangments(&sorted_input)
}

fn get_number_of_arrangments(input: &[usize]) -> usize {
    let mut number_of_arrangments = 0;
    if input.len() == 1 {
        return 1;
    }
    for difference in 1..=3 {
        let limit = std::cmp::min(difference, input.len() - 1);
        for index in 1..=limit {
            if input[0] + difference == input[index] {
                number_of_arrangments += get_number_of_arrangments(&input[index..])
            }
        }
    }
    number_of_arrangments
}

// fn get_number_of_arrangments(input: &[usize]) -> usize {
//     let mut number_of_arrangments = 1;
//     let length = input.len();
//     for index in 1..length {
//         let mut possibilites = 0;
//         for difference in 1..=3 {
//             let limit = std::cmp::min(difference, input.len() - 1 - index);
//             for index_difference in 1..=limit {
//                 if input[index] + difference == input[index + index_difference] {
//                     possibilites += 1;
//                 }
//             }
//         }
//         number_of_arrangments *= possibilites;
//         dbg!(number_of_arrangments);
//     }

//     number_of_arrangments
// }

#[cfg(test)]
#[test]
fn test_part_2_example_1_get_total_number_of_arrangments() {
    let input = "\
16
10
15
5
1
11
7
19
6
12
4";

    let instructions = parse_input_day10(input);
    assert_eq!(8, get_total_number_of_arrangments(&instructions));
}
#[cfg(test)]
#[test]
fn test_part_2_example_2_get_total_number_of_arrangments() {
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
    assert_eq!(19208, get_total_number_of_arrangments(&instructions));
}
