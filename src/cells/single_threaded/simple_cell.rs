use bevy::{
    math::IVec3,
    tasks::TaskPool
};

use crate::{
    utilities,
    render::CellRenderer,
    rule::Rule,
};

#[derive(Clone, Copy)]
struct SimpleCell {
    value: u8,
    neighbours: u8,
}

impl SimpleCell {
    fn new() -> SimpleCell {
        SimpleCell {
            value: 0,
            neighbours: 0,
        }
    }
    // Return `true` if the cell has value 0
    pub fn is_dead(self) -> bool {
        self.value == 0
    }

}

pub struct SingleThreaded {
    cells: Vec<SimpleCell>,
    bounds: i32,
    // group: u8, // todo! Different cell types that compete

}

impl SingleThreaded {
    pub fn new() -> Self {
        SingleThreaded {
            cells: vec![],
            bounds: 0,
        }
    }

    // Set the boundary for cells
    pub fn set_bounds(&mut self, new_bounds: i32) -> i32 {
        // Check if bounds has changed
        if new_bounds != self.bounds {
            // Clear the array
            self.cells.clear();
            // Initialise vector of cells, with length bounds^3
            self.cells.resize(
                (new_bounds.pow(3)) as usize,
                SimpleCell::new(),
            );
            // vec!(vec!(SimpleCell { value: 0, neighbours: 0 }));
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
            if !cell.is_dead() {
                result += 1;
            }
        }
        result
    }

    // Convert index in vector to a position (x, y, z) in a 3D cube
    fn idx_to_pos(&self, index: usize) -> IVec3 {
        utilities::idx_to_pos(index as i32, self.bounds)
    }

    // Convert position (x, y, z) in a 3D cube to an index in a 2D vector
    fn pos_to_idx(&self, position: IVec3) -> usize {
        utilities::pos_to_idx(position, self.bounds)
    }

    fn wrap(&self, pos: IVec3) -> IVec3 {
        utilities::wrap(pos, self.bounds)
    }

    fn update_neighbours(&mut self, rule: &Rule, index: usize, inc: bool) {
        let pos = self.idx_to_pos(index);
        for dir in rule.neighbourhood.get_neighbourhood_iter() {
            let neighbour_position = self.wrap(pos + *dir);
            let index = self.pos_to_idx(neighbour_position);
            if inc {
                self.cells[index].neighbours += 1;
            }
            else {
                self.cells[index].neighbours -= 1;
            }
        }
    }

    pub fn update(&mut self, rule: &Rule) {
        let mut spawns = vec![];
        let mut deaths = vec![];

        for (index, cell) in self.cells.iter_mut().enumerate() {
            if cell.is_dead() {
                if rule.birth.in_range(cell.neighbours) {
                    cell.value = rule.states;
                    spawns.push(index);
                }
            } else {
                if cell.value < rule.states || !rule.survival_rule.in_range(cell.neighbours) {
                    if cell.value == rule.states {
                        deaths.push(index);
                    }
                    cell.value -= 1;
                }
            }
        }

        // Update neighbouring cells
        for index in spawns {
            self.update_neighbours(rule, index, true);
        }
        // Update dead cells
        for index in deaths {
            self.update_neighbours(rule, index, false);
        }
    }

    // #[allow(dead_code)]
    // pub fn validate(&self, rule: &Rule) {
    //     for index in 0..self.cells.len() {
    //         let pos = self.idx_to_pos(index);
    //
    //         let mut neighbours = 0;
    //         for dir in rule.neighbourhood.get_neighbourhood_iter() {
    //             let neighbour_pos = self.wrap(pos + *dir);
    //
    //             let index = self.pos_to_idx(neighbour_pos);
    //             if self.cells[index].value == rule.states {
    //                 neighbours += 1;
    //             }
    //         }
    //         assert_eq!(neighbours, self.cells[index].neighbours);
    //     }
    // }

    pub fn spawn_noise(&mut self, rule: &Rule) {
        utilities::default_noise(utilities::get_centre(self.bounds), |pos| {
            let index = self.pos_to_idx(self.wrap(pos));
            if self.cells[index].is_dead() {
                self.cells[index].value = rule.states;
                self.update_neighbours(rule, index, true);
            }
        });
    }
}

impl crate::cells::Sim for SingleThreaded {
    fn update(&mut self, rule: &Rule, _task_pool: &TaskPool) {
        self.update(rule);
    }

    fn render(&self, renderer: &mut CellRenderer) {
        for (index, cell) in self.cells.iter().enumerate() {
            renderer.set(index, cell.value, cell.neighbours);
        }
    }

    fn spawn_noise(&mut self, rule: &Rule) {
        self.spawn_noise(rule);
    }

    fn get_count(&self) -> usize {
        self.count_cells()
    }

    fn get_bounds(&self) -> i32 {
        self.bounds
    }

    fn set_bounds(&mut self, new_bounds: i32) -> i32 {
        self.set_bounds(new_bounds)
    }

}