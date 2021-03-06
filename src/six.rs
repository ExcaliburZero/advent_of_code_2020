use std::collections::BTreeSet;
use std::io::{self, BufRead};

pub fn part_one() {
    let groups = read_input(io::stdin().lock());
    let answer = sum_num_questions_any_anwered_yes(&groups);

    println!("{}", answer)
}

pub fn part_two() {
    let groups = read_input(io::stdin().lock());
    let answer = sum_num_questions_all_anwered_yes(&groups);

    println!("{}", answer)
}

struct GroupAnswers {
    member_answers: Vec<BTreeSet<char>>,
}

impl GroupAnswers {
    fn from_lines(lines: &[String]) -> GroupAnswers {
        let mut member_answers: Vec<BTreeSet<char>> = vec![];
        for line in lines.iter() {
            let mut answers: BTreeSet<char> = BTreeSet::new();
            for c in line.chars() {
                answers.insert(c);
            }

            member_answers.push(answers);
        }

        GroupAnswers { member_answers }
    }

    fn num_questions_any_anwered_yes(&self) -> i32 {
        self.member_answers
            .iter()
            .fold(BTreeSet::new(), |mut a, b| {
                b.iter().for_each(|c| {
                    a.insert(c);
                });
                a
            })
            .len() as i32
    }

    fn num_questions_all_anwered_yes(&self) -> i32 {
        if self.member_answers.is_empty() {
            0
        } else {
            self.member_answers[0]
                .iter()
                .filter(|a| self.member_answers.iter().all(|ma| ma.contains(a)))
                .count() as i32
        }
    }
}

fn read_input<R>(reader: R) -> Vec<GroupAnswers>
where
    R: BufRead,
{
    let group_strings: Vec<Vec<String>> = reader
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("\n")
        .split("\n\n")
        .map(|lines| lines.split('\n').map(|l| l.to_string()).collect())
        .collect();

    group_strings
        .iter()
        .map(|lines| GroupAnswers::from_lines(lines))
        .collect::<Vec<GroupAnswers>>()
}

fn sum_num_questions_any_anwered_yes(groups: &[GroupAnswers]) -> i32 {
    groups
        .iter()
        .map(GroupAnswers::num_questions_any_anwered_yes)
        .sum()
}

fn sum_num_questions_all_anwered_yes(groups: &[GroupAnswers]) -> i32 {
    groups
        .iter()
        .map(GroupAnswers::num_questions_all_anwered_yes)
        .sum()
}
