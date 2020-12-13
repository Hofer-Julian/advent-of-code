use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day13, part1)]
fn parse_input_day13_part1(input: &str) -> (i32, Vec<i32>) {
    let lines: Vec<_> = input.lines().collect();
    let timestamp = lines[0].parse().unwrap();
    let bus_ids = lines[1]
        .split(",")
        .filter_map(|id| id.parse().ok())
        .collect();
    (timestamp, bus_ids)
}

#[aoc(day13, part1)]
fn get_answer_day13_part1(input: &(i32, Vec<i32>)) -> i32 {
    let mut shortest_wait = i32::MAX;
    let mut earliest_bus = None;

    let (timestamp, bus_ids) = input;

    for bus_id in bus_ids {
        let wait = bus_id - timestamp % bus_id;
        if wait < shortest_wait {
            shortest_wait = wait;
            earliest_bus = Some(bus_id)
        }
    }
    let earliest_bus = earliest_bus.expect("Could not find any valid bus");
    earliest_bus * shortest_wait
}

#[cfg(test)]
#[test]
fn test_get_answer_day13_part1() {
    let input = "\
939
7,13,x,x,59,x,31,19";

    let instructions = parse_input_day13_part1(input);
    assert_eq!(295, get_answer_day13_part1(&instructions));
}
