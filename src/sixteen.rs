extern crate regex;

use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, BufRead};

pub fn part_one() {
    let (fields, _, nearby_tickets) = read_input(io::stdin().lock());
    let answer = get_sum_invalid_numbers(&fields, &nearby_tickets);

    println!("{}", answer);
}

pub fn part_two() {
    let (fields, my_ticket, nearby_tickets) = read_input(io::stdin().lock());
    let answer = get_product_of_my_departure_values(&fields, &my_ticket, &nearby_tickets);

    println!("{}", answer);
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

    fn num_valid_for(&self, numbers: &BTreeSet<u32>) -> usize {
        numbers
            .iter()
            .copied()
            .filter(|n| self.range_1.contains(*n) || self.range_2.contains(*n))
            .count()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

fn find_field_allocation(
    possible_fields: &BTreeMap<usize, Vec<Field>>,
    remaining_indices: &[usize],
    allocated_fields: &[Field],
    allocations: &[(usize, Field)],
) -> Option<Vec<(usize, Field)>> {
    if remaining_indices.is_empty() {
        Some(allocations.to_owned())
    } else {
        for i in remaining_indices.iter() {
            for field in possible_fields.get(i).unwrap() {
                if !allocated_fields.contains(field) {
                    let mut new_remaining_indices = remaining_indices.to_owned();
                    new_remaining_indices.retain(|x| x != i);

                    let mut new_allocated_fields = allocated_fields.to_owned();
                    new_allocated_fields.push(field.clone());

                    let mut new_allocations = allocations.to_owned();
                    new_allocations.push((*i, field.clone()));

                    match find_field_allocation(
                        possible_fields,
                        &new_remaining_indices,
                        &new_allocated_fields,
                        &new_allocations,
                    ) {
                        None => (),
                        Some(a) => return Some(a),
                    }
                }
            }
        }

        None
    }
}

fn find_fields_order(fields: &[Field], tickets: &[&Ticket]) -> Vec<Field> {
    let columns_values: Vec<BTreeSet<u32>> = (0..fields.len())
        .map(|i| tickets.iter().map(|t| t.values[i]).collect())
        .collect();

    let fields_possibilities: Vec<(usize, Vec<Field>)> = columns_values
        .iter()
        .map(|column_values| {
            fields
                .iter()
                .filter(|field| field.num_valid_for(column_values) == column_values.len())
                .cloned()
                .collect()
        })
        .enumerate()
        .collect();

    let mut edges = BTreeMap::new();
    for (i, fields) in fields_possibilities {
        edges.insert(i, fields);
    }

    let mut indices: Vec<usize> = edges.keys().cloned().collect();
    indices.sort_by(|a, b| {
        edges
            .get(a)
            .unwrap()
            .len()
            .cmp(&edges.get(b).unwrap().len())
    });

    let mut fields_with_indexes = find_field_allocation(&edges, &indices, &[], &[]).unwrap();

    fields_with_indexes.sort_by(|a, b| a.0.cmp(&b.0));

    fields_with_indexes
        .iter()
        .map(|(_, field)| field)
        .cloned()
        .collect()
}

fn get_sum_invalid_numbers(fields: &[Field], nearby_tickets: &[Ticket]) -> u32 {
    nearby_tickets
        .iter()
        .flat_map(|t| t.get_invalid_numbers(fields))
        .sum()
}

fn get_product_of_my_departure_values(
    fields: &[Field],
    my_ticket: &Ticket,
    nearby_tickets: &[Ticket],
) -> u64 {
    let valid_nearby_tickets: Vec<&Ticket> = nearby_tickets
        .iter()
        .filter(|ticket| ticket.get_invalid_numbers(fields).is_empty())
        .collect();

    let fields_in_order = find_fields_order(fields, &valid_nearby_tickets);

    fields_in_order
        .iter()
        .enumerate()
        .filter(|(_, field)| field.name.starts_with("departure"))
        .map(|(i, _)| my_ticket.values[i] as u64)
        .product()
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
