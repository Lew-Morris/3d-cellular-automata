mod rule;

use std::{fmt, io::stdin};
use crate::rule::Rule;

fn main() {
    let states: u8 = 5;
    let current_rule: Rule = Rule::new(states); // New rule with default values

    // User chooses BOUNDS size
    println!("Enter BOUNDS size [1-512]: ");
    let mut input_bounds: String = String::new();
    stdin()
        .read_line(&mut input_bounds)
        .expect("Failed to read input");

     let bounds: i32 = input_bounds
        .trim()
        .parse::<i32>()
        .expect("Input not a number");

    // Input validation
    if bounds > 512 { panic!("Bounds are too large!") }
    else if bounds < 0 { panic!("Bounds cannot be negative!") }
    else if bounds == 0 { panic!("Bounds cannot be zero!") }

}

#[derive(Clone, Copy)]
struct SimpleCell {
    value: u8,
    neighbours: u8,
}

struct SingleThreaded {
    cells: Vec<SimpleCell>,
    bounds: i32,
}

#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    pub fn new() -> Position {
        Position {
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

impl SimpleCell {
    pub fn dead(self) -> bool {
        self.value == 0
    }
}

impl SingleThreaded {
    pub fn new() -> Self {
        SingleThreaded {
            cells: vec![],
            bounds: 0,
        }
    }

    pub fn set_bounds(&mut self, bounds: i32) -> i32 {
        if bounds != self.bounds {
            self.cells.clear();
            self.cells.resize(
                (bounds.pow(3)) as usize,
                SimpleCell { value: 0, neighbours: 0});
            self.bounds = bounds;
        }
        self.bounds
    }

    pub fn count_cells(&self) -> usize {
        let mut result = 0;
        for cell in &self.cells {
            if !cell.dead() {
                result += 1;
            }
        }
        result
    }

    fn update_neighbours(&mut self, rule: &Rule, index: usize) {

    }
}


//
// impl fmt::Display for Position {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}, {}, {}", self.x, self.y, self.z)
//     }
// }
