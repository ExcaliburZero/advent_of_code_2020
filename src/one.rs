use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let mut numbers = read_input();
    let answer = multiply_2020_addends(&mut numbers);

    println!("{}", answer);
}

pub fn part_two() {
}

fn read_input() -> Vec<i32> {
    let stdin = io::stdin();

    let mut numbers: Vec<i32> = Vec::new();
    for line in stdin.lock().lines() {
        let number: i32 = line.unwrap().parse().unwrap();

        numbers.push(number);
    }

    numbers
}

fn multiply_2020_addends(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();

    for num in numbers.iter() {
        let target = 2020 - *num;

        if numbers.binary_search(&target).is_ok() {
            return *num * target;
        }
    }

    panic!("No valid answer.");
}