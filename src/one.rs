use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let mut numbers = read_input();
    let answer = multiply_2020_addends(&mut numbers);

    println!("{}", answer);
}

pub fn part_two() {
    let numbers = read_input();
    let answer = multiply_2020_three_addends(&numbers);

    println!("{}", answer);
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
    numbers.sort_unstable();

    for num in numbers.iter() {
        let target = 2020 - *num;

        if numbers.binary_search(&target).is_ok() {
            return *num * target;
        }
    }

    panic!("No valid answer.");
}

fn multiply_2020_three_addends(numbers: &[i32]) -> i32 {
    let size = numbers.len();

    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                if i != j && i != k && j != k {
                    let a = numbers[i];
                    let b = numbers[j];
                    let c = numbers[k];

                    if a + b + c == 2020 {
                        return a * b * c;
                    }
                }
            }
        }
    }

    panic!("No valid answer.");
}
