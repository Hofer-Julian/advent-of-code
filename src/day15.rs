use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
fn parse_input_day15(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

#[aoc(day15, part1)]
fn number_2020th(input: &Vec<usize>) -> usize {
    let start_len = input.len();
    let mut numbers = input.clone();
    for index in start_len..2020 {
        let last_number = numbers[index - 1];
        if let Some(last_index) = numbers[..numbers.len() - 1]
            .iter()
            .rposition(|number| *number == last_number)
        {
            numbers.push(index - last_index - 1);
        } else {
            numbers.push(0);
        }
    }
    numbers[2020 - 1]
}

#[cfg(test)]
#[test]
fn test_part1_example_1() {
    let input = vec![0, 3, 6];
    assert_eq!(436, number_2020th(&input));
}

#[cfg(test)]
#[test]
fn test_part1_example_2() {
    let input = vec![1, 3, 2];
    assert_eq!(1, number_2020th(&input));
}

#[cfg(test)]
#[test]
fn test_part1_example_3() {
    let input = vec![2, 1, 3];
    assert_eq!(10, number_2020th(&input));
}

#[cfg(test)]
#[test]
fn test_part1_example_4() {
    let input = vec![1, 2, 3];
    assert_eq!(27, number_2020th(&input));
}

#[cfg(test)]
#[test]
fn test_part1_example_5() {
    let input = vec![2, 3, 1];
    assert_eq!(78, number_2020th(&input));
}

#[cfg(test)]
#[test]
fn test_part1_example_6() {
    let input = vec![3, 2, 1];
    assert_eq!(438, number_2020th(&input));
}

#[cfg(test)]
#[test]
fn test_part1_example_7() {
    let input = vec![3, 1, 2];
    assert_eq!(1836, number_2020th(&input));
}

#[aoc(day15, part2)]
fn number_30000000th(input: &Vec<usize>) -> usize {
    let start_len = input.len();
    let mut numbers = input.clone();
    for index in start_len..30000000 {
        let last_number = numbers[index - 1];
        if let Some(last_index) = numbers[..numbers.len() - 1]
            .iter()
            .rposition(|number| *number == last_number)
        {
            numbers.push(index - last_index - 1);
        } else {
            numbers.push(0);
        }
    }
    numbers[30000000 - 1]
}

#[cfg(test)]
#[test]
fn test_part2_example_1() {
    let input = vec![0, 3, 6];
    assert_eq!(175594, number_30000000th(&input));
}
