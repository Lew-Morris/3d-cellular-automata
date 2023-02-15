use std::{fmt, io::stdin};

fn main() {
    // let my_first_cell = SimpleCell {
    //     position: (1, 2, 3),
    //     alive: true,
    //     states: 5,
    //     neighbours: 5,
    // };
    // println!("Original Cell\n-------------\n{}", my_first_cell);
    let mut cell_list: Vec<SimpleCell> = Vec::new();
    println!("Enter bounds size: ");

    // User chooses bounds size
    let mut input_bounds: String = String::new();
    stdin()
        .read_line(&mut input_bounds)
        .expect("Failed to read input");

    let bounds: i32 = input_bounds.trim().parse::<i32>().expect("Input not a number");

    // Input validation
    if bounds > 512 { panic!("Bounds are too large!") }
    else if bounds < 0 { panic!("Bounds cannot be negative!") }

    for z in 0..bounds {
        for y in 0..bounds {
            for x in 0..bounds {
                cell_list.push(SimpleCell::new(x, y, z, 5, 0))
            }
        }
    }
    let count = cell_list.iter().count();
    println!("Total cells: {}", count)
}

#[derive(Clone)]
struct SimpleCell {
    position: (i32, i32, i32),
    alive: bool,
    states: i32,
    neighbours: i32,
}

impl SimpleCell {
    pub fn new(x: i32, y: i32, z: i32, states: i32, neighbours: i32) -> SimpleCell {
        SimpleCell {
            position: (x, y, z),
            alive: true,
            states,
            neighbours,
        }
    }
    // pub fn get_position(&self) ->(i32, i32, i32) {
    //     self.position
    // }
    //
    // pub fn get_alive(&self) -> bool {
    //     self.alive
    // }
    //
    // pub fn get_states(&self) -> i32 {
    //     self.states
    // }
    //
    // pub fn get_neighbours(&self) -> i32 {
    //     self.neighbours
    // }

    // pub fn set_position(&mut self, position: (i32, i32, i32)) {
    //     self.position = position;
    // }
    //
    // pub fn set_alive(&mut self, alive: bool) {
    //     self.alive = alive;
    // }
    //
    // pub fn set_states(&mut self, states: i32){
    //     self.states = states;
    // }
    //
    // pub fn set_neighbours(&mut self, neighbours: i32) {
    //     self.neighbours = neighbours;
    // }
}

impl fmt::Display for SimpleCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Position: {}, {}, {}\nAlive: {}\nStates: {}\nNeighbours: {}\n",
               self.position.0, self.position.1, self.position.2, self.alive, self.states, self.neighbours)
    }
}


// struct Position {
//     x: i32,
//     y: i32,
//     z: i32,
// }

// impl fmt::Display for Position {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}, {}, {}", self.x, self.y, self.z)
//     }
// }
