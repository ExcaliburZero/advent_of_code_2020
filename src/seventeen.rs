extern crate itertools;

use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

use self::itertools::Itertools;

pub fn part_one() {
    let dimension = read_input(io::stdin().lock());
    let answer = get_num_active_cells(&dimension, 6);

    println!("{}", answer)
}

pub fn part_two() {}

fn read_input<R>(reader: R) -> PocketDimension
where
    R: BufRead,
{
    PocketDimension::from_str(&reader.lines().map(|l| l.unwrap()).join("\n")).unwrap()
}

fn get_num_active_cells(dimension: &PocketDimension, num_cycles: u64) -> usize {
    (0..num_cycles)
        .fold(dimension.clone(), |d, _| d.get_next_cycle())
        .get_num_active_cells()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CellState {
    Inactive,
    Active,
}

impl CellState {
    fn from_str(cell_state_str: &str) -> Option<CellState> {
        match cell_state_str {
            "." => Some(CellState::Inactive),
            "#" => Some(CellState::Active),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct PocketDimension {
    cells: HashMap<Position, CellState>,
}

impl PocketDimension {
    fn from_str(grid_str: &str) -> Result<PocketDimension, String> {
        let mut cells: HashMap<Position, CellState> = HashMap::new();
        for (y, line) in grid_str.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match CellState::from_str(&c.to_string()) {
                    None => return Err(format!("Invalid cell character: {}", c)),
                    Some(state) => {
                        cells.insert(Position::new(x as i64, y as i64, 0), state);
                    }
                }
            }
        }

        Ok(PocketDimension { cells })
    }

    fn get_cell(&self, position: &Position) -> CellState {
        match self.cells.get(position) {
            None => CellState::Inactive,
            Some(state) => *state,
        }
    }

    fn set_cell(&mut self, position: &Position, state: CellState) {
        self.cells.insert(*position, state);
    }

    fn get_num_active_cells(&self) -> usize {
        self.cells
            .values()
            .filter(|s| **s == CellState::Active)
            .count()
    }

    fn get_possible_positions(&self) -> HashSet<Position> {
        let mut possible_positions: HashSet<Position> = HashSet::new();
        for pos in self.cells.keys() {
            possible_positions.insert(*pos);

            for n_pos in pos.get_neighbors() {
                possible_positions.insert(n_pos);
            }
        }

        possible_positions
    }

    fn get_num_active_neighbors(&self, position: &Position) -> usize {
        position
            .get_neighbors()
            .iter()
            .map(|p| self.get_cell(p))
            .filter(|p| *p == CellState::Active)
            .count()
    }

    fn get_next_cycle_state(&self, position: &Position) -> CellState {
        let num_active_neighbors = self.get_num_active_neighbors(position);

        match self.get_cell(position) {
            CellState::Active => {
                if (2..=3).contains(&num_active_neighbors) {
                    CellState::Active
                } else {
                    CellState::Inactive
                }
            }
            CellState::Inactive => {
                if num_active_neighbors == 3 {
                    CellState::Active
                } else {
                    CellState::Inactive
                }
            }
        }
    }

    fn get_next_cycle(&self) -> PocketDimension {
        let mut new_dimension = self.clone();

        for pos in self.get_possible_positions() {
            let new_state = self.get_next_cycle_state(&pos);

            new_dimension.set_cell(&pos, new_state);
        }

        new_dimension
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    fn new(x: i64, y: i64, z: i64) -> Position {
        Position { x, y, z }
    }

    fn get_neighbors(&self) -> Vec<Position> {
        let mut neighbors: Vec<Position> = vec![];
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    } else {
                        neighbors.push(Position {
                            x: self.x + x,
                            y: self.y + y,
                            z: self.z + z,
                        });
                    }
                }
            }
        }

        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_get_neighbors() {
        let position = Position::new(0, 0, 0);

        let expected = vec![
            Position::new(-1, -1, -1),
            Position::new(-1, -1, 0),
            Position::new(-1, -1, 1),
            Position::new(-1, 0, -1),
            Position::new(-1, 0, 0),
            Position::new(-1, 0, 1),
            Position::new(-1, 1, -1),
            Position::new(-1, 1, 0),
            Position::new(-1, 1, 1),
            //
            Position::new(0, -1, -1),
            Position::new(0, -1, 0),
            Position::new(0, -1, 1),
            Position::new(0, 0, -1),
            Position::new(0, 0, 1),
            Position::new(0, 1, -1),
            Position::new(0, 1, 0),
            Position::new(0, 1, 1),
            //
            Position::new(1, -1, -1),
            Position::new(1, -1, 0),
            Position::new(1, -1, 1),
            Position::new(1, 0, -1),
            Position::new(1, 0, 0),
            Position::new(1, 0, 1),
            Position::new(1, 1, -1),
            Position::new(1, 1, 0),
            Position::new(1, 1, 1),
        ];
        let actual = position.get_neighbors();

        assert_eq!(expected, actual);
    }

    #[test]
    fn pocket_dimension_cycle_empty() {
        let dimension = PocketDimension::from_str(".").unwrap();
        let final_dimension = dimension.get_next_cycle();

        assert_eq!(0, final_dimension.get_num_active_cells());
    }

    #[test]
    fn pocket_dimension_cycle_very_simple() {
        let dimension = PocketDimension::from_str("#").unwrap();
        let final_dimension = dimension.get_next_cycle();

        assert_eq!(0, final_dimension.get_num_active_cells());
    }

    #[test]
    fn pocket_dimension_cycle_simple() {
        let dimension = PocketDimension::from_str("###").unwrap();
        let final_dimension = dimension.get_next_cycle();

        assert_eq!(9, final_dimension.get_num_active_cells());
    }

    #[test]
    fn pocket_dimension_get_cell_active() {
        let dimension = PocketDimension::from_str("###").unwrap();

        assert_eq!(
            CellState::Active,
            dimension.get_cell(&Position::new(0, 0, 0))
        );
        assert_eq!(
            CellState::Active,
            dimension.get_cell(&Position::new(1, 0, 0))
        );
        assert_eq!(
            CellState::Active,
            dimension.get_cell(&Position::new(2, 0, 0))
        );
        assert_eq!(
            CellState::Inactive,
            dimension.get_cell(&Position::new(3, 0, 0))
        );
    }

    #[test]
    fn pocket_dimension_get_num_active_neighbors() {
        let dimension = PocketDimension::from_str("###").unwrap();

        assert_eq!(
            1,
            dimension.get_num_active_neighbors(&Position::new(0, 0, 0))
        );
        assert_eq!(
            2,
            dimension.get_num_active_neighbors(&Position::new(1, 0, 0))
        );
    }

    #[test]
    fn pocket_dimension_get_next_cycle_state() {
        let dimension = PocketDimension::from_str("###").unwrap();

        assert_eq!(
            CellState::Inactive,
            dimension.get_next_cycle_state(&Position::new(0, 0, 0))
        );
        assert_eq!(
            CellState::Active,
            dimension.get_next_cycle_state(&Position::new(1, 0, 0))
        );
        assert_eq!(
            CellState::Inactive,
            dimension.get_next_cycle_state(&Position::new(2, 0, 0))
        );
        assert_eq!(
            CellState::Inactive,
            dimension.get_next_cycle_state(&Position::new(3, 0, 0))
        );
    }
}
