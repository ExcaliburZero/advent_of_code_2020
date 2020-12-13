extern crate itertools;

use std::io::{self, BufRead};

use eleven::itertools::Itertools;

pub fn part_one() {
    let seats = read_input(io::stdin().lock());
    let answer = num_occupied_seats_once_stable(&seats);
    println!("{}", answer);
}

pub fn part_two() {}

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
            .flat_map(|p| self.get(p).map(|v| (p.clone(), v)))
            .collect()
    }

    fn num_occupied_neighbors(&self, pos: &Position) -> u32 {
        self.get_neighbors(pos)
            .iter()
            .map(|(_, status)| status)
            .filter(|status| **status == SeatStatus::Occupied)
            .count() as u32
    }

    fn seat_next_value(&self, pos: &Position) -> SeatStatus {
        match self.get(pos) {
            None => panic!(),
            Some(v) => match v {
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

    fn iteration(&self, output_seats: &mut Seats) {
        for pos in self.positions() {
            output_seats.set(&pos, self.seat_next_value(&pos));
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

#[derive(Clone)]
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

fn num_occupied_seats_once_stable(seats: &Seats) -> usize {
    let mut seats_a = seats.clone();
    let mut seats_b = seats.clone();

    let mut unstable = true;
    while unstable {
        seats_a.iteration(&mut seats_b);

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
        seats.iteration(&mut next_seats);

        assert_eq!(Some(SeatStatus::Occupied), next_seats.get(&position(0, 0)));
    }

    #[test]
    fn seats_iteration_simple_filled() {
        let lines: Vec<String> = vec!["#".into()];
        let seats = Seats::from_lines(&lines);

        assert_eq!(1, seats.num_rows);
        assert_eq!(1, seats.num_columns);

        let mut next_seats = seats.clone();
        seats.iteration(&mut next_seats);

        assert_eq!(Some(SeatStatus::Occupied), next_seats.get(&position(0, 0)));
    }

    #[test]
    fn seats_iteration_simple_floor() {
        let lines: Vec<String> = vec![".".into()];
        let seats = Seats::from_lines(&lines);

        assert_eq!(1, seats.num_rows);
        assert_eq!(1, seats.num_columns);

        let mut next_seats = seats.clone();
        seats.iteration(&mut next_seats);

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
        let actual = num_occupied_seats_once_stable(&seats);

        assert_eq!(expected, actual);
    }
}
