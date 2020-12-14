extern crate itertools;

use std::io::{self, BufRead};

use eleven::itertools::Itertools;

pub fn part_one() {
    let seats = read_input(io::stdin().lock());
    let answer = num_occupied_seats_once_stable(&seats, &IterationRule::DirectNeighbors);
    println!("{}", answer);
}

pub fn part_two() {
    let seats = read_input(io::stdin().lock());
    let answer = num_occupied_seats_once_stable(&seats, &IterationRule::LinesOfSight);
    println!("{}", answer);
}

enum IterationRule {
    DirectNeighbors,
    LinesOfSight,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SeatStatus {
    Floor,
    Empty,
    Occupied,
}

#[derive(Clone, Debug, PartialEq)]
struct Seats {
    seats: Vec<SeatStatus>,
    num_rows: usize,
    num_columns: usize,
}

impl Seats {
    fn new(num_rows: usize, num_columns: usize, default_value: SeatStatus) -> Seats {
        let last_position = Position {
            row: num_rows - 1,
            column: num_columns - 1,
        };
        let num_entries = Seats::pos_to_index(num_rows, num_columns, &last_position).unwrap() + 1;
        let seats = vec![default_value; num_entries];

        Seats {
            seats,
            num_rows,
            num_columns,
        }
    }

    fn from_lines(lines: &[String]) -> Seats {
        let num_rows = lines.len();
        assert!(num_rows > 0);
        let num_columns = lines[0].len();

        let mut seats = Seats::new(num_rows, num_columns, SeatStatus::Floor);
        for (row, line) in lines.iter().enumerate() {
            for (column, v) in line.chars().enumerate() {
                let status = match v {
                    '.' => SeatStatus::Floor,
                    'L' => SeatStatus::Empty,
                    '#' => SeatStatus::Occupied,
                    _ => panic!(),
                };

                let pos = Position { row, column };
                seats.set(&pos, status);
            }
        }

        seats
    }

    fn pos_to_index(num_rows: usize, num_columns: usize, pos: &Position) -> Option<usize> {
        if pos.row >= num_rows || pos.column >= num_columns {
            None
        } else {
            Some(pos.row * num_columns + pos.column)
        }
    }

    fn get(&self, pos: &Position) -> Option<SeatStatus> {
        Seats::pos_to_index(self.num_rows, self.num_columns, pos).map(|i| self.seats[i])
    }

    fn set(&mut self, pos: &Position, value: SeatStatus) -> bool {
        if pos.row >= self.num_rows || pos.column >= self.num_columns {
            false
        } else {
            let i = pos.row * self.num_columns + pos.column;

            self.seats[i] = value;
            true
        }
    }

    fn get_neighbors(&self, center: &Position) -> Vec<(Position, SeatStatus)> {
        let neighbor_positions: Vec<Position> = vec![-1, 0, 1]
            .iter()
            .cartesian_product(vec![-1, 0, 1])
            .filter(|(r, c): &(&i32, i32)| **r != 0 || *c != 0)
            .map(|(r, c)| center.add(*r, c))
            .filter(|p| p.is_some())
            .map(|p| p.unwrap())
            .collect();

        neighbor_positions
            .iter()
            .flat_map(|p| self.get(p).map(|v| (*p, v)))
            .collect()
    }

    fn get_line_of_sight_neighbors(&self, center: &Position) -> Vec<(Position, SeatStatus)> {
        let mut directions: Vec<(i32, i32)> = vec![];
        for rows in -1..=1 {
            for columns in -1..=1 {
                if rows != 0 || columns != 0 {
                    directions.push((rows, columns));
                }
            }
        }

        let mut line_of_sight_neighbors: Vec<(Position, SeatStatus)> = vec![];
        for dir in directions {
            let mut seat_in_dir = Some(*center);
            let mut found_seat = false;
            while !found_seat && seat_in_dir.is_some() {
                seat_in_dir = seat_in_dir.unwrap().add(dir.0, dir.1);

                match seat_in_dir {
                    None => (),
                    Some(ref seat_pos) => match self.get(&seat_pos) {
                        None => seat_in_dir = None,
                        Some(SeatStatus::Floor) => (),
                        Some(SeatStatus::Empty) => {
                            found_seat = true;
                            line_of_sight_neighbors.push((seat_in_dir.unwrap(), SeatStatus::Empty));
                        }
                        Some(SeatStatus::Occupied) => {
                            found_seat = true;
                            line_of_sight_neighbors
                                .push((seat_in_dir.unwrap(), SeatStatus::Occupied));
                        }
                    },
                }
            }
        }

        line_of_sight_neighbors
    }

