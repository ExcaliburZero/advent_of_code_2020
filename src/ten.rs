use std::collections::BTreeSet;
use std::io::{self, BufRead};

pub fn part_one() {
    let adapters = read_input(io::stdin().lock());
    let answer = get_product_of_num_1_diffs_and_num_3_diffs(&adapters);

    println!("{}", answer)
}

pub fn part_two() {}

fn read_input<R>(reader: R) -> BTreeSet<i32>
where
    R: BufRead,
{
    reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

fn get_product_of_num_1_diffs_and_num_3_diffs(adapters: &BTreeSet<i32>) -> i32 {
    let mut adapters_wall_and_device = adapters.clone();
    adapters_wall_and_device.insert(0);
    adapters_wall_and_device.insert(adapters_wall_and_device.iter().max().unwrap() + 3);

    let diffs: Vec<i32> = adapters_wall_and_device
        .iter()
        .zip(adapters_wall_and_device.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect();

    let num_1_diffs = diffs.iter().filter(|a| **a == 1).count() as i32;
    let num_3_diffs = diffs.iter().filter(|a| **a == 3).count() as i32;

    num_1_diffs * num_3_diffs
}
