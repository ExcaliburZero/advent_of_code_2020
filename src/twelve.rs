extern crate regex;

use std::io::{self, BufRead};

pub fn part_one() {
    let actions = read_input(io::stdin().lock());
    let answer = get_manhatten_dist_after_applying_actions(&Turtle::new_default(), &actions);

    println!("{}", answer)
}

pub fn part_two() {}

enum Action {
    North(u32),
    South(u32),
    East(u32),
    West(u32),
    Left(u32),
    Right(u32),
    Foreward(u32),
}

impl Action {
    fn from_str(action_str: &str) -> Option<Action> {
        let action_regex = self::regex::Regex::new(r"^([NSEWLRF])(\d+)$").unwrap();

        match action_regex.captures(action_str) {
            None => None,
            Some(capture) => {
                let num: u32 = capture.get(2).unwrap().as_str().parse().unwrap();
                let action = match capture.get(1).unwrap().as_str() {
                    "N" => Action::North(num),
                    "S" => Action::South(num),
                    "E" => Action::East(num),
                    "W" => Action::West(num),
                    "L" => Action::Left(num / 90),
                    "R" => Action::Right(num / 90),
                    "F" => Action::Foreward(num),
                    _ => panic!(),
                };

                Some(action)
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone)]
struct Position {
    row: i32,
    column: i32,
}

impl Position {
    fn add(&self, rows: i32, columns: i32) -> Position {
        Position {
            row: self.row + rows,
            column: self.column + columns,
        }
    }

    fn manhattan_distance(&self, other: &Position) -> u32 {
        ((self.row - other.row).abs() + (self.column - other.column).abs()) as u32
    }
}

#[derive(Clone)]
struct Turtle {
    position: Position,
    direction: Direction,
}

impl Turtle {
    fn new_default() -> Turtle {
        Turtle {
            position: Position { row: 0, column: 0 },
            direction: Direction::East,
        }
    }

    fn apply(&mut self, action: &Action) {
        match action {
            Action::North(n) => self.position = self.position.add(*n as i32, 0),
            Action::South(n) => self.position = self.position.add(-(*n as i32), 0),
            Action::East(n) => self.position = self.position.add(0, *n as i32),
            Action::West(n) => self.position = self.position.add(0, -(*n as i32)),
            Action::Foreward(n) => {
                let (rows, columns) = match self.direction {
                    Direction::North => (1, 0),
                    Direction::South => (-1, 0),
                    Direction::East => (0, 1),
                    Direction::West => (0, -1),
                };

                self.position = self.position.add(rows * *n as i32, columns * *n as i32)
            }
            Action::Left(n) => {
                for _ in 0..*n {
                    self.direction = match self.direction {
                        Direction::North => Direction::West,
                        Direction::West => Direction::South,
                        Direction::South => Direction::East,
                        Direction::East => Direction::North,
                    };
                }
            }
            Action::Right(n) => {
                for _ in 0..*n {
                    self.direction = match self.direction {
                        Direction::North => Direction::East,
                        Direction::West => Direction::North,
                        Direction::South => Direction::West,
                        Direction::East => Direction::South,
                    };
                }
            }
        }
    }
}

fn read_input<R>(reader: R) -> Vec<Action>
where
    R: BufRead,
{
    reader
        .lines()
        .map(|l| Action::from_str(&l.unwrap()))
        .flatten()
        .collect()
}

fn get_manhatten_dist_after_applying_actions(turtle: &Turtle, actions: &[Action]) -> u32 {
    let mut new_turtle = turtle.clone();

    let starting_pos = new_turtle.position.clone();
    for action in actions.iter() {
        new_turtle.apply(action);
    }

    new_turtle.position.manhattan_distance(&starting_pos)
}