    fn num_occupied_neighbors(&self, pos: &Position) -> u32 {
        self.get_neighbors(pos)
            .iter()
            .map(|(_, status)| status)
            .filter(|status| **status == SeatStatus::Occupied)
            .count() as u32
    }

    fn num_occupied_line_of_sights(&self, pos: &Position) -> u32 {
        self.get_line_of_sight_neighbors(pos)
            .iter()
            .map(|(_, status)| status)
            .filter(|status| **status == SeatStatus::Occupied)
            .count() as u32
    }

    fn seat_next_value(&self, pos: &Position, iteration_rule: &IterationRule) -> SeatStatus {
        match self.get(pos) {
            None => panic!(),
            Some(v) => match iteration_rule {
                IterationRule::DirectNeighbors => match v {
                    SeatStatus::Floor => SeatStatus::Floor,
                    SeatStatus::Empty => {
                        if self.num_occupied_neighbors(pos) == 0 {
                            SeatStatus::Occupied
                        } else {
                            SeatStatus::Empty
                        }
                    }
                    SeatStatus::Occupied => {
                        if self.num_occupied_neighbors(pos) >= 4 {
                            SeatStatus::Empty
                        } else {
                            SeatStatus::Occupied
                        }
                    }
                },
                IterationRule::LinesOfSight => match v {
                    SeatStatus::Floor => SeatStatus::Floor,
                    SeatStatus::Empty => {
                        if self.num_occupied_line_of_sights(pos) == 0 {
                            SeatStatus::Occupied
                        } else {
                            SeatStatus::Empty
                        }
                    }
                    SeatStatus::Occupied => {
                        if self.num_occupied_line_of_sights(pos) >= 5 {
                            SeatStatus::Empty
                        } else {
                            SeatStatus::Occupied
                        }
                    }
                },
            },
        }
    }

    fn positions(&self) -> Vec<Position> {
        let mut positions = vec![];
        for row in 0..self.num_rows {
            for column in 0..self.num_columns {
                positions.push(Position { row, column });
            }
        }

        positions
    }

    fn iteration(&self, iteration_rule: &IterationRule, output_seats: &mut Seats) {
        for pos in self.positions() {
            output_seats.set(&pos, self.seat_next_value(&pos, iteration_rule));
        }
    }

    fn get_num_seats_occupied(&self) -> usize {
        self.seats
            .iter()
            .filter(|status| **status == SeatStatus::Occupied)
            .count()
    }

    /*fn print(&self) {
        for row in 0..self.num_rows {
            for column in 0..self.num_columns {
                let pos = Position { row, column };

                let c = match self.get(&pos).unwrap() {
                    SeatStatus::Floor => '.',
                    SeatStatus::Empty => 'L',
                    SeatStatus::Occupied => '#',
                };

                print!("{}", c);
            }

            println!();
        }
    }*/
}

#[derive(Clone, Copy)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn add(&self, rows: i32, columns: i32) -> Option<Position> {
        let row = self.row as i32 + rows;
        let column = self.column as i32 + columns;

        if row < 0 || column < 0 {
            None
        } else {
            Some(Position {
                row: row as usize,
                column: column as usize,
            })
        }
    }
}

fn read_input<R>(reader: R) -> Seats
where
    R: BufRead,
{
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    Seats::from_lines(&lines)
}

