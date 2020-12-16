use std::collections::BTreeMap;
use std::io::{self, BufRead};

pub fn part_one() {
    let starting_numbers = read_input(io::stdin().lock());
    let answer = get_nth_number(&starting_numbers, 2020);

    println!("{}", answer)
}

pub fn part_two() {}

fn read_input<R>(reader: R) -> Vec<u64>
where
    R: BufRead,
{
    reader
        .lines()
        .map(|l| l.unwrap())
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

fn get_nth_number(starting_numbers: &[u64], n: u64) -> u64 {
    let mut remaining_starting_numbers: Vec<u64> = starting_numbers.to_owned();
    remaining_starting_numbers.reverse();

    let mut nums_last_turn: BTreeMap<u64, u64> = BTreeMap::new();
    let mut prev_num: Option<(Option<u64>, u64)> = None;
    for i in 1..=n {
        let num = if !remaining_starting_numbers.is_empty() {
            remaining_starting_numbers.pop().unwrap()
        } else {
            prev_num
                .unwrap()
                .0
                .map(|prev_i| (i - 1) - prev_i)
                .unwrap_or(0)
        };

        let prev_turn_called = nums_last_turn.get(&num).copied();
        nums_last_turn.insert(num, i);

        prev_num = Some((prev_turn_called, num));
    }

    prev_num.unwrap().1
}
