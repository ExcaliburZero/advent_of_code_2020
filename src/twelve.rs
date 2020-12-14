extern crate regex;

use std::io::{self, BufRead};

pub fn part_one() {
    let actions = read_input(io::stdin().lock());
    let answer = get_manhatten_dist_after_applying_actions(&Turtle::new_default(), &actions);

    println!("{}", answer)
}

pub fn part_two() {
    let actions = read_input(io::stdin().lock());
    let answer =
        get_manhatten_dist_after_applying_actions_with_waypoint(&Turtle::new_default(), &actions);

    println!("{}", answer)
}

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

#[derive(Clone, Debug, PartialEq)]
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

    fn add_pos_times(&self, other: &Position, num_times: u32) -> Position {
        (0..num_times).fold(self.clone(), |p, _| p.add(other.row, other.column))
    }

    fn rotate_left(&self) -> Position {
        Position {
            row: self.column,
            column: -self.row,
        }
    }

    fn rotate_right(&self) -> Position {
        Position {
            row: -self.column,
            column: self.row,
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
    waypoint: Position,
}

impl Turtle {
    fn new_default() -> Turtle {
        Turtle {
            position: Position { row: 0, column: 0 },
            direction: Direction::East,
            waypoint: Position { row: 1, column: 10 },
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

    fn apply_with_waypoint(&mut self, action: &Action) {
        match action {
            Action::North(n) => self.waypoint = self.waypoint.add(*n as i32, 0),
            Action::South(n) => self.waypoint = self.waypoint.add(-(*n as i32), 0),
            Action::East(n) => self.waypoint = self.waypoint.add(0, *n as i32),
            Action::West(n) => self.waypoint = self.waypoint.add(0, -(*n as i32)),
            Action::Foreward(n) => self.position = self.position.add_pos_times(&self.waypoint, *n),
            Action::Left(n) => {
                for _ in 0..*n {
                    self.waypoint = self.waypoint.rotate_left();
                }
            }
            Action::Right(n) => {
                for _ in 0..*n {
                    self.waypoint = self.waypoint.rotate_right();
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

fn get_manhatten_dist_after_applying_actions_with_waypoint(
    turtle: &Turtle,
    actions: &[Action],
) -> u32 {
    let mut new_turtle = turtle.clone();

    let starting_pos = new_turtle.position.clone();
    for action in actions.iter() {
        new_turtle.apply_with_waypoint(action);
    }

    new_turtle.position.manhattan_distance(&starting_pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turtle_apply_with_waypoint_foreward() {
        let mut turtle = Turtle::new_default();

        turtle.apply_with_waypoint(&Action::Foreward(4));

        let expected = Position { row: 4, column: 40 };
        let actual = turtle.position;

        assert_eq!(expected, actual);
    }

    #[test]
    fn position_rotate_left_right() {
        let position = Position {
            row: 12,
            column: 14,
        };
        let actual = position.rotate_left().rotate_right();

        assert_eq!(position, actual);
    }

    #[test]
    fn position_rotate_left() {
        let position = Position { row: 3, column: 1 };

        let rotated_once = position.rotate_left();
        assert_eq!(Position { row: 1, column: -3 }, rotated_once);

        let rotated_twice = rotated_once.rotate_left();
        assert_eq!(
            Position {
                row: -3,
                column: -1
            },
            rotated_twice
        );

        let rotated_thrice = rotated_twice.rotate_left();
        assert_eq!(Position { row: -1, column: 3 }, rotated_thrice);

        let rotated_four = rotated_thrice.rotate_left();
        assert_eq!(position, rotated_four);
    }
}
