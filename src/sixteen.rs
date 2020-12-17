extern crate regex;

use std::io::{self, BufRead};

pub fn part_one() {
    let (fields, _, nearby_tickets) = read_input(io::stdin().lock());
    let answer = get_sum_invalid_numbers(&fields, &nearby_tickets);

    println!("{}", answer);
}

pub fn part_two() {}

#[derive(Debug, PartialEq)]
struct Field {
    name: String,
    range_1: NumberRange,
    range_2: NumberRange,
}

impl Field {
    fn from_str(field_str: &str) -> Option<Field> {
        let field_regex =
            self::regex::Regex::new(r"^([a-z, ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

        match field_regex.captures(field_str) {
            None => None,
            Some(capture) => {
                let name = capture.get(1).unwrap().as_str().to_string();

                let range_1_lower = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
                let range_1_upper = capture.get(3).unwrap().as_str().parse::<u32>().unwrap();

                let range_2_lower = capture.get(4).unwrap().as_str().parse::<u32>().unwrap();
                let range_2_upper = capture.get(5).unwrap().as_str().parse::<u32>().unwrap();

                let range_1 = NumberRange::new(range_1_lower, range_1_upper);
                let range_2 = NumberRange::new(range_2_lower, range_2_upper);

                Some(Field {
                    name,
                    range_1,
                    range_2,
                })
            }
        }
    }

    fn valid_for(&self, num: u32) -> bool {
        self.range_1.contains(num) || self.range_2.contains(num)
    }
}

#[derive(Debug, PartialEq)]
struct NumberRange {
    lower: u32,
    upper: u32,
}

impl NumberRange {
    fn new(lower: u32, upper: u32) -> NumberRange {
        NumberRange { lower, upper }
    }

    fn contains(&self, num: u32) -> bool {
        (self.lower..=self.upper).contains(&num)
    }
}

#[derive(Debug, PartialEq)]
struct Ticket {
    values: Vec<u32>,
}

impl Ticket {
    fn from_str(ticket_str: &str) -> Ticket {
        let values: Vec<u32> = ticket_str
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        Ticket { values }
    }

    fn get_invalid_numbers(&self, fields: &[Field]) -> Vec<u32> {
        self.values
            .iter()
            .copied()
            .filter(|n| !fields.iter().any(|field| field.valid_for(*n)))
            .collect()
    }
}

fn read_input<R>(reader: R) -> (Vec<Field>, Ticket, Vec<Ticket>)
where
    R: BufRead,
{
    let parts: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("\n")
        .split("\n\n")
        .map(|p| p.to_string())
        .collect();

    let fields: Vec<Field> = parts[0]
        .lines()
        .map(|l| Field::from_str(l).unwrap())
        .collect();

    let my_ticket = Ticket::from_str(parts[1].lines().nth(1).unwrap());

    let nearby_tickets: Vec<Ticket> = parts[2].lines().skip(1).map(Ticket::from_str).collect();

    (fields, my_ticket, nearby_tickets)
}

fn get_sum_invalid_numbers(fields: &[Field], nearby_tickets: &[Ticket]) -> u32 {
    nearby_tickets
        .iter()
        .flat_map(|t| t.get_invalid_numbers(fields))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_from_str() {
        let field_str = "departure location: 31-538 or 546-960";

        let expected = Some(Field {
            name: "departure location".to_string(),
            range_1: NumberRange {
                lower: 31,
                upper: 538,
            },
            range_2: NumberRange {
                lower: 546,
                upper: 960,
            },
        });
        let actual = Field::from_str(field_str);

        assert_eq!(expected, actual);
    }
}
