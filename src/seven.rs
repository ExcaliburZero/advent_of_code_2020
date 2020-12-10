extern crate regex;

use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, BufRead};

pub fn part_one() {
    let rules = read_input(io::stdin().lock());
    let answer = num_bags_can_contain(&rules, "shiny gold");

    println!("{}", answer);
}

pub fn part_two() {
    let rules = read_input(io::stdin().lock());
    let answer = num_bags_within(&rules, "shiny gold");

    println!("{}", answer);
}

struct BagRules {
    rules: BTreeMap<String, Vec<(String, u32)>>,
}

impl BagRules {
    fn from_lines(lines: &[String]) -> BagRules {
        let rule_regex = self::regex::Regex::new(
            r"^([a-z,\s]+) bags contain (((\d* ([a-z,\s])+ bags?,?\s?)|(no other bags))+).$",
        )
        .unwrap();

        let num_bags_regex = self::regex::Regex::new(r"^(\d+) ([a-z, ]+) bags?$").unwrap();

        //    ^([a-z,\s]+) contain \d* ([a-z,\s])+ bags?.
        //    ^([a-z,\s]+) contain (\d* ([a-z,\s])+ bags?,?\s?)+.
        //    ^([a-z,\s]+) contain no other bags.
        //    ^([a-z,\s]+) contain ((\d* ([a-z,\s])+ bags?,?\s?)|(no other bags))+.

        let mut rules: BTreeMap<String, Vec<(String, u32)>> = BTreeMap::new();
        for line in lines.iter() {
            match rule_regex.captures(line) {
                None => panic!(),
                Some(capture) => {
                    let bag_type = capture.get(1).unwrap().as_str();
                    let contained_str = capture.get(2).unwrap().as_str();

                    let contained_bags = if contained_str == "no other bags" {
                        vec![]
                    } else {
                        contained_str
                            .split(", ")
                            .map(|t| match num_bags_regex.captures(t) {
                                None => panic!(),
                                Some(num_bags_capture) => {
                                    let num_bags: u32 =
                                        num_bags_capture.get(1).unwrap().as_str().parse().unwrap();
                                    let contained_bag_type =
                                        num_bags_capture.get(2).unwrap().as_str();

                                    (contained_bag_type.to_string(), num_bags)
                                }
                            })
                            .collect()
                    };

                    rules.insert(bag_type.to_string(), contained_bags);
                }
            }
        }

        BagRules { rules }
    }

    /*fn print(&self) {
        for (k, v) in self.rules.iter() {
            println!(
                "{} -> {}",
                k,
                v.iter()
                    .fold("".to_string(), |a, b| format!("{}   {} {}", a, b.1, b.0))
            );
        }
    }*/
}

fn read_input<R>(reader: R) -> BagRules
where
    R: BufRead,
{
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    BagRules::from_lines(&lines)
}

fn num_bags_can_contain(rules: &BagRules, desired_bag: &str) -> u32 {
    rules
        .rules
        .keys()
        .filter(|b| {
            let path = path_to(rules, b, desired_bag);

            match path {
                None => false,
                Some(p) => p.len() > 1,
            }
        })
        .count() as u32
}

fn path_to(rules: &BagRules, starting_bag: &str, target_bag: &str) -> Option<Vec<String>> {
    let mut visited_bags: BTreeSet<String> = BTreeSet::new();
    let mut bags_to_visit: BTreeSet<String> = BTreeSet::new();

    let mut path: Vec<String> = vec![];
    bags_to_visit.insert(starting_bag.to_string());
    while !bags_to_visit.is_empty() {
        let current_bag = bags_to_visit.pop_first().unwrap();

        visited_bags.insert(current_bag.clone());
        path.push(current_bag.clone());

        if current_bag == target_bag {
            return Some(path);
        }

        for r in rules.rules.get(&current_bag).unwrap() {
            if !visited_bags.contains(&r.0) {
                bags_to_visit.insert(r.0.clone());
            }
        }

        if bags_to_visit.is_empty() {
            return None;
        }
    }

    panic!()
}

fn num_bags_within(rules: &BagRules, starting_bag: &str) -> u32 {
    rules
        .rules
        .get(starting_bag)
        .unwrap()
        .iter()
        .map(|(b, num)| (num_bags_within(rules, b) + 1) * num)
        .sum()
}
