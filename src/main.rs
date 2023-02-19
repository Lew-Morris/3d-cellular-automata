// mod rule;
// mod tests;
mod simple_cell;

use std::{io::stdin};
use crate::simple_cell::SingleThreaded;

fn main() {
    // let states: u8 = 5;
    // let current_rule: Rule = Rule::new(states); // New rule with default values

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

    // Instantiate SingleThreaded
    let mut cells: SingleThreaded = SingleThreaded::new();
    cells.set_bounds(bounds);
    let cell_count = cells.count_cells();

    println!("There are {} live cells", cell_count);

    // A simple loop to test the `is_boundary` method
    loop {
        // Request input from user
        println!("Enter in a cell's position: ");
        let mut input: String = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Parse and store input as i32 (32-bit integer)
        let position: i32 = input
            .trim()
            .parse::<i32>()
            .expect("Input was not a number");

        // Validate input
        if position - 1 > bounds.pow(3) - 1 { panic!("Position out of bounds - The maximum value was {}", bounds.pow(3)) }
        else if position - 1 < 0 { panic!("Position out of bounds - too small!") }
        else {
            // Output the specified cell's properties (Value/ Boundary)
            cells.print_cell(position as usize);
            println!("Boundary Cell: {}", is_boundary(bounds as usize, position as usize))
        }
    }

}



// Check if the cell is on the boundary using the index
fn is_boundary(bounds: usize, index: usize) -> bool {
    // todo! Return the type of boundary
    let size = bounds.pow(3);
    match index {
        // first layer
        index if index < bounds * bounds => true,
        // last layer
        index if index >= size - (bounds * bounds) => true,
        // first column
        index if index % bounds == 0 => true,
        // last column
        index if index % bounds == bounds - 1 => true,
        // first slice
        index if index % (bounds * bounds) < bounds => true,
        // last slice
        index if index % (bounds * bounds) >= bounds * (bounds - 1) => true,
        _ => false,
    }
}

//
// impl fmt::Display for Position {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}, {}, {}", self.x, self.y, self.z)
//     }
// }