fn num_occupied_seats_once_stable(seats: &Seats, iteration_rule: &IterationRule) -> usize {
    let mut seats_a = seats.clone();
    let mut seats_b = seats.clone();

    let mut unstable = true;
    while unstable {
        seats_a.iteration(iteration_rule, &mut seats_b);

        if seats_a == seats_b {
            unstable = false;
        }

        seats_a = seats_b.clone();
    }

    seats_a.get_num_seats_occupied()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn position(row: usize, column: usize) -> Position {
        Position { row, column }
    }

    #[test]
    fn seats_from_lines_simple() {
        let lines: Vec<String> = vec!["L.L".into(), ".#L".into()];
        let seats = Seats::from_lines(&lines);

        assert_eq!(2, seats.num_rows);
        assert_eq!(3, seats.num_columns);

        assert_eq!(Some(SeatStatus::Empty), seats.get(&position(0, 0)));
        assert_eq!(Some(SeatStatus::Floor), seats.get(&position(0, 1)));
        assert_eq!(Some(SeatStatus::Empty), seats.get(&position(0, 2)));
        assert_eq!(Some(SeatStatus::Floor), seats.get(&position(1, 0)));
        assert_eq!(Some(SeatStatus::Occupied), seats.get(&position(1, 1)));
        assert_eq!(Some(SeatStatus::Empty), seats.get(&position(1, 2)));
    }

    #[test]
    fn seats_equal() {
        let lines_a: Vec<String> = vec!["L".into()];
        let seats_a = Seats::from_lines(&lines_a);

        let lines_b: Vec<String> = vec!["L".into()];
        let seats_b = Seats::from_lines(&lines_b);

        assert_eq!(seats_a, seats_b);
    }

    #[test]
    fn seats_not_equal() {
        let lines_a: Vec<String> = vec!["L".into()];
        let seats_a = Seats::from_lines(&lines_a);

        let lines_b: Vec<String> = vec!["#".into()];
        let seats_b = Seats::from_lines(&lines_b);

        assert_ne!(seats_a, seats_b);
    }

    #[test]
    fn seats_iteration_simple_empty() {
        let lines: Vec<String> = vec!["L".into()];
        let seats = Seats::from_lines(&lines);

        assert_eq!(1, seats.num_rows);
        assert_eq!(1, seats.num_columns);

        let mut next_seats = seats.clone();
        seats.iteration(&IterationRule::DirectNeighbors, &mut next_seats);

        assert_eq!(Some(SeatStatus::Occupied), next_seats.get(&position(0, 0)));
    }

    #[test]
    fn seats_iteration_simple_filled() {
        let lines: Vec<String> = vec!["#".into()];
        let seats = Seats::from_lines(&lines);

        assert_eq!(1, seats.num_rows);
        assert_eq!(1, seats.num_columns);

        let mut next_seats = seats.clone();
        seats.iteration(&IterationRule::DirectNeighbors, &mut next_seats);

        assert_eq!(Some(SeatStatus::Occupied), next_seats.get(&position(0, 0)));
    }

    #[test]
    fn seats_iteration_simple_floor() {
        let lines: Vec<String> = vec![".".into()];
        let seats = Seats::from_lines(&lines);

        assert_eq!(1, seats.num_rows);
        assert_eq!(1, seats.num_columns);

        let mut next_seats = seats.clone();
        seats.iteration(&IterationRule::DirectNeighbors, &mut next_seats);

        assert_eq!(Some(SeatStatus::Floor), next_seats.get(&position(0, 0)));
    }

    #[test]
    fn num_occupied_seats_once_stable_example_case() {
        let lines: Vec<String> = vec![
            "L.LL.LL.LL".into(),
            "LLLLLLL.LL".into(),
            "L.L.L..L..".into(),
            "LLLL.LL.LL".into(),
            "L.LL.LL.LL".into(),
            "L.LLLLL.LL".into(),
            "..L.L.....".into(),
            "LLLLLLLLLL".into(),
            "L.LLLLLL.L".into(),
            "L.LLLLL.LL".into(),
        ];
        let seats = Seats::from_lines(&lines);

        let expected = 37;
        let actual = num_occupied_seats_once_stable(&seats, &IterationRule::DirectNeighbors);

        assert_eq!(expected, actual);
    }
}
