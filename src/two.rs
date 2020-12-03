
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let cases = read_input();
    let answer = count_invalid_passwords_count(&cases);

    println!("{}", answer);
}

pub fn part_two() {
    let cases = read_input();
    let answer = count_invalid_passwords_positions(&cases);

    println!("{}", answer);
}

pub struct PasswordRule {
    min: i32,
    max: i32,
    letter: char,
}

impl PasswordRule {
    pub fn parse_rule_and_password(line: &str) -> (PasswordRule, String) {
        let parts: Vec<&str> = line.split(' ').collect();

        assert!(parts.len() == 3);

        let min_max: Vec<&str> = parts[0].split('-').collect();
        let letter = parts[1].chars().next().unwrap();
        let password = parts[2];

        let min: i32 = min_max[0].parse().unwrap();
        let max: i32 = min_max[1].parse().unwrap();

        let rule = PasswordRule {
            min, max, letter
        };

        (rule, password.to_string())
    }

    pub fn validate_count(&self, password: &str) -> bool {
        let letter_count = password.chars()
            .filter(|l| *l == self.letter)
            .count() as i32;

        self.min <= letter_count && letter_count <= self.max
    }

    pub fn validate_positions(&self, password: &str) -> bool {
        let first_matches = password.chars().nth((self.min - 1) as usize).unwrap() == self.letter;
        let second_matches = password.chars().nth((self.max - 1) as usize).unwrap() == self.letter;

        first_matches ^ second_matches
    }
}

fn read_input() -> Vec<(PasswordRule, String)> {
    let stdin = io::stdin();

    let mut rules_and_passwords: Vec<(PasswordRule, String)> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let rule_and_password = PasswordRule::parse_rule_and_password(&line);

        rules_and_passwords.push(rule_and_password);
    }

    rules_and_passwords
}

fn count_invalid_passwords_count(rules_and_passwords: &Vec<(PasswordRule, String)>) -> i32 {
    rules_and_passwords.iter()
        .filter(|rule_and_password| {
            let (rule, password) = rule_and_password;

            rule.validate_count(password)
        }).count() as i32
}

fn count_invalid_passwords_positions(rules_and_passwords: &Vec<(PasswordRule, String)>) -> i32 {
    rules_and_passwords.iter()
        .filter(|rule_and_password| {
            let (rule, password) = rule_and_password;

            rule.validate_positions(password)
        }).count() as i32
}