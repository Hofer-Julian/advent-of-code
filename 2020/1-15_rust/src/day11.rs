use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn parse_input_day11(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day11, part1)]
fn day11_part1(input: &Vec<Vec<char>>) -> usize {
    final_number_occupied_seats(input, &number_adjacent_occupied_seats, 4)
}

#[aoc(day11, part2)]
fn day11_part2(input: &Vec<Vec<char>>) -> usize {
    final_number_occupied_seats(input, &number_distant_occupied_seats, 5)
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
    number_seats_function: &dyn Fn(&Vec<Vec<char>>, usize, usize) -> i32,
    acceptable_seats: i32,
) -> Vec<Vec<char>> {
    let mut new_state = input.clone();
    for (column_pos, row) in input.iter().enumerate() {
        for (row_pos, element) in row.iter().enumerate() {
            match element {
                'L' => {
                    if number_seats_function(input, row_pos, column_pos) == 0 {
                        new_state[column_pos][row_pos] = '#';
                    }
                }
                '#' => {
                    if number_seats_function(input, row_pos, column_pos) >= acceptable_seats {
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
    let mut number_of_occupied_seats = 0;

    for column in column_lower..=column_upper {
        for row in row_lower..=row_upper {
            if column_pos == column && row_pos == row {
                continue;
            }
            if input[column][row] == '#' {
                number_of_occupied_seats += 1;
            }
        }
    }
    number_of_occupied_seats
}

fn number_distant_occupied_seats(input: &Vec<Vec<char>>, row_pos: usize, column_pos: usize) -> i32 {
    let mut number_of_occupied_seats = 0;
    let row_max = input[0].len() - 1;
    let column_max = input.len() - 1;

    // 西
    let row = row_pos;
    let mut column = column_pos;
    loop {
        if column > 0 {
            column -= 1;
        } else {
            break;
        }
        if input[column][row] == '#' {
            number_of_occupied_seats += 1;
            break;
        }
        if input[column][row] == 'L' {
            break;
        }
    }

    // 西北
    let mut row = row_pos;
    let mut column = column_pos;
    loop {
        if row > 0 {
            row -= 1;
        } else {
            break;
        }
        if column > 0 {
            column -= 1;
        } else {
            break;
        }
        if input[column][row] == '#' {
            number_of_occupied_seats += 1;
            break;
        }
        if input[column][row] == 'L' {
            break;
        }
    }

    // 北
    let mut row = row_pos;
    let column = column_pos;
    loop {
        if row > 0 {
            row -= 1;
        } else {
            break;
        }
        if input[column][row] == '#' {
            number_of_occupied_seats += 1;
            break;
        }
        if input[column][row] == 'L' {
            break;
        }
    }

    // 东北
    let mut row = row_pos;
    let mut column = column_pos;
    loop {
        if column < column_max {
            column += 1;
        } else {
            break;
        }
        if row > 0 {
            row -= 1;
        } else {
            break;
        }
        if input[column][row] == '#' {
            number_of_occupied_seats += 1;
            break;
        }
        if input[column][row] == 'L' {
            break;
        }
    }

    // 东
    let row = row_pos;
    let mut column = column_pos;
    loop {
        if column < column_max {
            column += 1;
        } else {
            break;
        }
        if input[column][row] == '#' {
            number_of_occupied_seats += 1;
            break;
        }
        if input[column][row] == 'L' {
            break;
        }
    }

    // 东南
    let mut row = row_pos;
    let mut column = column_pos;
    loop {
        if column < column_max {
            column += 1;
        } else {
            break;
        }
        if row < row_max {
            row += 1;
        } else {
            break;
        }
        if input[column][row] == '#' {
            number_of_occupied_seats += 1;
            break;
        }
        if input[column][row] == 'L' {
            break;
        }
    }

    // 南
    let mut row = row_pos;
    let column = column_pos;
    loop {
        if row < row_max {
            row += 1;
        } else {
            break;
        }
        if input[column][row] == '#' {
            number_of_occupied_seats += 1;
            break;
        }
        if input[column][row] == 'L' {
            break;
        }
    }

    // 西南
    let mut row = row_pos;
    let mut column = column_pos;
    loop {
        if column > 0 {
            column -= 1;
        } else {
            break;
        }
        if row < row_max {
            row += 1;
        } else {
            break;
        }
        if input[column][row] == '#' {
            number_of_occupied_seats += 1;
            break;
        }
        if input[column][row] == 'L' {
            break;
        }
    }

    number_of_occupied_seats
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

#[cfg(test)]
#[test]
fn test_part_2_state_change() {
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

    let state_2_str = "\
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
    let state_2 = parse_input_day11(state_2_str);
    assert_eq!(
        state_2,
        state_change(&instructions, &number_distant_occupied_seats, 5)
    );

    let state_3_str = "\
#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#";
    let state_3 = parse_input_day11(state_3_str);
    assert_eq!(
        state_3,
        state_change(&state_2, &number_distant_occupied_seats, 5)
    );

    let state_4_str = "\
#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#";
    let state_4 = parse_input_day11(state_4_str);
    assert_eq!(
        state_4,
        state_change(&state_3, &number_distant_occupied_seats, 5)
    );
}

#[cfg(test)]
#[test]
fn test_part_2_final_state() {
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
    assert_eq!(26, day11_part2(&instructions));
}
