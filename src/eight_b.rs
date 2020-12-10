use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, BufRead};

const EXISTING_EDGE: u32 = 0;
const NEW_EDGE: u32 = 1;

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

struct ControlFlowGraph<'a> {
    edges: BTreeMap<usize, Vec<(u32, usize)>>,
    program: &'a Program,
}

impl ControlFlowGraph<'_> {
    fn from_program(program: &Program) -> ControlFlowGraph {
        let mut edges: BTreeMap<usize, Vec<(u32, usize)>> = BTreeMap::new();
        for index in 0..program.instructions.len() {
            let instruction = program.instructions.get(index).unwrap();
            let next_index = ControlFlowGraph::next_instruction(index, &instruction);

            edges.insert(index, vec![(EXISTING_EDGE, next_index)]);
        }

        ControlFlowGraph { edges, program }
    }

    fn add_nop_jmp_alternatives(&mut self) {
        let nop_and_jmp_indices: Vec<(usize, &Instruction)> = self
            .program
            .instructions
            .iter()
            .enumerate()
            .filter(|(_, inst)| match inst {
                Instruction::Nop(_) => true,
                Instruction::Jmp(_) => true,
                Instruction::Acc(_) => false,
            })
            .collect();

        for (index, instruction) in nop_and_jmp_indices {
            let new_instruction = switch_nop_or_jmp(&instruction);
            let next_index = ControlFlowGraph::next_instruction(index, &new_instruction);

            self.edges
                .get_mut(&index)
                .unwrap()
                .push((NEW_EDGE, next_index));
        }
    }

    fn reverse_edges(&mut self) {
        let mut new_edges: BTreeMap<usize, Vec<(u32, usize)>> = BTreeMap::new();

        for (source_index, destinations) in self.edges.iter() {
            for (weight, destination_index) in destinations {
                if !new_edges.contains_key(destination_index) {
                    new_edges.insert(*destination_index, vec![]);
                }

                new_edges
                    .get_mut(destination_index)
                    .unwrap()
                    .push((*weight, *source_index));
            }
        }

        self.edges = new_edges;
    }

    fn next_instruction(index: usize, instruction: &Instruction) -> usize {
        match instruction {
            Instruction::Nop(_) => index + 1,
            Instruction::Jmp(offset) => ((index as i32) + offset) as usize,
            Instruction::Acc(_) => index + 1,
        }
    }

    /*fn print(&self) {
        for (source, destinations) in self.edges.iter() {
            println!("{}", source);

            for (weight, destination_index) in destinations {
                println!("    {}  {}", weight, destination_index);
            }
        }
    }*/

    fn depth_first_search(
        &self,
        source: usize,
        destination: usize,
        path: &mut Vec<(u32, usize)>,
        path_weight: u32,
        max_path_weight: u32,
    ) -> Option<Vec<(u32, usize)>> {
        if source == destination {
            Some(path.clone())
        } else {
            // Find which of the next nodes the path has not already included (cycles are always infinite)
            let next_nodes: Vec<&(u32, usize)> = self.edges[&source]
                .iter()
                .filter(|(_, i)| !ControlFlowGraph::path_contains_node(path, *i))
                .collect();

            if next_nodes.is_empty() {
                // No next nodes left to try
                None
            } else {
                // Try to form a path to destination with any of the next reachable nodes
                for (weight, next_index) in next_nodes {
                    // Enforce path weight limit
                    if path_weight + weight > max_path_weight {
                        continue;
                    }

                    // Try to form a path to destination using this next node, by using recusion
                    path.push((*weight, *next_index));
                    let new_path_weight = path_weight + weight;
                    let search_results = self.depth_first_search(
                        *next_index,
                        destination,
                        path,
                        new_path_weight,
                        max_path_weight,
                    );
                    match search_results {
                        None => (),
                        Some(path) => return Some(path),
                    };

                    // Search failed, revert path to try next possible node
                    path.pop();
                }

                // No options left, so destination must not be reachable via this path
                None
            }
        }
    }

    fn path_contains_node(path: &[(u32, usize)], index: usize) -> bool {
        path.iter()
            .filter(|(_, i)| *i == index)
            .peekable()
            .peek()
            .is_some()
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
    // Find the instruction to swap by constructing a control flow graph of the program where each
    // edge has a weight of 0. Add new edges for each of the possible jmp and nop instruction
    // swaps, with each new edge having a weight of 1. The reverse the direction of all of the
    // edges in the graph.
    //
    // We can then find the correct instruction to swap by finding a path between the end of the
    // program to the beginning of the program, where that path as a total weight of at most 1.
    //
    // A path weight of 1 proves that one and only one instruction needs to be swapped for the path
    // (when reversed) to be a valid path from the beginning to the end of the program. If for
    // some reason a path of weight 0 can be found, then it means that the program does not need
    // to have any instructions swapped to terminate, but we know this will not be the case with
    // any offical inputs.
    //
    // If we fail to find a path with a weight of 1 or less, then the program would either require
    // more than one instruction swap to terminate or would never be able to terminate with any
    // combination of instruction swaps.
    let mut graph = ControlFlowGraph::from_program(&program);
    graph.add_nop_jmp_alternatives();
    graph.reverse_edges();

    let program_start = 0;
    let program_end = program.instructions.len();
    let path = graph.depth_first_search(program_end, program_start, &mut vec![], 0, 1);

    match path {
        None => None,
        Some(p) => {
            // Find which instruction we need to apply the swap to
            let index_to_change = p
                .iter()
                .filter(|(weight, _)| *weight == NEW_EDGE)
                .map(|(_, index)| index)
                .next()
                .unwrap();

            // Fix the program by swapping the instruction
            let mut fixed_program = program.clone();
            fixed_program.change_instruction(*index_to_change, &switch_nop_or_jmp);

            // Run the fixed program to get the accumulator state at termination
            let result = fixed_program.run(&ProgramState {
                current_instruction: 0,
                accumulator_value: 0,
            });
            match result {
                ProgramResult::InfiniteLoop(_) => panic!(),
                ProgramResult::Terminated(state) => Some(state.accumulator_value),
            }
        }
    }
}

fn switch_nop_or_jmp(instruction: &Instruction) -> Instruction {
    match instruction {
        Instruction::Nop(n) => Instruction::Jmp(*n),
        Instruction::Jmp(n) => Instruction::Nop(*n),
        Instruction::Acc(_) => panic!(),
    }
}
