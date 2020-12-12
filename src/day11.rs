use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn parse_input_day11(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day11, part1)]
fn day11_part1(input: &Vec<Vec<char>>) -> usize {
    final_number_occupied_seats(input, &number_adjacent_occupied_seats, 4)
}

fn final_number_occupied_seats(
    input: &Vec<Vec<char>>,
    function: &dyn Fn(&Vec<Vec<char>>, usize, usize) -> i32,
    acceptable_seats: i32,
) -> usize {
    let mut old_state = input.clone();
    loop {
        let state = state_change(&old_state, function, acceptable_seats);
        if state == old_state {
            break;
        }
        old_state = state;
    }
    old_state
        .iter()
        .flatten()
        .filter(|seat| **seat == '#')
        .count()
}

fn state_change(
    input: &Vec<Vec<char>>,
    adjacent_seats_function: &dyn Fn(&Vec<Vec<char>>, usize, usize) -> i32,
    acceptable_seats: i32,
) -> Vec<Vec<char>> {
    let mut new_state = input.clone();
    for (column_pos, row) in input.iter().enumerate() {
        for (row_pos, element) in row.iter().enumerate() {
            match element {
                'L' => {
                    if adjacent_seats_function(input, row_pos, column_pos) == 0 {
                        new_state[column_pos][row_pos] = '#';
                    }
                }
                '#' => {
                    if adjacent_seats_function(input, row_pos, column_pos) >= acceptable_seats {
                        new_state[column_pos][row_pos] = 'L';
                    }
                }
                _ => continue,
            }
        }
    }
    new_state
}

fn number_adjacent_occupied_seats(
    input: &Vec<Vec<char>>,
    row_pos: usize,
    column_pos: usize,
) -> i32 {
    let row_lower = std::cmp::max(1, row_pos) - 1;
    let row_upper = std::cmp::min(input[0].len() - 1, row_pos + 1);
    let column_lower = std::cmp::max(1, column_pos) - 1;
    let column_upper = std::cmp::min(input.len() - 1, column_pos + 1);
    let mut number_of_adjacent_seats = 0;

    for column in column_lower..=column_upper {
        for row in row_lower..=row_upper {
            if column_pos == column && row_pos == row {
                continue;
            }
            if input[column][row] == '#' {
                number_of_adjacent_seats += 1;
            }
        }
    }
    number_of_adjacent_seats
}

#[cfg(test)]
#[test]
fn test_part_1_state_change() {
    let input = "\
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";
    let instructions = parse_input_day11(input);

    let state_2_str = "\
#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##";
    let state_2 = parse_input_day11(state_2_str);
    assert_eq!(
        state_2,
        state_change(&instructions, &number_adjacent_occupied_seats, 4)
    );

    let state_3_str = "\
#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##";
    let state_3 = parse_input_day11(state_3_str);
    assert_eq!(
        state_3,
        state_change(&state_2, &number_adjacent_occupied_seats, 4)
    );
}

#[cfg(test)]
#[test]
fn test_part_1_final_state() {
    let input = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    let instructions = parse_input_day11(input);
    assert_eq!(37, day11_part1(&instructions));
}
