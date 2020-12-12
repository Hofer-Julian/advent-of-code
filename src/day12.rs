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
fn manhattan_distance(input: &[Instruction]) -> i32 {
    let position = get_position(input);
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
    fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
    fn change_direction(&mut self, instruction: &Instruction) {
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
        match self {
            Coordinates { x: 0, y: -1 } => self.set(-1, 0),
            Coordinates { x: -1, y: 0 } => self.set(0, 1),
            Coordinates { x: 0, y: 1 } => self.set(1, 0),
            Coordinates { x: 1, y: 0 } => self.set(0, -1),
            _ => panic!("Invalid coordinates {:?}", self),
        }
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

fn get_position(input: &[Instruction]) -> Coordinates {
    let mut position = Coordinates::new(0, 0);
    let mut direction = Coordinates::new(-1, 0);
    for instruction in input {
        match instruction.operation {
            Operation::North => position.y += 1,
            Operation::South => position.y -= 1,
            Operation::East => position.x += 1,
            Operation::West => position.x -= 1,
            Operation::Forward => position += direction * instruction.argument,
            // right or left
            _ => direction.change_direction(instruction),
        }
    }
    position
}

#[cfg(test)]
#[test]
fn test_part_1_acc_of_fixed_executions() {
    let input = "\
F10
N3
F7
R90
F11";

    let instructions = parse_input_day12(input);
    let expected = Coordinates::new(17, 8);
    assert_eq!(expected, get_position(&instructions));
}
