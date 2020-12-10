use std::io::{self, BufRead};

pub fn part_one() {
    let numbers = read_input(io::stdin().lock());
    let answer = get_first_non_prev_sum_number(&numbers);

    println!("{}", answer.unwrap())
}

pub fn part_two() {}

fn read_input<R>(reader: R) -> Vec<i64>
where
    R: BufRead,
{
    reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

fn get_first_non_prev_sum_number(numbers: &Vec<i64>) -> Option<i64> {
    assert!(numbers.len() >= 25);

    for i in 25..numbers.len() {
        let num = numbers[i];

        let mut found = false;
        for j in (i - 25)..i {
            for k in (i - 25)..i {
                if j != k && numbers[j] + numbers[k] == num {
                    found = true;
                    break;
                }
            }

            if found {
                break;
            }
        }

        if !found {
            return Some(num);
        }
    }

    None
}
