use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum Operation {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    operation: Operation,
    argument: i32,
}

impl Instruction {
    fn new(operation: Operation, argument: i32) -> Self {
        Self {
            operation,
            argument,
        }
    }
}

#[aoc_generator(day12)]
fn parse_input_day12(input: &str) -> Vec<Instruction> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Instruction {
    let operation = match &line[..1] {
        "N" => Operation::North,
        "S" => Operation::South,
        "E" => Operation::East,
        "W" => Operation::West,
        "L" => Operation::Left,
        "R" => Operation::Right,
        "F" => Operation::Forward,
        _ => panic!("Invalid operation."),
    };

    let argument: i32 = str::parse(&line[1..]).expect("Argument has to be an integer.");
    Instruction::new(operation, argument)
}

#[aoc(day12, part1)]
fn manhattan_distance_part_1(input: &[Instruction]) -> i32 {
    let position = get_position_part_1(input);
    position.manhattan_distance()
}

#[aoc(day12, part2)]
fn manhattan_distance_part_2(input: &[Instruction]) -> i32 {
    let position = get_position_part_2(input);
    position.manhattan_distance()
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
    fn rotate(&mut self, instruction: &Instruction) {
        let number_right_rotations = match instruction.operation {
            Operation::Right => instruction.argument / 90,
            Operation::Left => (360 - instruction.argument) / 90,
            _ => panic!("Invalid operation {:?}", instruction.operation),
        };
        for _ in 0..number_right_rotations {
            self.rotate_right()
        }
    }
    fn rotate_right(&mut self) {
        *self = Coordinates::new(self.y, -self.x);
    }
}
impl std::ops::Add<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn add(self, _rhs: Coordinates) -> Coordinates {
        Coordinates {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl std::ops::Mul<i32> for Coordinates {
    type Output = Coordinates;

    fn mul(self, _rhs: i32) -> Coordinates {
        Coordinates {
            x: self.x * _rhs,
            y: self.y * _rhs,
        }
    }
}
impl std::ops::AddAssign for Coordinates {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

fn get_position_part_1(input: &[Instruction]) -> Coordinates {
    let mut position = Coordinates::new(0, 0);
    let mut direction = Coordinates::new(1, 0);
    for instruction in input {
        match instruction.operation {
            Operation::North => position.y += instruction.argument,
            Operation::South => position.y -= instruction.argument,
            Operation::East => position.x += instruction.argument,
            Operation::West => position.x -= instruction.argument,
            Operation::Forward => position += direction * instruction.argument,
            // right or left
            _ => direction.rotate(instruction),
        }
    }
    position
}

fn get_position_part_2(input: &[Instruction]) -> Coordinates {
    let mut position = Coordinates::new(0, 0);
    let mut waypoint = Coordinates::new(10, 1);
    for instruction in input {
        match instruction.operation {
            Operation::North => waypoint.y += instruction.argument,
            Operation::South => waypoint.y -= instruction.argument,
            Operation::East => waypoint.x += instruction.argument,
            Operation::West => waypoint.x -= instruction.argument,
            Operation::Forward => position += waypoint * instruction.argument,
            // right or left
            _ => waypoint.rotate(instruction),
        }
    }
    position
}

#[cfg(test)]
#[test]
fn test_get_position_part_1() {
    let input = "\
F10
N3
F7
R90
F11";

    let instructions = parse_input_day12(input);
    let expected = Coordinates::new(17, -8);
    assert_eq!(expected, get_position_part_1(&instructions));
}

#[cfg(test)]
#[test]
fn test_get_position_part_2() {
    let input = "\
F10
N3
F7
R90
F11";

    let instructions = parse_input_day12(input);
    let expected = Coordinates::new(214, -72);
    assert_eq!(expected, get_position_part_2(&instructions));
}
