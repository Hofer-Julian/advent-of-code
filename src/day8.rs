use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Acc,
    Jmp,
    Nop,
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

#[aoc_generator(day8)]
fn parse_input_day8(input: &str) -> Vec<Instruction> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Instruction {
    let line_vec: Vec<_> = line.split(" ").collect();
    let operation = match line_vec[0] {
        "acc" => Operation::Acc,
        "jmp" => Operation::Jmp,
        "nop" => Operation::Nop,
        _ => panic!("Invalid operation."),
    };

    let argument: i32 = str::parse(line_vec[1]).expect("Argument has to be an integer.");
    Instruction::new(operation, argument)
}

#[aoc(day8, part1)]
fn acc_of_unique_executions(input: &Vec<Instruction>) -> i32 {
    let mut index: i32 = 0;
    let mut acc = 0;
    let mut visited_indices = Vec::new();

    while !visited_indices.contains(&index) {
        visited_indices.push(index);
        let Instruction {
            operation,
            argument,
        } = input[index as usize];

        match operation {
            Operation::Acc => {
                acc += argument;
                index += 1
            }
            Operation::Jmp => index += argument,
            Operation::Nop => index += 1,
        }
    }
    acc
}

#[test]
fn test_part_1_acc_of_unique_executions() {
    let input = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    let hashmap = parse_input_day8(input);
    assert_eq!(5, acc_of_unique_executions(&hashmap));
}
