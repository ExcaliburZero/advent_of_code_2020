use std::io::{self, BufRead};

pub fn part_one() {
    let seat_locations = read_input(io::stdin().lock());
    let answer = get_highest_seat_id(&seat_locations);

    println!("{}", answer);
}

pub fn part_two() {}

enum VerticalDirection {
    Up,
    Down,
}

enum HorizontalDirection {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Position {
    row: u32,
    column: u32,
}

impl Position {
    fn to_id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

struct SeatLocation {
    vertical_directions: Vec<VerticalDirection>,
    horizontal_directions: Vec<HorizontalDirection>,
}

impl SeatLocation {
    fn from_str(seat_location_str: &str) -> SeatLocation {
        let mut vertical_directions: Vec<VerticalDirection> = vec![];
        let mut horizontal_directions: Vec<HorizontalDirection> = vec![];

        for c in seat_location_str.chars() {
            match c {
                'F' => vertical_directions.push(VerticalDirection::Up),
                'B' => vertical_directions.push(VerticalDirection::Down),
                'L' => horizontal_directions.push(HorizontalDirection::Left),
                'R' => horizontal_directions.push(HorizontalDirection::Right),
                _ => panic!(),
            }
        }

        SeatLocation {
            vertical_directions,
            horizontal_directions,
        }
    }

    fn to_position(&self) -> Position {
        let mut row = 0;
        for direction in self.vertical_directions.iter() {
            match direction {
                VerticalDirection::Up => row = row << 1,
                VerticalDirection::Down => row = (row << 1) + 1,
            }
        }

        let mut column = 0;
        for direction in self.horizontal_directions.iter() {
            match direction {
                HorizontalDirection::Left => column = column << 1,
                HorizontalDirection::Right => column = (column << 1) + 1,
            }
        }

        Position { row, column }
    }
}

fn read_input<R>(reader: R) -> Vec<SeatLocation>
where
    R: BufRead,
{
    reader
        .lines()
        .map(|l| SeatLocation::from_str(&l.unwrap()))
        .collect()
}

fn get_highest_seat_id(seat_locations: &Vec<SeatLocation>) -> u32 {
    seat_locations
        .iter()
        .map(|l| l.to_position().to_id())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat_location_to_id() {
        let location = SeatLocation::from_str("BFFFBBFRRR");

        let expected_position = Position { row: 70, column: 7 };
        let position = location.to_position();

        assert_eq!(expected_position, position);

        let expected_id = 567;
        let id = position.to_id();

        assert_eq!(expected_id, id);
    }
}
