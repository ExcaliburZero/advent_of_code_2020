use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let grid = read_input();
    let answer = count_trees_on_path(
        &grid,
        &PositionChange {
            x_shift: 3,
            y_shift: 1,
        },
        &Position { x: 0, y: 0 },
        0,
    );

    println!("{}", answer);
}

pub fn part_two() {
    let grid = read_input();
    let answer = count_and_multiply_trees_on_paths(
        &grid,
        &vec![
            PositionChange {
                x_shift: 1,
                y_shift: 1,
            },
            PositionChange {
                x_shift: 3,
                y_shift: 1,
            },
            PositionChange {
                x_shift: 5,
                y_shift: 1,
            },
            PositionChange {
                x_shift: 7,
                y_shift: 1,
            },
            PositionChange {
                x_shift: 1,
                y_shift: 2,
            },
        ],
        &Position { x: 0, y: 0 },
    );

    println!("{}", answer);
}

struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn shifted(&self, change: &PositionChange) -> Position {
        let x_shift = change.x_shift;
        let y_shift = change.y_shift;

        Position {
            x: ((self.x as i32) + x_shift) as usize,
            y: ((self.y as i32) + y_shift) as usize,
        }
    }
}

struct PositionChange {
    x_shift: i32,
    y_shift: i32,
}

struct Grid {
    /// Grid of cells indicating whether a give cell in the grid contains a tree or not
    /// (True=tree, False=no tree). First dimension is rows where 0 is top, second dimension is
    /// columns where 0 is left side.
    cells: Vec<Vec<bool>>,
}

impl Grid {
    pub fn from_str(grid_str: &str) -> Grid {
        let mut cells: Vec<Vec<bool>> = vec![];
        for line in grid_str.split("\n") {
            let mut row: Vec<bool> = vec![];
            for c in line.chars() {
                let value = match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("Invalid grid character: {}", c),
                };

                row.push(value);
            }

            cells.push(row);
        }

        Grid { cells }
    }

    pub fn get(&self, position: &Position) -> bool {
        assert!(position.y < self.height());

        let x_adjusted = position.x % self.width();

        self.cells[position.y][x_adjusted]
    }

    pub fn height(&self) -> usize {
        self.cells.len()
    }

    pub fn width(&self) -> usize {
        assert!(self.cells.len() > 0);

        self.cells[0].len()
    }
}

fn read_input() -> Grid {
    let grid_string = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    Grid::from_str(&grid_string)
}

fn count_trees_on_path(
    grid: &Grid,
    slope: &PositionChange,
    position: &Position,
    trees_hit: i32,
) -> i32 {
    if position.y >= grid.height() {
        return trees_hit;
    } else {
        let trees_hit_inc = if grid.get(&position) { 1 } else { 0 };

        return count_trees_on_path(
            grid,
            slope,
            &position.shifted(slope),
            trees_hit + trees_hit_inc,
        );
    }
}

fn count_and_multiply_trees_on_paths(
    grid: &Grid,
    slopes: &Vec<PositionChange>,
    starting_position: &Position,
) -> i64 {
    let mut nums_of_trees_hit: Vec<i64> = vec![];
    for slope in slopes.iter() {
        let trees_hit = count_trees_on_path(grid, slope, starting_position, 0);

        nums_of_trees_hit.push(trees_hit as i64);
    }

    nums_of_trees_hit.iter().fold(1, |a, b| a * b)
}
