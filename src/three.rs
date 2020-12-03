use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let grid = read_input();
    let answer = count_trees_right_3_down_1(&grid, Position { x: 0, y: 0 }, 0);

    println!("{}", answer);
}

pub fn part_two() {}

struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn shifted(&self, x_shift: i32, y_shift: i32) -> Position {
        Position {
            x: ((self.x as i32) + x_shift) as usize,
            y: ((self.y as i32) + y_shift) as usize,
        }
    }
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
        assert!(position.x >= 0);
        assert!(position.y >= 0);
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

fn count_trees_right_3_down_1(grid: &Grid, position: Position, trees_hit: i32) -> i32 {
    if position.y >= grid.height() {
        return trees_hit;
    } else {
        let trees_hit_inc = if grid.get(&position) { 1 } else { 0 };

        return count_trees_right_3_down_1(grid, position.shifted(3, 1), trees_hit + trees_hit_inc);
    }
}
