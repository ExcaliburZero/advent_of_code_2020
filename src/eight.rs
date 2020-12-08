use std::collections::BTreeSet;
use std::io::{self, BufRead};

pub fn part_one() {
    let program = read_input(io::stdin().lock());
    let answer = program
        .run_until_inifnite_loop(&ProgramState {
            current_instruction: 0,
            accumulator_value: 0,
        })
        .accumulator_value;

    println!("{}", answer)
}

pub fn part_two() {}

struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn from_lines(lines: &Vec<String>) -> Program {
        Program {
            instructions: lines.iter().map(|l| Instruction::from_str(l)).collect(),
        }
    }

    fn run_until_inifnite_loop(&self, intial_state: &ProgramState) -> ProgramState {
        let mut state = intial_state.clone();

        let mut visited_lines: BTreeSet<usize> = BTreeSet::new();
        loop {
            if visited_lines.contains(&state.current_instruction) {
                break;
            }
            visited_lines.insert(state.current_instruction);

            state.execute(&self.instructions[state.current_instruction]);
        }

        state.clone()
    }
}

#[derive(Clone)]
struct ProgramState {
    current_instruction: usize,
    accumulator_value: i32,
}

impl ProgramState {
    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Nop() => self.current_instruction += 1,
            Instruction::Acc(change) => {
                self.current_instruction += 1;
                self.accumulator_value += change;
            }
            Instruction::Jmp(offset) => {
                self.current_instruction = ((self.current_instruction as i32) + offset) as usize;
            }
        }
    }
}

enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(),
}

impl Instruction {
    fn from_str(instruction_str: &str) -> Instruction {
        let parts: Vec<&str> = instruction_str.split(" ").collect();

        assert!(parts.len() == 2);

        match parts[0] {
            "nop" => Instruction::Nop(),
            "acc" => Instruction::Acc(parts[1].parse().unwrap()),
            "jmp" => Instruction::Jmp(parts[1].parse().unwrap()),
            _ => panic!(),
        }
    }
}

fn read_input<R>(reader: R) -> Program
where
    R: BufRead,
{
    Program::from_lines(&reader.lines().map(|l| l.unwrap().to_string()).collect())
}
