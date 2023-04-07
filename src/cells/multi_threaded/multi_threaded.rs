// use std::thread::spawn;
use bevy::{
    math::{
        IVec3,
    },
    tasks::TaskPool,
};
use crate::{
    render::CellRenderer,
    rule::Rule,
    utilities::{
        wrap,
        default_noise,
        get_centre,
    }
};

/*
todo!
    - Split the cube into x amount of chunks
        - Likely to be the number of CPUS/threads on the system
    - Give each thread its chunk of the cube, with an extra layer on each side as a boundary
        - The thread would only need to do the cells which are not in the boundary
    - Wait for each thread to finish, then render
    - This is called `Domain Decomposition`
 */

// CONSTANTS
#[allow(unused)] // todo! remove
const CHUNK_SIZE: usize = 16;
#[allow(unused)] // todo! remove
const CELLS_PER_CHUNK: usize = CHUNK_SIZE.pow(3);


// todo! Move or remove
// fn bounds_to_chunk(bounds: i32) -> usize {
//     (bounds as usize + CHUNK_SIZE - 1) / CHUNK_SIZE
// }
//
// fn offset_to_position(offset: usize) -> IVec3 {
//     utilities::idx_to_pos(offset as i32, CHUNK_SIZE as i32)
// }
//
// fn is_border(pos: IVec3, offset: i32) -> bool {
//     pos.x - offset <= 0 || pos.x + offset >= (CHUNK_SIZE - 1) as i32 ||
//         pos.y - offset <= 0 || pos.y + offset >= (CHUNK_SIZE - 1) as i32 ||
//         pos.z - offset <= 0 || pos.z + offset >= (CHUNK_SIZE - 1) as i32
// }

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct ParallelCell {
    pub state: u8,
    pub neighbours: u8,
    // pub position: Position,
    // chunk_index: usize,
}

#[derive(Clone)]
pub struct MultiThreaded {
    pub cells: Vec<Vec<Vec<ParallelCell>>>,
    pub bounds: i32,
    pub chunk_radius: usize,
    pub chunk_count: usize,
}

impl ParallelCell {
    fn new() -> ParallelCell {
        ParallelCell {
            state: 0,
            neighbours: 0,
            // chunk_index: 0,
            // position: Position::new(),
        }
    }

    pub fn is_dead(&self) -> bool { self.state == 0}
}

impl Position {
    fn new(x: i32, y: i32, z: i32) -> Position {
        Position {
            x: x as usize,
            y: y as usize,
            z: z as usize,
        }
    }

    fn from_vec(pos: IVec3) -> Position {
        Position {
            x: pos.x as usize,
            y: pos.y as usize,
            z: pos.z as usize,
        }
    }
//
//     #[allow(dead_code)] // todo! remove
//     pub fn is_boundary(&self) {
//         todo!()
//     }
}

unsafe impl Sync for ParallelCell {}
unsafe impl Send for ParallelCell {}

impl MultiThreaded {
    #[allow(dead_code)] // todo! remove
    pub fn new() -> Self {
        MultiThreaded {
            cells: vec![vec![vec![]]],
            bounds: 0,
            chunk_radius: 0,
            chunk_count: 0,
        }
    }

    pub fn get_cell(&self, index: Position) -> ParallelCell {
        // println!("Index: {:#?}", index);
        // println!("Cell Length: {:#?}", self.cells.len());
        self.cells[index.x][index.y][index.z]
    }

    pub fn get_bounds(&self) -> i32 {
        self.bounds as i32
        // (self.chunk_radius * CHUNK_SIZE) as i32
    }

    // fn get_centre(&self) -> IVec3 {
    //     let centre = self.get_bounds() / 2;
    //     ivec3(
    //         centre,
    //         centre,
    //         centre,
    //     )
    // }

    pub fn get_count(&self) -> usize{
        let mut total = 0;
        for x in 0..self.bounds {
            for y in 0..self.bounds {
                for z in 0..self.bounds {
                    if !self.cells[x as usize][y as usize][z as usize].is_dead() {
                        total += 1;
                    }
                }
            }
        }
        total
    }

    // fn get_total_cells(&self) -> i32 {
    //     self.bounds.pow(3)
    // }

    pub fn set_bounds(&mut self, new_bounds: i32) -> i32 {
        if new_bounds != self.bounds {
            self.cells.clear();
            // Source: https://programming-idioms.org/idiom/27/create-a-3-dimensional-array/452/rust
            self.cells = vec![vec![vec![ParallelCell::new(); new_bounds as usize]; new_bounds as usize]; new_bounds as usize];
            self.bounds = new_bounds;
        }

        self.bounds as i32
    }

    // fn update_cells() {
    //     // todo Not sure if needed
    // }

