extern crate regex;

use std::collections::BTreeMap;
use std::io::{self, BufRead};

pub fn part_one() {
    let commands = read_input(io::stdin().lock());
    let answer = get_sum_mem_values_after_execution_v1(&commands);

    println!("{}", answer);
}

pub fn part_two() {
    let commands = read_input(io::stdin().lock());
    let answer = get_sum_mem_values_after_execution_v2(&commands);

    println!("{}", answer);
}

fn read_input<R>(reader: R) -> Vec<Command>
where
    R: BufRead,
{
    reader
        .lines()
        .map(|l| Command::from_str(&l.unwrap()).unwrap())
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MaskBit {
    X,
    Zero,
    One,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Mask {
    bits: [MaskBit; 36],
}

impl Mask {
    fn new_defaults() -> Mask {
        Mask {
            bits: [MaskBit::X; 36],
        }
    }

    fn from_str(mask_str: &str) -> Result<Mask, String> {
        if mask_str.len() != 36 {
            return Err(format!("Invalid mask length: {}", mask_str.len()));
        }

        let mut bits = [MaskBit::X; 36];
        for (i, c) in mask_str.chars().enumerate() {
            match c {
                'X' => (),
                '0' => bits[i] = MaskBit::Zero,
                '1' => bits[i] = MaskBit::One,
                _ => return Err(format!("Invalid mask bit: {}", c)),
            };
        }

        Ok(Mask { bits })
    }

    fn apply_to_v1(&self, value: u64) -> u64 {
        let mut masked_value = value;
        for (i, bit) in self.bits.iter().rev().enumerate() {
            match bit {
                MaskBit::X => (),
                MaskBit::Zero => {
                    let bit_mask: u64 = !(1 << i);
                    masked_value &= bit_mask;
                }
                MaskBit::One => {
                    let bit_mask: u64 = 1 << i;
                    masked_value |= bit_mask;
                }
            };
        }

        masked_value
    }

    fn apply_to_v2(&self, value: u64) -> Vec<u64> {
        let indices_and_bits: Vec<(usize, &MaskBit)> = self.bits.iter().rev().enumerate().collect();

        let one_bits: Vec<usize> = indices_and_bits
            .iter()
            .filter(|(_, b)| **b == MaskBit::One)
            .map(|(i, _)| *i)
            .collect();

        let x_bits: Vec<usize> = indices_and_bits
            .iter()
            .filter(|(_, b)| **b == MaskBit::X)
            .map(|(i, _)| *i)
            .collect();

        let mut masked_value = value;
        for i in one_bits.iter() {
            let bit_mask: u64 = 1 << i;
            masked_value |= bit_mask;
        }

        let mut possibilities: Vec<u64> = vec![];
        Mask::get_possible_values(masked_value, &x_bits, &mut possibilities);

        possibilities
    }

    fn get_possible_values(value: u64, x_bit_indices: &[usize], possibilities: &mut Vec<u64>) {
        if x_bit_indices.is_empty() {
            possibilities.push(value);
            return;
        }

        let i = x_bit_indices[0];

        let zero_mask: u64 = !(1 << i);
        let zero_value = value & zero_mask;
        Mask::get_possible_values(zero_value, &x_bit_indices[1..], possibilities);

        let one_mask: u64 = 1 << i;
        let one_value = value | one_mask;
        Mask::get_possible_values(one_value, &x_bit_indices[1..], possibilities);
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    SetMask(Mask),
    SetValue(usize, u64),
}

impl Command {
    fn from_str(command_str: &str) -> Result<Command, String> {
        let mask_regex = self::regex::Regex::new(r"^mask = ([0,1,X]+)$").unwrap();
        let mem_set_regex = self::regex::Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

        match mask_regex.captures(command_str) {
            Some(capture) => {
                let mask_str = capture.get(1).unwrap().as_str();

                Mask::from_str(mask_str).map(Command::SetMask)
            }
            None => match mem_set_regex.captures(command_str) {
                Some(capture) => {
                    let index: usize = capture.get(1).unwrap().as_str().parse().unwrap();
                    let value: u64 = capture.get(2).unwrap().as_str().parse().unwrap();

                    Ok(Command::SetValue(index, value))
                }
                None => Err(format!("Unrecognized command string: {}", command_str)),
            },
        }
    }
}

struct State {
    mask: Mask,
    memory: BTreeMap<usize, u64>,
}

impl State {
    fn new_defaults() -> State {
        State {
            mask: Mask::new_defaults(),
            memory: BTreeMap::new(),
        }
    }

    fn execute_v1(&mut self, command: &Command) {
        match command {
            Command::SetMask(mask) => self.mask = *mask,
            Command::SetValue(index, value) => {
                let masked_value = self.mask.apply_to_v1(*value);

                self.memory.insert(*index, masked_value);
            }
        }
    }

    fn execute_v2(&mut self, command: &Command) {
        match command {
            Command::SetMask(mask) => self.mask = *mask,
            Command::SetValue(index, value) => {
                let possibile_indices = self.mask.apply_to_v2(*index as u64);

                for i in possibile_indices.iter() {
                    self.memory.insert(*i as usize, *value);
                }
            }
        }
    }

    fn sum_memory_values(&self) -> u64 {
        self.memory.values().sum()
    }
}

fn get_sum_mem_values_after_execution_v1(commands: &[Command]) -> u64 {
    let mut state = State::new_defaults();
    for comm in commands.iter() {
        state.execute_v1(comm);
    }

    state.sum_memory_values()
}

fn get_sum_mem_values_after_execution_v2(commands: &[Command]) -> u64 {
    let mut state = State::new_defaults();
    for comm in commands.iter() {
        state.execute_v2(comm);
    }

    state.sum_memory_values()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mem_set_from_str() {
        let mem_set_str = "mem[8] = 11";

        let expected = Ok(Command::SetValue(8, 11));
        let actual = Command::from_str(mem_set_str);

        assert_eq!(expected, actual);
    }

    #[test]
    fn mask_from_str() {
        let mask_str = "101XX10X1X00001010011011X1XXX1001011";

        let expected = Ok(Mask {
            bits: [
                MaskBit::One,
                MaskBit::Zero,
                MaskBit::One,
                MaskBit::X,
                MaskBit::X,
                MaskBit::One,
                MaskBit::Zero,
                MaskBit::X,
                MaskBit::One,
                MaskBit::X,
                MaskBit::Zero,
                MaskBit::Zero,
                MaskBit::Zero,
                MaskBit::Zero,
                MaskBit::One,
                MaskBit::Zero,
                MaskBit::One,
                MaskBit::Zero,
                MaskBit::Zero,
                MaskBit::One,
                MaskBit::One,
                MaskBit::Zero,
                MaskBit::One,
                MaskBit::One,
                MaskBit::X,
                MaskBit::One,
                MaskBit::X,
                MaskBit::X,
                MaskBit::X,
                MaskBit::One,
                MaskBit::Zero,
                MaskBit::Zero,
                MaskBit::One,
                MaskBit::Zero,
                MaskBit::One,
                MaskBit::One,
            ],
        });
        let actual = Mask::from_str(mask_str);

        assert_eq!(expected, actual);
    }

    #[test]
    fn mask_apply_to_v1() {
        let mask = Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX101").unwrap();
        let input: u64 = 0b1100;

        let expected: u64 = 0b1101;
        let actual = mask.apply_to_v1(input);

        //println!("actual: {:#036b}", actual);

        assert_eq!(expected, actual);
    }

    #[test]
    fn mask_apply_to_v2() {
        let mask = Mask::from_str("000000000000000000000000000000000X1X").unwrap();
        let input: u64 = 0b1000;

        let expected: Vec<u64> = vec![0b1010, 0b1110, 0b1011, 0b1111];
        let actual = mask.apply_to_v2(input);

        assert_eq!(expected, actual);
    }
}
