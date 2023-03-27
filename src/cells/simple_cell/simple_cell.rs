use crate::{cell_render::CellRenderer, helper, rule::Rule};

use crate::cells::Sim;
use bevy::{math::IVec3, tasks::TaskPool};
// use std::fmt;
// use std::fmt::{Display, Formatter};

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

pub struct SingleThreaded {
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

    // pub fn print_cell(&self, index: usize) {
    //     if self.cells[index - 1].dead() {
    //         print!("DEAD ");
    //     }
    //     println!(
    //         "Cell #{} \n--------\nValue: {}",
    //         index,
    //         self.cells[index - 1]
    //     );
    // }

    // Set the boundary for cells
    pub fn set_bounds(&mut self, new_bounds: i32) -> i32 {
        // Check if bounds has changed
        if new_bounds != self.bounds {
            // Clear the array
            self.cells.clear();
            // Initialise vector of cells, with length bounds^3
            self.cells.resize(
                (new_bounds.pow(3)) as usize,
                SimpleCell {
                    value: 0,
                    neighbours: 0,
                },
            );
            // vec!(vec!(SimpleCell { value: 0, neighbours: 0 })));
            self.bounds = new_bounds;
        }
        self.bounds
    }

    // Count the number of live cells
    pub fn count_cells(&self) -> usize {
        let mut result: usize = 0;
        // Loop through each
        for cell in &self.cells {
            // Increment if the cell is not dead
            if !cell.dead() {
                result += 1;
            }
        }
        result
    }

    fn idx_to_pos(&self, index: usize) -> IVec3 {
        helper::idx_to_pos(index, self.bounds)
    }

    fn pos_to_idx(&self, position: IVec3) -> usize {
        helper::pos_to_idx(position, self.bounds)
    }

    fn wrap(&self, pos: IVec3) -> IVec3 {
        helper::wrap(pos, self.bounds)
    }

    fn update_neighbours(&mut self, rule: &Rule, index: usize, inc: bool) {
        let pos = self.idx_to_pos(index);
        for dir in rule.neighbourhood.get_neighbourhood_iter() {
            let neighbour_position = self.wrap(pos + *dir);
            let index = self.pos_to_idx(neighbour_position);

            if inc {
                self.cells[index].neighbours += 1;
            } else {
                self.cells[index].neighbours -= 1;
            }
        }
    }

    pub fn update(&mut self, rule: &Rule) {
        let mut spawns = vec![];
        let mut deaths = vec![];

        for (index, cell) in self.cells.iter_mut().enumerate() {
            if cell.dead() {
                if rule.birth.in_range_incorrect(cell.neighbours) {
                    cell.value = rule.states;
                    spawns.push(index);
                }
            } else {
                if cell.value < rule.states || !rule.survival_rule.in_range_incorrect(cell.neighbours) {
                    if cell.value == rule.states {
                        deaths.push(index);
                    }
                    cell.value -= 1;
                }
            }
        }

        // update neighbors.
        for index in spawns {
            self.update_neighbours(rule, index, true);
        }
        for index in deaths {
            self.update_neighbours(rule, index, false);
        }
    }

    #[allow(dead_code)]
    pub fn validate(&self, rule: &Rule) {
        for index in 0..self.cells.len() {
            let pos = self.idx_to_pos(index);

            let mut neighbours = 0;
            for dir in rule.neighbourhood.get_neighbourhood_iter() {
                let neighbour_pos = self.wrap(pos + *dir);

                let index = self.pos_to_idx(neighbour_pos);
                if self.cells[index].value == rule.states {
                    neighbours += 1;
                }
            }

            assert_eq!(neighbours, self.cells[index].neighbours);
        }
    }

    pub fn spawn_noise(&mut self, rule: &Rule) {
        helper::generate_noise_default(helper::centre(self.bounds), |pos| {
            let index = self.pos_to_idx(self.wrap(pos));
            if self.cells[index].dead() {
                self.cells[index].value = rule.states;
                self.update_neighbours(rule, index, true);
            }
        });
    }
}

impl Sim for SingleThreaded {
    fn update(&mut self, rule: &Rule, _task_pool: &TaskPool) {
        self.update(&rule);
    }

    fn render(&self, renderer: &mut CellRenderer) {
        for (index, cell) in self.cells.iter().enumerate() {
            renderer.set(index, cell.value, cell.neighbours);
        }
    }

    fn spawn_noise(&mut self, rule: &Rule) {
        self.spawn_noise(rule);
    }

    fn cell_count(&self) -> usize {
        self.count_cells()
    }

    fn bounds(&self) -> i32 {
        self.bounds
    }

    fn set_bounds(&mut self, new_bounds: i32) -> i32 {
        self.set_bounds(new_bounds)
    }
}

// impl Display for SimpleCell {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         write!(f, "{}", self.value)
//     }
// }

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