    // todo! write as linear, than make parallel
    fn update(&mut self, rule: &Rule) {
        //Initialize two empty vectors, spawns and deaths, to store cell positions
        let mut spawns: Vec<Position> = vec![];
        let mut deaths: Vec<Position> = vec![];

        // todo! Consider using .iter().enumerate() to get values

        // for (x, outer_vec) in self.cells.iter().enumerate() {
        //     for (y, inner_vec) in outer_vec.iter().enumerate() {
        //         for (z, cell) in inner_vec.iter().enumerate() {
        //             let index = Position::new(x as i32, y as i32, z as i32);
        //             println!("Cell @ {:#?} has values {:#?}", index, cell);
        //         }
        //     }
        // }

        // Loop through each cell
        for x in 0..=self.bounds - 1 {
            for y in 0..=self.bounds - 1 {
                for z in 0..=self.bounds - 1 {
                    let index = Position::new(x, y, z);
                    let mut cell = self.get_cell(index);

                    // Check cell state (dead/alive)
                    match cell.is_dead() {
                        // Dead cell
                        true => {
                            // Spawn a new cell if it has a valid number of neighbours
                            if rule.birth.is_valid(cell.neighbours) {
                                cell.state = rule.states;
                                spawns.push(index)
                            }
                        },
                        // Alive cell
                        false => {
                            let num_states = rule.states;
                            let valid_survival = rule.survival.is_valid(cell.neighbours);

                            // Kill cell if it has too few states, or does not have enough to survive
                            if cell.state < num_states || !valid_survival {
                                if cell.state == num_states {
                                    deaths.push(index);
                                }
                                // Decrement cell state
                                cell.state -= 1;
                            }
                        },
                    }
                }
            }
        }

        // todo! Discussion: Would this be parallel or linear?
        // Linear would be easier, but would lead to lower performance

        // Update each cell's neighbours
        for position in spawns {
            self.update_neighbours(
                rule,
                position,
                true
            );
        }

        for position in deaths {
            self.update_neighbours(
                rule,
                position,
                false
            );
        }
    }

    fn update_neighbours(&mut self, rule: &Rule, pos: Position, inc: bool) {
        for n in rule.neighbourhood.get_neighbourhood_iter() {
            let neighbour_pos = Position::from_vec(self.wrap(IVec3 { x: pos.x as i32, y: pos.y as i32, z: pos.z as i32 } + *n));

            match inc {
                true => {
                    self.get_cell(neighbour_pos).neighbours += 1;
                },
                false => {
                    self.get_cell(neighbour_pos).neighbours -= 1;
                },
            }
        }
    }

    fn wrap(&self, pos: IVec3) -> IVec3 {
        wrap(pos, self.bounds)
    }

    fn spawn_noise(&mut self, rule: &Rule) {
        default_noise(get_centre(self.bounds), |pos| {
            let position = Position::from_vec(pos);
            let dead = self.get_cell(position).is_dead();

            if dead {
                self.get_cell(position).state = rule.states;
                self.update_neighbours(rule, position, true);
            }
        });
    }
}

impl crate::cells::Sim for MultiThreaded {
    fn update(&mut self, rule: &Rule, _task_pool: &TaskPool) {
        self.update(rule);
    }

    fn render(&self, renderer: &mut CellRenderer) {
        // Convert 3D vector into 1D vector
        for (index, cell) in
        self.cells
            // Flatten vec
            .concat()
            // Flatten vector again
            .concat()
            // Iterate through each cell
            .iter()
            // Add a counter to determine index
            .enumerate() {
            renderer.set(index, cell.state, cell.neighbours);
        }
    }

    fn spawn_noise(&mut self, rule: &Rule) {
        self.spawn_noise(rule);
    }

    fn count(&self) -> usize {
        self.get_count()
    }

    fn get_bounds(&self) -> i32 {
        self.get_bounds()
    }

    fn set_bounds(&mut self, new_bounds: i32) -> i32 {
        self.set_bounds(new_bounds)
    }
}

// TESTS
#[cfg(test)]
mod multi_threading {
    use super::*;

    #[test]
    fn test_count_cells() {
        let cells = vec![
            vec![
                vec![
                    ParallelCell { state: 5, neighbours: 0},
                    ParallelCell { state: 1, neighbours: 0},
                ],
                vec![
                    ParallelCell { state: 0, neighbours: 0},
                    ParallelCell { state: 1, neighbours: 0},
                ],
            ],
            vec![
                vec![
                    ParallelCell { state: 2, neighbours: 0},
                    ParallelCell { state: 1, neighbours: 0},
                ],
                vec![
                    ParallelCell { state: 0, neighbours: 0},
                    ParallelCell { state: 1, neighbours: 0},
                ],
            ],
        ];

        let bounds: usize = 2;
        let chunk_radius: usize = 1;
        let chunk_count: usize = (bounds * 2 + 1) / (chunk_radius * 2 + 1);

        let multi_threaded = MultiThreaded {
            cells,
            bounds: bounds as i32,
            chunk_radius,
            chunk_count,
        };

        // Validate there are two dead cells in the grid
        assert_eq!(multi_threaded.get_count(), 6);
    }
}