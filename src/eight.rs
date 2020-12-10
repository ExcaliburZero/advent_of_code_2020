use std::collections::BTreeSet;
use std::io::{self, BufRead};

pub fn part_one() {
    let program = read_input(io::stdin().lock());
    let answer = program
        .run(&ProgramState {
            current_instruction: 0,
            accumulator_value: 0,
        })
        .get_state()
        .accumulator_value;

    println!("{}", answer)
}

pub fn part_two() {
    let program = read_input(io::stdin().lock());
    let answer = get_terminating_accumulator_value_after_fix(&program).unwrap();

    println!("{}", answer)
}

enum ProgramResult {
    InfiniteLoop(ProgramState),
    Terminated(ProgramState),
}

impl ProgramResult {
    fn get_state(&self) -> &ProgramState {
        match self {
            ProgramResult::InfiniteLoop(state) => state,
            ProgramResult::Terminated(state) => state,
        }
    }
}

#[derive(Clone)]
struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn from_lines(lines: &[String]) -> Program {
        Program {
            instructions: lines.iter().map(|l| Instruction::from_str(l)).collect(),
        }
    }

    fn run(&self, intial_state: &ProgramState) -> ProgramResult {
        let mut state = intial_state.clone();

        let mut visited_lines: BTreeSet<usize> = BTreeSet::new();
        loop {
            if visited_lines.contains(&state.current_instruction) {
                return ProgramResult::InfiniteLoop(state);
            } else if state.current_instruction == self.instructions.len() {
                return ProgramResult::Terminated(state);
            }
            visited_lines.insert(state.current_instruction);

            state.execute(&self.instructions[state.current_instruction]);
        }
    }

    fn change_instruction(&mut self, index: usize, fun: &dyn Fn(&Instruction) -> Instruction) {
        self.instructions[index] = fun(&self.instructions[index]);
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
            Instruction::Nop(_) => self.current_instruction += 1,
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

#[derive(Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Instruction {
    fn from_str(instruction_str: &str) -> Instruction {
        let parts: Vec<&str> = instruction_str.split(' ').collect();

        assert!(parts.len() == 2);

        match parts[0] {
            "nop" => Instruction::Nop(parts[1].parse().unwrap()),
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
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    Program::from_lines(&lines)
}

fn get_terminating_accumulator_value_after_fix(program: &Program) -> Option<i32> {
    let nop_and_jmp_indices: Vec<usize> = program
        .instructions
        .iter()
        .enumerate()
        .filter(|(_, inst)| match inst {
            Instruction::Nop(_) => true,
            Instruction::Jmp(_) => true,
            Instruction::Acc(_) => false,
        })
        .map(|(index, _)| index)
        .collect();

    let mut new_program = program.clone();
    for index in nop_and_jmp_indices.iter() {
        new_program.change_instruction(*index, &switch_nop_or_jmp);

        let results = new_program.run(&ProgramState {
            current_instruction: 0,
            accumulator_value: 0,
        });

        match results {
            ProgramResult::InfiniteLoop(_) => (),
            ProgramResult::Terminated(state) => return Some(state.accumulator_value),
        }

        new_program.change_instruction(*index, &switch_nop_or_jmp);
    }

    None
}

fn switch_nop_or_jmp(instruction: &Instruction) -> Instruction {
    match instruction {
        Instruction::Nop(n) => Instruction::Jmp(*n),
        Instruction::Jmp(n) => Instruction::Nop(*n),
        Instruction::Acc(_) => panic!(),
    }
}
