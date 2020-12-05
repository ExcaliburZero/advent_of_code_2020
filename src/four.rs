extern crate regex;

use std::io::{self, BufRead};

pub fn part_one() {
    let passports = read_input(io::stdin().lock());
    let answer = count_passports_with_required_fields(&passports);

    println!("{}", answer)
}

pub fn part_two() {
    let passports = read_input(io::stdin().lock());
    let answer = count_valid_passports(&passports);

    println!("{}", answer)
}

struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    _country_id: Option<String>,
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
            _country_id: country_id,
        }
    }

    pub fn has_required_fields(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    pub fn is_valid(&self) -> bool {
        if !self.has_required_fields() {
            return false;
        }

        self.birth_year_is_valid()
            && self.issue_year_is_valid()
            && self.expiration_year_is_valid()
            && self.height_is_valid()
            && self.hair_color_is_valid()
            && self.eye_color_is_valid()
            && self.passport_id_is_valid()
    }

    pub fn birth_year_is_valid(&self) -> bool {
        if self.birth_year.as_ref().unwrap().len() != 4 {
            return false;
        }

        let year = self.birth_year.as_ref().unwrap().parse::<i32>().unwrap();

        year >= 1920 && year <= 2002
    }

    pub fn issue_year_is_valid(&self) -> bool {
        if self.issue_year.as_ref().unwrap().len() != 4 {
            return false;
        }

        let year = self.issue_year.as_ref().unwrap().parse::<i32>().unwrap();

        year >= 2010 && year <= 2020
    }

    pub fn expiration_year_is_valid(&self) -> bool {
        if self.expiration_year.as_ref().unwrap().len() != 4 {
            return false;
        }

        let year = self
            .expiration_year
            .as_ref()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        year >= 2020 && year <= 2030
    }

    pub fn height_is_valid(&self) -> bool {
        let height_str = self.height.as_ref().unwrap();

        let height_regex = self::regex::Regex::new(r"(\d+)(cm|in)$").unwrap();

        match height_regex.captures(height_str) {
            None => false,
            Some(capture) => {
                let value: i32 = capture.get(1).unwrap().as_str().parse().unwrap();
                let unit = capture.get(2).unwrap().as_str();

                assert!(unit == "cm" || unit == "in");

                if unit == "cm" {
                    150 <= value && value <= 193
                } else {
                    59 <= value && value <= 76
                }
            }
        }
    }

    pub fn hair_color_is_valid(&self) -> bool {
        let hair_color_str = self.hair_color.as_ref().unwrap();
        let hair_color_regex = self::regex::Regex::new(r"^#[0-9,a-f]{6}$").unwrap();

        hair_color_regex.is_match(hair_color_str)
    }

    pub fn eye_color_is_valid(&self) -> bool {
        let eye_color_str = self.eye_color.as_ref().unwrap();
        let eye_color_regex = self::regex::Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();

        eye_color_regex.is_match(eye_color_str)
    }

    pub fn passport_id_is_valid(&self) -> bool {
        let passport_id_str = self.passport_id.as_ref().unwrap();
        let passport_id_regex = self::regex::Regex::new(r"^[0-9]{9}$").unwrap();

        passport_id_regex.is_match(passport_id_str)
    }
}

fn read_input<R>(reader: R) -> Vec<Passport>
where
    R: BufRead,
{
    let password_strings: Vec<Vec<String>> = reader
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

fn count_passports_with_required_fields(passports: &Vec<Passport>) -> i32 {
    passports.iter().filter(|p| p.has_required_fields()).count() as i32
}

fn count_valid_passports(passports: &Vec<Passport>) -> i32 {
    passports.iter().filter(|p| p.is_valid()).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_invalid_passport_01() {
        let input = b"eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n";
        let passports = read_input(&input[..]);

        let expected = 0;
        let actual = count_valid_passports(&passports);

        assert_eq!(expected, actual)
    }

    #[test]
    fn validate_invalid_passport_02() {
        let input = b"iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946\n";
        let passports = read_input(&input[..]);

        let expected = 0;
        let actual = count_valid_passports(&passports);

        assert_eq!(expected, actual)
    }

    #[test]
    fn validate_invalid_passport_03() {
        let input =
            b"hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n";
        let passports = read_input(&input[..]);

        let expected = 0;
        let actual = count_valid_passports(&passports);

        assert_eq!(expected, actual)
    }

    #[test]
    fn validate_invalid_passport_04() {
        let input = b"hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007\n";
        let passports = read_input(&input[..]);

        let expected = 0;
        let actual = count_valid_passports(&passports);

        assert_eq!(expected, actual)
    }

    #[test]
    fn validate_valid_passport_01() {
        let input = b"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f\n";
        let passports = read_input(&input[..]);

        let expected = 1;
        let actual = count_valid_passports(&passports);

        assert_eq!(expected, actual)
    }

    #[test]
    fn validate_valid_passport_02() {
        let input =
            b"eyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n";
        let passports = read_input(&input[..]);

        let expected = 1;
        let actual = count_valid_passports(&passports);

        assert_eq!(expected, actual)
    }

    #[test]
    fn validate_valid_passport_03() {
        let input =
            b"hcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022\n";
        let passports = read_input(&input[..]);

        let expected = 1;
        let actual = count_valid_passports(&passports);

        assert_eq!(expected, actual)
    }

    #[test]
    fn validate_valid_passport_04() {
        let input = b"iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719\n";
        let passports = read_input(&input[..]);

        let expected = 1;
        let actual = count_valid_passports(&passports);

        assert_eq!(expected, actual)
    }
}
