use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let passports = read_input();
    let answer = count_valid_passports(&passports);

    println!("{}", answer)
}

pub fn part_two() {}

struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    pub fn from_lines(lines: &Vec<String>) -> Passport {
        let mut birth_year: Option<String> = None;
        let mut issue_year: Option<String> = None;
        let mut expiration_year: Option<String> = None;
        let mut height: Option<String> = None;
        let mut hair_color: Option<String> = None;
        let mut eye_color: Option<String> = None;
        let mut passport_id: Option<String> = None;
        let mut country_id: Option<String> = None;

        let parts: Vec<String> = lines
            .iter()
            .flat_map(|l| l.split(" "))
            .map(|s| s.to_string())
            .collect();

        for part in parts.iter() {
            let key_value: Vec<&str> = part.split(":").collect();
            assert!(key_value.len() == 2);

            let key = key_value[0];
            let value = key_value[1];

            match key {
                "byr" => birth_year = Some(value.to_string()),
                "iyr" => issue_year = Some(value.to_string()),
                "eyr" => expiration_year = Some(value.to_string()),
                "hgt" => height = Some(value.to_string()),
                "hcl" => hair_color = Some(value.to_string()),
                "ecl" => eye_color = Some(value.to_string()),
                "pid" => passport_id = Some(value.to_string()),
                "cid" => country_id = Some(value.to_string()),
                _ => panic!(),
            }
        }

        Passport {
            birth_year,
            issue_year,
            expiration_year,
            height,
            hair_color,
            eye_color,
            passport_id,
            country_id,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }
}

fn read_input() -> Vec<Passport> {
    let password_strings: Vec<Vec<String>> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("\n")
        .split("\n\n")
        .map(|lines| lines.split("\n").map(|l| l.to_string()).collect())
        .collect();

    password_strings
        .iter()
        .map(Passport::from_lines)
        .collect::<Vec<Passport>>()
}

fn count_valid_passports(passports: &Vec<Passport>) -> i32 {
    passports.iter().filter(|p| p.is_valid()).count() as i32
}
