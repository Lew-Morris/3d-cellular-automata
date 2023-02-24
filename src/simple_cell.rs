use crate::rule::Rule;

use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy)]
struct SimpleCell {
    value: u8,
    neighbours: u8,
}

impl SimpleCell {
    // Return `true` if the cell has value 0
    pub fn dead(self) -> bool {
        self.value == 0
    }
}

pub(crate) struct SingleThreaded {
    cells: Vec<SimpleCell>,
    bounds: i32,
}

impl SingleThreaded {
    pub fn new() -> Self {
        SingleThreaded {
            cells: vec![],
            bounds: 0,
        }
    }

    pub fn print_cell(&self, index: usize) {
        if self.cells[index - 1].dead() {
            print!("DEAD ");
        }
        println!("Cell #{} \n--------\nValue: {}", index, self.cells[index - 1]);
    }

    // Set the boundary for cells
    pub fn set_bounds(&mut self, bounds: i32) -> i32 {
        // Check if bounds has changed
        if bounds != self.bounds {
            // Clear the array
            self.cells.clear();
            // Initialise vector of cells, with length bounds^3
            self.cells.resize(
                (bounds.pow(3)) as usize,
                // SimpleCell { value: 0, neighbours: 0});
                SimpleCell { value: 0, neighbours: 0 });
            self.bounds = bounds;
        }
        self.bounds
    }

    // Count the number of live cells
    pub fn count_cells(&self, all: bool) -> usize {
        let mut result: usize = 0;
        // Loop through all the cells
        for cell in &self.cells {
            // Increment if the cell is not dead
            if !cell.dead() | all {
                result += 1;
            }
        }
        result
    }

    // todo! Cells need to be stored in a 3D vector
    // pub fn count_neighbours(&self) -> {
    //
    // }

    // // Update the values of the neighbours if the cell dies
    // fn update_neighbours(&mut self, rule: &Rule, index: usize) {
    //
    // }
}

impl Display for SimpleCell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}


// #[derive(Clone, Copy)]
// struct Position {
//     x: i32,
//     y: i32,
//     z: i32,
// }

// impl Position {
//     pub fn new() -> Position {
//         Position {
//             x: 0,
//             y: 0,
//             z: 0,
//         }
//     }
// }